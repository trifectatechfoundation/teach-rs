Use `cargo bindgen` to generate the FFI bindings. Bindgen will look at a C header file, and generate rust functions, types and constants based on the C definitions.

But the generated code is ugly and non-idiomatic. To wrap a C library properly, good API design and documentation is needed. 

# tweetnacl-bindgen

Making rust bindings for the [tweetnacl](https://tweetnacl.cr.yp.to/) C library

## Exercise: implement `crypto_hash_sha256_tweet`

Below you find instructions for using bindgen and wrapping `crypto_hash_sha512_tweet`. Follow the instructions, then repeat the steps for `crypto_hash_sha256_tweet`

## Instructions

Prerequisites:

- a C compiler is installed on the system
- bindgen, install with `cargo install bindgen-cli`

Steps

1. Create the rust bindings: `bindgen tweetnacl.h -o src/bindings.rs`
2. Use `build.rs` to compile and link `tweetnacl.c`. Create `build.rs` and insert
    ```rust
    fn main() {
        cc::Build::new()
            .file("tweetnacl.c")
            .compile("tweetnacl");   // outputs `libtweetnacl.a`
    }
    ```

    And add this section to your `Cargo.toml`

    ```toml
    [build-dependencies]
    cc = "1"
    ```
3. Create `src/lib.rs` with the contents `pub mod bindings;`. This will make the `bindings` module available in `main.rs`.
4. Run `cargo check` to verify everything is compiling correctly.
5. By default building will generate a bunch of warnings. we can turn those off by replacing our build.rs with

    ```rust
    fn main() {
        cc::Build::new()
            .warnings(false)
            .extra_warnings(false)
            .file("tweetnacl.c")
            .compile("tweetnacl"); // outputs `libtweetnacl.a`
    }
    ```

    and adding this line at the top of `src/bindings.rs`:
    ```rust
    #![allow(non_upper_case_globals)]
    ```

## Inspecting our bindings

In the generated `bindings.rs` file we find this signature for the `crypto_hash_sha512_tweet` C function from tweetNaCl:

```rust
extern "C" {
    pub fn crypto_hash_sha512_tweet(
        arg1: *mut ::std::os::raw::c_uchar,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
```

Some observations

- The definition is inside of an `extern "C"` block, and has no body. Therefore this function is marked as an extern, and rust expects it to be linked in.
- The function is marked `pub`, meaning we can import and use it in other modules (like `main.rs` in our case)
- We can deduce the behavior from the type signature:
    * `arg1` is the output: a mutable pointer to a sequence of bytes
    * `arg2` is the input: a constant pointer to a sequence of bytes
    * `arg3` is a length (unclear of what)
    * the return value is probably an error code
- These are raw C types, which makes it a hassle to call directly from rust.

We will deal with the last point by writing some nice rust wrappers *around* the generated bindings.

In rust we bundle a pointer to a sequence of elements and its length in a slice. We could write the signature of our own rust wrapper function as:

```rust
pub fn crypto_hash_sha512_tweet(out: &mut [u8], data: &[u8]) -> i32 {
    todo!()
}
```

## Modelling with types

But by looking at the tweetNaCl source code we can see that the contract is a bit stronger:

- the output is always 64 bytes wide (64 * 8 = 512)
- we only ever return `0`

```c
int crypto_hash(u8 *out,const u8 *m,u64 n)
{
  u8 h[64],x[256];
  u64 i,b = n;

  FOR(i,64) h[i] = iv[i];

  crypto_hashblocks(h,m,n);
  m += n;
  n &= 127;
  m -= n;

  FOR(i,256) x[i] = 0;
  FOR(i,n) x[i] = m[i];
  x[n] = 128;

  n = 256-128*(n<112);
  x[n-9] = b >> 61;
  ts64(x+n-8,b<<3);
  crypto_hashblocks(h,x,n);

  FOR(i,64) out[i] = h[i];

  return 0;
}
```

The rust type system can model these invariants: We can explicitly make the output 64 elements long by using a reference to an array. Furthermore we can drop the return type if there is nothing useful to return.

```rust
pub fn crypto_hash_sha512_tweet(out: &mut [u8; 64], data: &[u8]) {
    todo!()
}
```

But even better, we can return the output array directly:

```rust
pub fn crypto_hash_sha512_tweet(data: &[u8]) -> [u8; 64] {
    todo!()
}
```

The compiler will turn this signature into the one we had before under the hood. Returning the value is more idiomatic and convenient in rust, and with modern compilers there is no performance penalty.

> In detail: The C ABI mandates that any return value larger than those that fit in a register (typically 128 bits nowadays) are allocated on the caller's stack. The first argument to the function is the pointer to write the result into. LLVM, the backend used by the rust compiler has specific optimizations to make sure the function result is written directly into this pointer.

## Writing our implementation

Allright, with the signature worked out, we can write the actual implementation.

We can reach the bindings from `main.rs` with e.g.

```rust
tweetnacl_bindgen::bindings::crypto_hash_sha512_tweet(a,b,c);
```

Here `tweetnacl_bindgen` is the name of the project, specified in the `package` section of the `Cargo.toml`

```toml
[package]
name = "tweetnacl-bindgen"
```

Then `bindings` is the module name (the file `src/bindings.rs` is implicitly also a module) and finally `crypto_hash_sha512_tweet` is the function name from the original C library.

On to the implmentation. Extern functions are considered unsafe in rust, so we will need an unsafe block to call ours.

```rust
pub fn crypto_hash_sha512_tweet(data: &[u8]) -> [u8; 64] {
    unsafe {
        tweetnacl_bindgen::bindings::crypto_hash_sha512_tweet(
            todo!(),
            todo!(),
            todo!(),
        );
    }
}
```

Next we can pass our argument: we turn the slice into a pointer with `.as_ptr()`, and get the length with `len()`. The length needs to be cast to the right type. In this case we can use `as _` where rust will infer the right type to cast to.

```rust
pub fn crypto_hash_sha512_tweet(data: &[u8]) -> [u8; 64] {
    unsafe {
        tweetnacl_bindgen::bindings::crypto_hash_sha512_tweet(
            todo!(),
            data.as_ptr(),
            data.len() as _,
        );
    }
}
```

Next we create an array for the return value, pass a mutable pointer to this memory to our extern functin, and return the array.

```rust
pub fn crypto_hash_sha512_tweet(data: &[u8]) -> [u8; 64] {
    let mut result = [ 0; 64 ];

    unsafe {
        tweetnacl_bindgen::bindings::crypto_hash_sha512_tweet(
            &mut result as *mut _,
            data.as_ptr(),
            data.len() as _,
        );
    }

    result
}
```

And we're done: an idiomatic rust wrapper around the `crypto_hash_sha512_tweet`!

## Uninitialized memory

There is one more trick: our current function initializes and zeroes out the memory for `result`. That is wasteful because the extern function will overwrite these zeroes. Because the extern function is linked in, the compiler likely does not have enough information to optimize the zeroing out away.

The solution is `MaybeUninit`:

```rust
use std::mem::MaybeUninit;

pub fn crypto_hash_sha512_tweet(data: &[u8]) -> [u8; 64] {
    let mut result : MaybeUninit<[u8; 64]> = MaybeUninit::uninit();

    unsafe {
        tweetnacl_bindgen::bindings::crypto_hash_sha512_tweet(
            result.as_mut_ptr() as *mut _,
            data.as_ptr(),
            data.len() as _,
        );

        result.assume_init()
    }
}
```

The `std::mem::MaybeUninit` type is an abstraction for uninitialized memory. The `.uninit()` method gives a chunk of uninitialized memory big enough to store a value of the desired type (in our case `[u8; 64]` will be inferred).

We can look at the LLVM IR to verify that 1) the initialization with zeroes is not optimized away and 2) using MaybeUninit does not initialize the array.

