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
# References `&T` and `&mut T` guarantee that

- their address is not `NULL`
- their address is well-aligned for type `T`
- they point into memory belonging to the process

These guarantees make rust memory safe

---
layout: default
---
# The borrow checker

The borrow + type checker ensures that these conditions are met, but the borrow checker is conservative...

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

- interacting with the hardware/OS
- interacting with other languages (FFI)
- optimization

---
layout: default
---
# Unsafe

- unsafe blocks

```rust
// BAD!
let reference: &u8 = unsafe {
    let ptr = 0usize as *const u8;

    &*ptr
};
```
- unsafe functions

```rust
unsafe function foobar() {
    ...
}
```

---
layout: default
---
# Undefined Behavior

```rust
// BAD!
let reference: &u8 = unsafe {
    let ptr = 0usize as *const u8;

    &*ptr
};
```

- compilers make assumptions
- when broken, the behavior of the program is undefined
- safe rust has no UB
---
layout: default
---
# So Rust is just as bad as C?

- if memory safety can be broken, how is rust any better than C?

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

In general the memory representation of rust values is explicitly undefined. Bitcasting is therefore very unsafe!

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

# Example: recreate `RocResult`



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
