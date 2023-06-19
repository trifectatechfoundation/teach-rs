# Module G - Foreign Function Interface 

[Slides](/slides/G/) (or [pdf](/slides/G-ffi.pdf))

## G.1 CRC in C ★★★

Use a CRC checksum function written in C in a Rust program

Follow the instructions `exercises/G/1-crc-in-c/README.md`!

## G.2 CRC in Rust ★★★

Use a CRC checksum function written in Rust in a C program

Follow the instructions `exercises/G/2-crc-in-rust/README.md`!

## G.3 Bindgen ★★★

Use `cargo bindgen` to generate the FFI bindings. Bindgen will look at a C header file, and generate rust functions, types and constants based on the C definitions.

But the generated code is ugly and non-idiomatic. To wrap a C library properly, good API design and documentation is needed. 

Follow the instructions `exercises/G/3-tweetnacl-bindgen/README.md`!

## G.4 PyO3 ★★★

Write a custom python extension using PyO3.

Python is a convenient and popular language, but it is not fast. By writing complex logic in faster languages, you can get the best of both worlds. PyO3 makes it extremely easy to write and distribute python extensions written in Rust.

Follow the instructions `exercises/G/4-pyo3/README.md`!
