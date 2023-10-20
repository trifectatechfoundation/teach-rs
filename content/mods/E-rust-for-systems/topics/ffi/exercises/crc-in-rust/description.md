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

