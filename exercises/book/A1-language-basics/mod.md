# Module A1 - Language basics

[Slides](/slides/A1/) (or [pdf](/slides/A1-intro-to-rust.pdf))

## A1.1 Basic syntax
Open `exercises/A1/1-basic-syntax` in your editor. This folder contains a number of exercises with which you can practise basic Rust syntax.

While inside the `exercises/A1/1-basic-syntax` folder, to get started, run:
```bash
cargo run --bin 01
```

This will try to compile exercise 1. Try and get the example to run, and continue on with the next exercise by replacing the number of the exercise in the cargo run command.

Some exercises contain unit tests. To run the test in `src/bin/01.rs`, run
```bash
cargo test --bin 01
```
Make sure all tests pass!

## A1.2 Move semantics
*This exercise is adapted from the [move semantics exercise](https://github.com/rust-lang/rustlings/tree/main/exercises/move_semantics) from Rustlings*


This exercise enables you to practise with move semantics. It works similarly to exercise `A1.1`. To get started, `exercises/A1/2-move-semantics` in your editor and run
```bash
cargo run --bin 01
```

`01.rs` should compile as is, but you'll have to make sure the others compile as well. For some exercises, instructions are included as doc comments at the top of the file. Make sure to adhere to them.
