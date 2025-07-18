# Unit 6.1 - Foreign Function Interface

<a href="/slides/6_1/" target="_blank">Slides</a>


## Exercise 6.1.1: Linked List

Follow the instructions in the comments of `exercises/6-rust-for-systems-programming/1-foreign-function-interface/1-linked-list/src/bin/unsafe.rs`!
## Exercise 6.1.2: execve

Follow the instructions in `exercises/F/2-execve/README.md` and implement in `exercises/F/2-execve/src/main.rs`!
## Exercise 6.1.3: Tagges union

Follow the instructions in the comments of `exercises/6-rust-for-systems-programming/1-foreign-function-interface/3-tagges-union/src/main.rs`!
## Exercise 6.1.4: CRC in C

Use a CRC checksum function written in C in a Rust program

## prerequisites

- A C compiler


## Steps

1. Add the `cc` build dependency, by adding to `Crate.toml` the lines:
    ```toml
    [build-dependencies]
    cc = "1.0"
    ```
2. Create `build.rs` with contents
    ```rust
    extern crate cc;

    fn main() {
        println!("cargo:rerun-if-changed=crc32.h");
        println!("cargo:rerun-if-changed=crc32.c");
        cc::Build::new().file("crc32.c").compile("crc32.a");
    }
    ```

    This will find your c code, compile it, and link it into the executable rust produces
3. In `main.rs`, define an extern (fill in the argument and return types)
    ```rust
    extern "C" {
        fn CRC32( ... ) -> ...; // hint: https://doc.rust-lang.org/std/os/raw
    }
    ```
4. Now, create a rust wrapper that calls the extern function
    ```rust
    fn crc32( ... ) -> ... { 
        ... // (hints: `unsafe`, `.as_ptr()`, `.len()`)
    }
    ```

5. Call our wrapper on some example input
    ```rust
    fn main() {
        println!("{:#x}", crc32(b"12345678"));
    }
    ```
    In the above example, the correct output is `0x9ae0daaf`
## Exercise 6.1.5: CRC in Rust

Use a CRC checksum function written in Rust in a C program

## Requirements

- A C compiler

## Steps

1. Change Cargo.toml to

    ```toml
    [package]
    name = "crc-in-rust"
    version = "0.1.0"
    edition = "2021"

    [lib]
    name = "crc_in_rust"
    crate-type = ["dylib"]

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    ```

2. Expose an extern rust function 

    ```rust
    #[no_mangle]
    pub extern "C" fn crc32(...) -> ... {

        ...

        crc32_rust(...)
    }
    ```

3. Create a C header file `crc_in_rust.h`

    ```c
    #include <inttypes.h> // uint32_t, uint8_t
    #include <stddef.h> // size_t

    uint32_t crc32(const uint8_t data[], size_t data_length);
    ```

4. Use the rust `crc32` function in C

    ```c
    #include <inttypes.h> // uint32_t, uint8_t
    #include <stddef.h> // size_t
    #include <stdio.h> // printf
    #include "crc_in_rust.h"

    int main() { 
        uint8_t data[] = { 0,1,2,3,4,5,6 };
        size_t data_length = 7;

        uint32_t hash = crc32(data, data_length);

        printf("Hash: 0x%d\n", hash);

        return 0;
    }
    ```

5. compile and run

    ```sh
    $ clang main.c target/debug/libcrc_in_rust.so -omain
    $ ./main
    Hash: -1386739207
    ```
## Exercise 6.1.6: QOI Bindgen

In this exercise, we will use `cargo bindgen` to generate the FFI bindings for a C library. Bindgen will look at a C header file, and generate Rust functions, types and constants based on the C definitions.

However, the generated code will likely be ugly and non-idiomatic. To wrap a C library properly, good API design and documentation is needed. 

