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


<!-- NOTES:
unsafe limits the number of lines where undefined behavior

get_unsafe out of bounds: wat gebeurt er?

existence is UB?

linked list pointer version: implementeer `len()` en `is_empty()`, IntoIterator
-->

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


- when to reach for `unsafe` code
- reason about undefined behavior
- familiarity with raw pointers in rust
---
layout: section
---
#  Module X
[The name of this module]
<!-- Start lecture content here -->

---
layout: default
---
# Content overview
- FFI types
- Types from std
<!-- Give an overview of the subjects covered in this lecture -->
<!-- Incorporate any breaks as well -->

---
layout: default
---
# Rust guarantees that references are valid

- their address is not `NULL`
- their address is well-aligned for type `T`
- they point into memory belonging to the process

These guarantees make rust memory safe

---
layout: default
---
# The borrow checker

The borrow + type checker ensures that these conditions are met

- We want a 100% guarantee that when the compiler says 👍
that really means our program is correct


- An analysis that is wrong in 1 out of 100 cases is worthless

---
layout: default
---
# The borrow checker is conservative

- "if it is not a hell yes, it's a no"

---
layout: default
---
# fail-proof borrow checker

```rust
fn borrow_checker<P>(program: P) -> bool {
    false
}
```

all programs that my borrow checker accepts are memory safe!

---
layout: default
---
# Hence

There are (many) correct programs that the rust borrow checker does not accept

---
layout: default
---
# Hence

There are (many) **useful** programs that the rust borrow checker does not accept

- interacting with other languages (FFI)
- interacting with the OS/hardware
- optimization

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

### Question: what is the memory layout of LinkedList. What is the size?

---
layout: default
---

# Exercises

- implement `Drop` for the pointer-based `LinkedList`
- Implement a process forwarding program using `std::process::Command`
- Implement a process forwarding program using `execve`


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
