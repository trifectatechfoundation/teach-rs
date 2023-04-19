## prerequisites

- A C compiler


## Steps

0. `cargo new crc-in-c` and `cd crc-in-c`
1. Copy `crc.c` and `crc.h`
2. Add the `cc` build dependency, by adding to `Crate.toml` the lines:
    ```toml
    [build-dependencies]
    cc = "1.0"
    ```
3. Create `build.rs` with contents
    ```rust
    extern crate cc;

    fn main() {
        cc::Build::new().file("crc32.c").compile("crc32.a");
    }
    ```

    This will find your c code, compile it, and link it into the executable rust produces
4. Define an extern (fill in the argument and return types)
    ```rust
    extern "C" {
        fn CRC32( ... ) -> ...; // hint: https://doc.rust-lang.org/std/os/raw
    }
    ```
5. Create a rust wrapper that calls the extern function
    ```rust
    fn crc32( ... ) -> ... { 
        ... // (hints: `unsafe`, `.as_ptr()`, `.len()`)
    }
    ```

6. Call our wrapper on some example input
    ```rust
    fn main() {
        println!("{}", crc32(b"12345678"));
    }
    ```