### Background
The [image crate](https://crates.io/crates/image) provides functionality for encoding, decoding and editing images in Rust. It supports many image formats, like JPEG, PNG and GIF, but also QOI. QOI is a "Quite OK Image format", which aims for fast encoding and decoding of images, while providing a file size similar to PNGs.
In this exercise, we test if the image crate produces the same results when decoding QOI images as the [QOI reference C library](https://github.com/phoboslab/qoi). 

The QOI C library is a header-only library, which means the function implementations are included within the header file instead of in a separate C file. We've added a separate C file which includes the header to make it easier to compile and include the library in our Rust program.

### 6.1.6 Generating bindings
Prerequisites:

- A C compiler is installed on the system
- Bindgen, which can be installed with `cargo install bindgen-cli`

Steps:

1. Create the Rust bindings: `bindgen qoi.h -o src/bindings.rs`
2. Use a `build.rs` script to compile and link `qoi.h`. Create `build.rs` and insert
    ```rust
    fn main() {
        cc::Build::new().file("qoi.c").compile("qoi"); // outputs `qoi.a`
    }
    ```

    And add this section to your `Cargo.toml`

    ```toml
    [build-dependencies]
    cc = "1"
    ```
3. Create `src/lib.rs` with the contents `pub mod bindings;`. This will make the `bindings` module available in `main.rs`.
4. Run `cargo check` to verify everything is compiling correctly.

### 6.1.6 Inspecting our bindings

In the generated `bindings.rs` file we find this signature for the `qoi_read` C function from QOI:

```rust
extern "C" {
    pub fn qoi_read(
        filename: *const ::std::os::raw::c_char,
        desc: *mut qoi_desc,
        channels: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
```

Some observations:

- The definition is inside an `extern "C"` block, and has no body. Therefore, this function is marked as an extern, and Rust expects it to be linked in.
- The function is marked `pub`, meaning we can import and use it in other modules (like `main.rs` in our case)
- We can deduce the behavior somewhat from the type signature:
    * `filename` is a C string with the name of the QOI file we want to read
    * `desc` describes some metadata about the image, the function will write to this `qoi_desc` struct. This struct was also generated by bindgen:
        ```rust
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub struct qoi_desc {
            pub width: ::std::os::raw::c_uint,
            pub height: ::std::os::raw::c_uint,
            pub channels: ::std::os::raw::c_uchar,
            pub colorspace: ::std::os::raw::c_uchar,
        }
        ```
    * `channels` is the number of channels the image has: either 3 for RGB images, or 4 for RGBA images (which also have an alpha channel for transparency). For this exercise, we will assume the images have an alpha channel.
    * The return value is a void pointer. If the function has successfully read the pixel data from a QOI image, then this pointer should point towards the pixel data.
- As the types are raw C types, it can be a hassle to call it directly from Rust.

We will deal with the last point by writing a nice Rust wrapper *around* the generated bindings.

### 6.1.6 Writing our wrapper
To make the `qoi_read` function easier to use, we would like to write a wrapper that takes a path and returns an image buffer:

```rust
fn read_qoi_image(filename: &Path) -> ImageBuffer<Rgba<u8>, &[u8]> {
    todo!()
}
```

To implement this wrapper, there are a couple of challenges that need to be solved:
- We need to turn the path into a C string. Hint: we can use `std::ffi::CString::new` to create a C string from a sequence of bytes, and the most convenient way to turn the path into bytes is to first get the `OsStr` from it. We can then pass the C string as a pointer.
- We need to provide a `qoi_desc`, this struct can be imported from the bindings. Pass a mutable reference to an instance of this struct to the function.
- After calling `qoi_read`, we need to turn the returned void pointer into an image buffer.
    - First, we should check if the returned void pointer `is_null()`. If it is null, something has gone wrong with reading the image.
    - Next, we need to determine the length of the returned pixel data. Assuming the image has an alpha channel, we have 4 bytes for every pixel in the image. The number of pixels in the image can be determined from the `qoi_desc` struct.
    - Now we can turn our void pointer into a `&[u8]`. We can cast our void pointer `as *const u8` first. Next, we use `std::slice::from_raw_parts` with the previously calculated length.
    - Finally, we can use `ImageBuffer::from_raw` to construct our image buffer.

To try out our wrapper, we can try to read a QOI image and export it as a PNG:
```rust
fn main() {
    let image = read_qoi_image(Path::new("image.qoi"));
    image.save("image.png").unwrap();
}
```
If implemented correctly, this should produce a nice picture!

Now that we can decode images using the QOI reference C library, we can test if the image crate produces the same results with the following unit test:
```rust
#[cfg(test)]
mod tests {
    use crate::read_qoi_image;
    use std::path::Path;

    #[test]
    fn test_qoi_read() {
        let filename = "image.qoi";
        let image = image::open(filename).unwrap().into_rgba8();
        let my_image = read_qoi_image(Path::new(filename));

        assert_eq!(image.width(), my_image.width());
        assert_eq!(image.height(), my_image.height());

        assert!(image.pixels().eq(my_image.pixels()));
    }
}
```
If you add this test to `main.rs` and run it with `cargo test` we should see:
```
running 1 test
test tests::test_qoi_read ... ok
```

### 6.1.6 Freeing the pixel data
When working with data from C, we are responsible for deallocating the memory once we are done using it. Some C libraries might provide a separate function to clean up data structures. For QOI, we instead have to call `libc::free` to free the memory, as indicated by the documentation of the `qoi_read` function:
> The returned pixel data should be free()d after use.

To make sure someone using our wrapper does not forget to free the memory, we can implement the `Drop` trait to automatically call `libc::free` when the variable goes out of scope.
- First, create a wrapper `struct QoiSlice { ptr: NonNull<u8>, desc: qoi_desc }`, which holds the image buffer.
- Next, implement the `Drop` trait for `QoiSlice` to free the memory:
    ```rust
    impl Drop for QoiSlice {
        fn drop(&mut self) {
            todo!(); // call libc::free here using a pointer to the image buffer
        }
    }
    ```
- To make this `QoiSlice` usable in an `ImageBuffer`, we have to implement the `Deref` trait:
    ```rust
    impl Deref for QoiSlice {
        type Target = [u8];
    
        fn deref(&self) -> &Self::Target {
            todo!() // create a slice from the ptr and lenght using `slice::from_raw_parts()`
        }
    }
    ```
- Now update the `read_qoi_image` function to return an instance of `ImageBuffer<Rgba<u8>, QoiSlice>`.

### 6.1.6 Uninitialized memory
There is one more trick: our current function initializes the `qoi_desc` struct with zeros (or whatever values you put there while creating an instance of the struct). This is wasteful because the extern function will overwrite these values. Because the extern function is linked in, the compiler likely does not have enough information to optimize this.

For a relatively small struct such as `qoi_desc`, this is not much of a problem. However, for larger structures or big arrays, this can make a serious impact on performance.

If we look at the LLVM IR, the intermediate representation which is generated and optimized before it gets turned into assembly code, we can see that it did not optimize away the initialization of the struct with values. Here we see it uses `memset` to initialize the `desc` with zeros before calling `qoi_read`:

```llvm
call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(10) %desc.i, i8 0, i64 10, i1 false), !noalias !142
%pointer.i = call noundef ptr @qoi_read(ptr noundef nonnull %t.0.i.i, ptr noundef nonnull %desc.i, i32 noundef 4) #17, !noalias !142
```

(The LLVM IR can be generated using `cargo rustc --bin qoi-bindgen --release -- --emit=llvm-ir=llvm-ir.ll`, then search for `@qoi_read` in `llvm-ir.ll`)

The solution is to use `std::mem::MaybeUninit`:

```rust
let mut desc = MaybeUninit::uninit();
let pointer = unsafe { qoi_read(filename.as_ptr(), desc.as_mut_ptr(), 4) };
let desc = unsafe { desc.assume_init() };
```

The `MaybeUninit` type is an abstraction for uninitialized memory. The `.uninit()` method gives a chunk of uninitialized memory big enough to store a value of the desired type (in our case `qoi_desc` will be inferred).

### 6.1.6 Safety documentation

At the moment, the safety of your program relies on the context you have for the wrapped C library.
To ensure somebody modifying your library later does not break any of your assumptions you should always document what you assumed when writing the `unsafe code`.

Add the following [clippy lint](https://rust-lang.github.io/rust-clippy/stable/index.html#multiple_unsafe_ops_per_block) to the top of your `lib.rs` and `main.rs`, run `cargo clippy` and document your assumptions:
```rust
#![deny(clippy::undocumented_unsafe_blocks)]
```

### 6.1.6 Bonus: Idiomatic interface

The current project is quite bare, you could improve that.

#### Structure

Currently, all your logic lives in `main.rs` where it cannot be used as a library.
Move your types and functions over to `lib.rs` and only expose the necessary functionality to the user.
Run `cargo doc --open` to inspect what a user would see.

#### Error handling

Currently, we use panicking to handle errors.
This is problematic since it does not offer the user to handle those errors.
Change `read_qoi_image()` to return a `Result` instead.
And change possible error cases to return an `Err()` instead of panicking.
Also consider which cases cannot possibly happen because of the guarantees you can make, and use `expect()` to panic in that case.

### Conclusion
In this exercise we saw how we can generate bindings to a C library with bindgen. The generated bindings are a bit difficult to work with, as they are unsafe and rely on C types. We've discussed how we can create nice wrappers around the generated bindings to deal with all these C types and to make them safer to work with.
## Exercise 6.1.7: TweetNaCl Bindgen

Use `cargo bindgen` to generate the FFI bindings. Bindgen will look at a C header file, and generate rust functions, types and constants based on the C definitions.

But the generated code is ugly and non-idiomatic. To wrap a C library properly, good API design and documentation is needed. 

### tweetnacl-bindgen

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
