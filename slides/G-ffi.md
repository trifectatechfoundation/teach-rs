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
# Unsafe, morally 

In rust "unsafe" means "I, the programmer, am responsible for checking the correctness of this code"

The type and borrow checker are still in fully effect. But we can use types like raw pointers on which the conditions that the type/borrow checker places are less strict.

---
layout: default
---
# Unsafe in code


- unsafe blocks: "programmer must check the rules" 

```rust
// BAD!
let reference: &u8 = unsafe {
    let ptr = 0usize as *const u8;

    &*ptr
};
```

- unsafe functions: "programmer must check the preconditions"

```rust
unsafe function foobar() {
    ...
}
```

---
layout: default
---
# Undefined Behavior & Optimizations

```rust
// std::hint::unreachable_unchecked
pub const unsafe fn unreachable_unchecked() -> !
```

- `unsafe fn`: to call this function, the programmer has to check the preconditions
- returns "never", the type of an infinite loop (diverging computation)

---
layout: default
---
# Undefined Behavior & Optimizations

```rust {all|2}
if expensive_pure_computation() == 0 { 
    println!("hello there");
    unsafe { std::hint::unreachable_unchecked() }
} else {
    different_computation()
}
```

- that print is unreachable if the rest of the branch is unreachable

---
layout: default
---
# Undefined Behavior & Optimizations

```rust {all|2}
if expensive_pure_computation() == 0 { 
    unsafe { std::hint::unreachable_unchecked() }
} else {
    different_computation()
}
```

- actually the whole branch is unreachable 

---
layout: default
---
# Undefined Behavior & Optimizations

```rust
expensive_pure_computation() == 0;
different_computation()
```

- actually that whole condition does not need to be computed 

---
layout: default
---
# Undefined Behavior & Optimizations

```rust {all|2}
if expensive_pure_computation() == 0 { 
    println!("hello there");
    unsafe { std::hint::unreachable_unchecked() }
} else {
    different_computation()
}
```

becomes just 

```rust
different_computation()
```

- but if the condition turns out to be reachable, behavior is confusing


---
layout: default
---
# Undefined Behavior & Optimizations

- misusing `unreachable_unchecked` is very explicit. There are many more subtle ways to introduce UB

```rust
// BAD!
let reference: &u8 = unsafe {
    let ptr = 0usize as *const u8;

    &*ptr
};
```

- the rust compiler assumes that references are valid, so this snippet contains UB!


---
layout: default
---

# `transmute`

e.g. bitcast a `i64` into a `f64`:

```rust
std::mem::transmute::<i64, f64>(42i64)
```

there are still checks! `transmute` errors when the size does not correspond

```
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
 --> src/main.rs:2:12
  |
2 |   unsafe { std::mem::transmute::<i64, f32>(42i64) };
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: source type: `i64` (64 bits)
  = note: target type: `f32` (32 bits)
```

---
layout: default
---

# `transmute`

Only some bit patterns are valid for a type. Creating an invalid bit pattern is UB!

Only `0b0000_0000` and `0b0000_0001` are valid `bool` bit patterns. This is instant UB:

```rust
std::mem::transmute::<u8, bool>(2u8)
```

The memory representation of rust values is explicitly undefined! Bitcasting is therefore very unsafe!

---
layout: default
---
# So Rust is just as bad as C?

- if memory safety can be broken, how is rust any better than C?

---
layout: default
---
# Examples

- interacting with other languages (FFI)
- interacting with the OS/hardware
- optimization

---
layout: default
---
# Using libc functions

```rust
// pub unsafe extern "C" fn getpid() -> pid_t

use libc;

println!("My pid is {}", unsafe { libc::getpid() });
```

---
layout: default
---
# Using libc functions

```rust
// pub fn id() -> u32

use std::process;

println!("My pid is {}", process::id());
```

---
layout: default
---
# Interacting with the OS

```rust
unsafe fn execve(&self, argv: &[*const c_char], envp: &[*const c_char]) -> c_int {
    match self {
        // ...

        #[cfg(target_family = "windows")]
        ExecutableFile::OnDisk(_, path) => {
            let path_cstring = CString::new(path.to_str().unwrap()).unwrap();

            libc::execve(path_cstring.as_ptr().cast(), argv.as_ptr(), envp.as_ptr())
        }
    }
}
```

---
layout: default
---
# Interacting with the Hardware

Using a SIMD intrinsic

```rust
#[target_feature(enable = "avx")]
unsafe fn vperilps(mut current: __m128, mask: (i32, i32, i32, i32)) -> __m128 {
    let mask = _mm_set_epi32(mask.3, mask.2, mask.1, mask.0);

    std::arch::asm!(
        "vpermilps {a:y}, {a:y}, {m:y}",
        a = inout(ymm_reg) current,
        m = in(ymm_reg) mask,

    );

    current
}
```

---
layout: default
---

# Example: Memory consumption of linked lists 

```rust
enum LinkedList {
    Nil,
    Cons(u64, Box<LinkedList>),
}

use LinkedList::*;

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        let mut list = Nil;
        for value in range.rev() {
            list = Cons(value, Box::new(list));
        }

        list
    }

    fn sum(&self) -> u64 {
        match self {
            Nil => 0,
            Cons(first, rest) => first + rest.sum(),
        }
    }
}
```

---
layout: default
---

# Example: Memory consumption of linked lists 

```rust {all|20|all}
enum LinkedList {
    Nil,
    Cons(u64, Box<LinkedList>),
}

// could be represented as

struct LinkedList { 
    tag: LinkedListTag,
    payload: LinkedListUnion,
}

enum LinkedListTag {
    Nil = 0,
    Cons = 1,
}

union LinkedListUnion {
    nil: (),
    cons: (u64, std::mem::ManuallyDrop<Box<LinkedList>>),
}
```

---
layout: default
---

# Example: Memory consumption of linked lists 

- what is the memory layout of this type?

```rust {all}
struct LinkedList { 
    tag: LinkedListTag,
    payload: LinkedListUnion,
}

enum LinkedListTag {
    Nil = 0,
    Cons = 1,
}

union LinkedListUnion {
    nil: (),
    cons: (u64, std::mem::ManuallyDrop<Box<LinkedList>>),
}
```

- field order
- alignment
- size

---
layout: default
---

# Example: Memory consumption of linked lists 

```rust {all|1-6|10|11-14|19-24|all}
struct LinkedList(*const Node);

struct Node {
    first: u64,
    rest: LinkedList,
}

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        let mut list = LinkedList(std::ptr::null());
        for value in range.rev() {
            let node = Node { first: value, rest: list };
            list = LinkedList(Box::into_raw(Box::new(node)));
        }

        list
    }

    fn sum(&self) -> u64 {
        if self.0.is_null() { 0 } else {
            let node = unsafe { std::ptr::read(self.0) };
            node.first + node.rest.sum()
        }
    }
}
```

---
layout: default
---

# Example: Memory consumption of linked lists 

```rust
struct LinkedList(Option<Box<Node>>);

struct Node {
    first: u64,
    rest: LinkedList,
}

impl LinkedList {
    fn range(range: Range<u64>) -> Self {
        todo!()
    }

    fn sum(&self) -> u64 {
        todo!()
    }
}
```


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
