Write a custom python extension using PyO3.

Python is a convenient and popular language, but it is not fast. By writing complex logic in faster languages, you can get the best of both worlds. PyO3 makes it extremely easy to write and distribute python extensions written in Rust.

# PyO3 and SIMD

PyO3 makes it easy to write python extensions in rust. The code for this exercise is a skeleton, taken from the PyO3 documentation.

you should be able to run this example like so from the repository root: 

```sh
folkertdev@folkertdev ~/t/teach-rs (mod-g)> cargo build -p pyo3-simd
   Compiling pyo3-simd v0.1.0 (/home/folkertdev/tg/teach-rs/exercises/G/4-pyo3)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
folkertdev@folkertdev ~/t/teach-rs (mod-g)> cp target/debug/libpointwise_simd.so pointwise_simd.so
folkertdev@folkertdev ~/t/teach-rs (mod-g)> python3
Python 3.8.5 (default, May 27 2021, 13:30:53) 
[GCC 9.3.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import pointwise_simd
>>> dir(pointwise_simd)
['__all__', '__doc__', '__file__', '__loader__', '__name__', '__package__', '__spec__', 'sum_as_string']
>>> pointwise_simd.sum_as_string(4,5)
'9'
>>> 
```

Our goal is to implement pointwise addition of two python lists of floats in rust using SIMD instructions.

- hook up the `pointwise_sum` pyfunction, it should call `pointwise_sum_simd`. It is easiest to use `Vec<f64>` in the interface. 
- next run `cargo test -p pyo3-simd`. This should compile, but the test fails. Use the given simd functions and pointer offsets to implement the `pointwise_sum_simd` correctly.
- verify that this works from python.

If that succeeded: congrats, you can now write arbitrary python extensions, and speed up python code. Rust and PyO3 make this really straightforward.
