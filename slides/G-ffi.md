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
Module X: description
<!-- Start with welcome, students entering -->
<!-- TODO add subject code -->

---
layout: three-slots
---
## Who am i?
::left::
- Ferris
- I Love Rust

::right::
<img src="https://rustacean.net/assets/rustacean-orig-noshadow.png" alt="Photo Ferris" width="300" />
<!-- Optionally quickly introduce yourself, add photo -->

---
layout: default
---
# Last time...
- Ownership model
- Move semantics
<!-- Recap on content from last time that current subject builds on -->

---
layout: section
---
# Quick questions

Any questions on last time's subject?
<!-- Keep it short. Any longer explanations can be deferred to tutorial -->

---
layout: section
---
# Recap Quiz

## [Link to quiz here]

---
layout: iframe
url: http://your-quiz-url-here
---
<!-- insert URL to quiz roundup in slide option `url` -->

---
layout: default
---
# In this module
<!-- Introduce today's subject -->


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

---
layout: default
---
# Content overview

- Calling convention and ABI
- FFI types
- Ownership
- Useful std features for FFI 
- using C from Rust
- using Rust from C
- `cargo bindgen`
- TweetNaCl example

<!-- Give an overview of the subjects covered in this lecture -->
<!-- Incorporate any breaks as well -->

---
layout: default
---
# Goal: call code written in C (or other languages) 

Many languages can use code written in other languages 

- JVM: Java, Scala, and Kotlin
- BEAM VM: Erlang and Elixir

The compiler checks names and types.

---
layout: default
---
<img src="https://faultlore.com/blah/c-isnt-a-language/abi-kiss.png" class="ml-50 h-120 rounded shadow" />

---
layout: default
---
# Why we cannot import C 

- Languages like Zig, D and Nim can import C code
- C and Rust do not agree on memory layout and calling convention
- Idiomatic C is not idiomatic Rust
- So we fall back on the C ABI and the linker


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

The implementation is provided by the linker

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
- The C ABI is the lingua franca of calling between languages

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
# So far

C and Rust don't just work together, we must

- tell rust the name and type of extern functions
- force rust to use the C calling convention 
- use only types that have a C-compatible representation

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

# Summary
<!-- Very quickly go over the learning objectives and how they were covered -->

---
layout: default
---
# Practicalities
<!-- Use this slide to announce any organizational information -->

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
