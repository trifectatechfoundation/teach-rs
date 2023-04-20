---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - X: Y"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: 'Rust - X: Y'
---
# Rust programming
Module G: FFI

---
layout: default
---
# Learning objectives
<!-- List this module's learning objectives -->

- Working with C from Rust and vice versa
- be familiar with the C representation
- be familiar with the C calling convention
- Work with `cargo bindgen`
- Make nice rust APIs around C libraries
- Create python extensions

---
layout: default
---
# Content overview

- Calling convention and ABI
- using C from Rust
- using Rust from C
- `cargo bindgen`
- PyO3

<!-- Give an overview of the subjects covered in this lecture -->
<!-- Incorporate any breaks as well -->

---
layout: default
---
# Why do languages talk with each other?

- You get an ecosystem for free
- The other language has capabilities (performance, hardware access) that you don't

---
layout: default
---
# Tight langugage coupling

Many languages can use code written in other languages

- JVM: Java, Scala, and Kotlin
- BEAM VM: Erlang and Elixir
- Bare Metal: Zig, D and Nim can import C code

The compiler checks names and types.

---
layout: default
---
# Rust cannot "just" import C code

- Idiomatic C is not idiomatic Rust
- C code cannot provide the guarantees that Rust expects
- maintaining half of a C compiler is not fun

Hence, a much looser coupling:

- generate assembly that is similar to what C generates
- have the linker stitch everything together

---
layout: default
---
<img src="https://faultlore.com/blah/c-isnt-a-language/abi-kiss.png" class="ml-50 h-120 rounded shadow" />

---
layout: default
---
# Rust & C disagree

- different calling conventions
- different memory layout

---
layout: default
---
# Idea: forward-declare the signature

In rust, this function can now be used like any other

```rust
extern "C" {
    fn my_c_function(x: i32) -> bool;
}
```

The linker will stitch this declaration together with the definition

---
layout: default
---
# How to call a function

```rust
extern "C" {
    fn my_c_function(x: i32) -> bool;
}

pub fn main () {
    unsafe { my_c_function(42) };
}
```

generates this code for `main`:

```asm
example::main:
 push   rax                             # free up rax
 mov    edi,0x2a                        # put the argument into the edi register
 call   80b0 <example::my_c_function>   # call `my_c_function`
 pop    rax                             # restore rax
 ret                                    # return
```

---
layout: default
---
# Space vs Speed

We can compile this code in two ways

```rust
fn foo(vec: Vec<u8>) -> usize { vec.len() }

fn main() { foo(vec![]); }
```

Using 3 registers:

```rust
fn foo(ptr: *const u8, len: usize, cap: usize) -> usize {
    len
}
```

or using one register and indirection:

```rust
fn foo(vec: *const (usize, usize, usize)) -> usize {
    vec.1
}
```

---
layout: default
---
# Calling convention

- Rust and C make different choices on by-value vs. by-reference
- `extern "C"` forces rust to use the C calling convention
- The C calling convention is the lingua franca of calling between languages

---
layout: default
---
# C types != Rust types

- for some types, Rust and C agree on the representation

```rust
extern "C" {
    // integers
    fn is_even(x: i32) -> bool;

    // pointers
    fn is_null(ptr: *const u32) -> bool;
}


#[repr(u8)]
enum Color { R, G, B }

extern "C" {
    // tag-only enums
    fn circle_with_me(c: Color) -> Color;
}
```

---
layout: default
---
# C types != Rust types

- for others, we must explicitly pick the representation

```rust
#[repr(C)]
struct Point { x: f32, y: f32 }

extern "C" {
    // repr(C) structs
    fn h(p: Point) -> bool;
}

#[repr(transparent)]
struct Wrapper<T>(T);

extern "C" {
    // repr(transparent) structs, if the inner type is repr(C)
    fn h(w: Wrapper<u64>) -> bool;
}
```

---
layout: default
---
# C types != Rust types

- for others, we must explicitly pick the representation

```rust
#[repr(C)]
union U { int: i64, float: f64 }

extern "C" {
    // repr(C) unions
    fn i(u: U) -> bool;
}
```

---
layout: default
---
# C types != Rust types

- many types just don't work:
- enums like `Result` or `Option`
- owned collections like `String` and `Vec<T>`
- fat pointers like `&str` or `&[T]`

these need special, manual treatment

---
layout: default
---
# `cargo-bindgen`

Generates rust API bindings based on C header files

```rust
extern "C" {
    pub fn crypto_stream_salsa20_tweet_xor(
        arg1: *mut ::std::os::raw::c_uchar,
        arg2: *const ::std::os::raw::c_uchar,
        arg3: ::std::os::raw::c_ulonglong,
        arg4: *const ::std::os::raw::c_uchar,
        arg5: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify_16_tweet(
        arg1: *const ::std::os::raw::c_uchar,
        arg2: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify_32_tweet(
        arg1: *const ::std::os::raw::c_uchar,
        arg2: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
```

---
layout: default
---
# So far

C and Rust don't just work together, we must

- tell rust the name and type of extern functions
- force rust to use the C calling convention
- use only types that have a C-compatible representation
- `cargo-bindgen` automates parts of this process

---
layout: default
---
# Using Rust from C

exposed functions look like this

```rust
#[no_mangle]
extern "C" fn sum(ptr: *const u64, len: usize) -> u64 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };

    slice.iter().sum()
}
```

Compiling rust into a static library requires some extra setup in the `Cargo.toml`.

---
layout: default
---
# Using Rust from Python

```rust
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

```

```shell
$ python
>>> import string_sum
>>> string_sum.sum_as_string(5, 20)
'25'
```

---
layout: default
---

# Demo

---
layout: default
---

# Optional demo: `roc glue`

---
layout: end
---
<!-- Below are example slides you can use -->

---
layout: playground
---
# Code example

```rust
fn main() {
  println!("Hello world!");
}
```
<!-- Slide for code examples with a link to Rust playground -->

---
layout: iframe
url: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main()%20%7B%0A%20%20println!(%22Hello%20world!%22)%3B%0A%7D
---
<!-- Iframe slide containing Rust playground -->
