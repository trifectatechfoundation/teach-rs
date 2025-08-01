In this exercise, we will use `cargo bindgen` to generate the FFI bindings for a C library. Bindgen will look at a C header file, and generate Rust functions, types and constants based on the C definitions.

However, the generated code will likely be ugly and non-idiomatic. To wrap a C library properly, good API design and documentation is needed. 

### Background
The [image crate](https://crates.io/crates/image) provides functionality for encoding, decoding and editing images in Rust. It supports many image formats, like JPEG, PNG and GIF, but also QOI. QOI is a "Quite OK Image format", which aims for fast encoding and decoding of images, while providing a file size similar to PNGs.
In this exercise, we test if the image crate produces the same results when decoding QOI images as the [QOI reference C library](https://github.com/phoboslab/qoi). 

The QOI C library is a header-only library, which means the function implementations are included within the header file instead of in a separate C file. We've added a separate C file which includes the header to make it easier to compile and include the library in our Rust program.

### #[modmod:exercise_ref] Generating bindings
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

### #[modmod:exercise_ref] Inspecting our bindings

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

### #[modmod:exercise_ref] Writing our wrapper
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

### #[modmod:exercise_ref] Freeing the pixel data
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

### #[modmod:exercise_ref] Uninitialized memory
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

### #[modmod:exercise_ref] Safety documentation

At the moment, the safety of your program relies on the context you have for the wrapped C library.
To ensure somebody modifying your library later does not break any of your assumptions you should always document what you assumed when writing the `unsafe code`.

Add the following [clippy lint](https://rust-lang.github.io/rust-clippy/stable/index.html#multiple_unsafe_ops_per_block) to the top of your `lib.rs` and `main.rs`, run `cargo clippy` and document your assumptions:
```rust
#![deny(clippy::undocumented_unsafe_blocks)]
```

### #[modmod:exercise_ref] Bonus: Idiomatic interface

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
