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