Below is a call site of our `crypto_hash_sha512_tweet` function that zeroes out the memory. Indeed, we see a `memset` that sets all the bytes to 0. (also not that our wrapper function actually got inlined)

```llvm
%result.i = alloca <64 x i8>, align 1
%0 = getelementptr inbounds <64 x i8>, <64 x i8>* %result.i, i64 0, i64 0
call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 1 dereferenceable(64) %0, i8 0, i64 64, i1 false), !alias.scope !8, !noalias !11
%_2.i = call i32 @bindings::crypto_hash_sha512_tweet(i8* nonnull %0, i8* nonnull "foobarbaz", i64 9)
```
In constrast, the version with `MaybeUninit` just calls our extern function without touching the memory at all:

```llvm
%result.i = alloca <64 x i8>, align 1
%0 = getelementptr inbounds <64 x i8>, <64 x i8>* %result.i, i64 0, i64 0

%_3.i = call i32 @bindings::crypto_hash_sha512_tweet(i8* nonnull %0, i8* nonnull "foobarbaz", i64 9), !noalias !6
```
<details>
<summary>Full LLVM IR</summary>
<p>

```llvm
define i8 @call_with_maybeuninit() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %result.i = alloca <64 x i8>, align 1
  %0 = getelementptr inbounds <64 x i8>, <64 x i8>* %result.i, i64 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 64, i8* nonnull %0), !noalias !2
  %_3.i = call i32 @crypto_hash_sha512_tweet(i8* nonnull %0, i8* nonnull getelementptr inbounds (<{ [9 x i8] }>, <{ [9 x i8] }>* @alloc1, i64 0, i32 0, i64 0), i64 9), !noalias !6
  %1 = load <64 x i8>, <64 x i8>* %result.i, align 1, !noalias !7
  call void @llvm.lifetime.end.p0i8(i64 64, i8* nonnull %0), !noalias !2
  %2 = call i8 @llvm.vector.reduce.add.v64i8(<64 x i8> %1)
  ret i8 %2
}

define i8 @call_without_maybeuninit() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %_4 = alloca <64 x i8>, align 1
  %0 = getelementptr inbounds <64 x i8>, <64 x i8>* %_4, i64 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 64, i8* nonnull %0)
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 1 dereferenceable(64) %0, i8 0, i64 64, i1 false), !alias.scope !8, !noalias !11
  %_2.i = call i32 @crypto_hash_sha512_tweet(i8* nonnull %0, i8* nonnull getelementptr inbounds (<{ [9 x i8] }>, <{ [9 x i8] }>* @alloc1, i64 0, i32 0, i64 0), i64 9)
  %1 = load <64 x i8>, <64 x i8>* %_4, align 1
  %2 = call i8 @llvm.vector.reduce.add.v64i8(<64 x i8> %1)
  call void @llvm.lifetime.end.p0i8(i64 64, i8* nonnull %0)
  ret i8 %2
}
```

</p>
</details> 
