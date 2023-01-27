---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - A4: Traits and Generics"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: 'Rust - A4: Traits and Generics'
---
# Rust programming
Module A4: Traits and generics
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

*Any questions?*

<!--
- Recap on content from last time that current subject builds on
- Keep it short. Any longer explanations can be deferred to tutorial
-->

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
Make your code more versatile
<!-- Introduce today's subject -->

---
layout: default
---
# Learning objectives
- Use traits and generics
- Use common traits from `std`
- Static versus dynamic dispatch
<!-- List this module's learning objectives -->

---
layout: section
---
# Mindmap

What do you know already about this subject?

## [Mindmap access code here]
<!-- Quick mindmap, show mindmap access code -->

---
layout: iframe
url: http://your-interactive-mindmap-url-here
---
<!-- insert URL to live mindmap in slide option `url` -->

---
layout: cover
---
#  Module A4
Traits and generics
<!-- Start lecture content here -->

---
layout: default
---
# Content overview
TODO
<!-- Give an overview of the subjects covered in this lecture -->
<!-- Incorporate any breaks as well -->

---
layout: section
---
# Introduction to traits

---
layout: default
---
# The problem

```rust
fn add_u32(l: u32, r: u32) -> u32 { /* -snip- */ }

fn add_i32(l: i32, r: i32) -> i32 { /* -snip- */ }

fn add_f32(l: f32, r: f32) -> f32 { /* -snip- */ }

/* ... */
```

<v-click>
<div>
<strong>We need generic code!</strong>
</div>
</v-click>

<!--
Let's have a look at this Rust module. We'd like to provide functionality for finding the maximum of two numbers, for several distict types. One way to go about it, is to define many similar functions that perform the operation. But there's a number of problems with that:
- What happens if we want to compare other types?
- What happens if we want to compare separate types?
-->

---
layout: default
---
# Generic code
```rust
fn add<T>(lhs: T, rhs: T) -> T { /* - snip - */}

fn log<T>(item: T) { /* - snip - */}
```

<v-click>
<div>
<br/>
Or, in plain English:

- `<T>` = "let `T` be a type"
- `lhs: T` "let `lhs` be of type `T`"
- `-> T` "let `T` be the return type of this function"
</div>
</v-click>
<v-click>
<div>
Some open points:
- What can we do with a `T`?
- What should body be?
</div>
</v-click>

---
layout: default
---
# Bounds on generic code
We need to provide information to compiler:
- Tell Rust what `T` can do
- Tell Rust what `T` is accepted
- Tell Rust how `T` implements functionality

More bounds = more functionality inside = less flexibility outside

---
layout: default
---

# `trait`
```rust
trait MyAdd {
    fn my_add(&self, other: &Self) -> Self;
}
```

- Describe what the type can do
- Describe how the type does it


---
layout: default
---
# `impl trait`
```rust{all|1|2-8}
impl MyAdd for u32 {
    fn my_add(&self, other: &Self) -> Self {
      *self + *other
    }
}
```

---
layout: default
---
# Using a `trait`

```rust{all|1-2|5-6}
// Import the type and the trait
use my_mod::{MyAdd}

fn main() {
  let left: u32 = 6;
  let right: u32 = 8;
  // Call trait method
  let result = left.my_add(&right);
  assert_eq!(result, 14)
}
```

- Trait needs to be in scope
- Call just like a method

---
layout: default
---
# Trait bounds

```rust{all|1-3|7-10}
fn add_values<T: MyAdd>(this: &T, other: &T) -> T {
  this.my_add(other)
}

// Or

fn add_values<T>(this: &T, other: &T) -> T 
  where T: MyAdd {
  this.my_add(other)
}
```

Now we've got a useful generic function!

In English:
"For all types `T` that implement the `MyAdd` `trait`, we define..."

---
layout: default
---

# Calling `trait` methods

```rust
impl MyAdd for i32 {/* - snip - */}
impl MyAdd for f32 {/* - snip - */}

fn add_values<T>(this: &T, other: &T) -> T 
  where T: MyAdd {
  this.my_add(other)
}

fn main() {
  let sum_one = add_values(&6, &8);
  assert_eq!(sum_one, 14);
  let sum_two = add_values(&6.5, &7.5);
  println!("Sum two: {}", sum_two); // 14
}
```

- Code is <em>monomorphized</em>
- Two versions of `add_values` end up in binary
- Very fast to run (static dispatch)
- Slow to compile and larger binary

---
layout: default
---
# Limitations of `MyAdd`
What happens if...

- We want to add other types?
- We want to add separate types?
- Addition yields a different type?

---
layout: default
---
# `std::ops::Add`
The way `std` does it

```rust{all|1|2-4}
pub trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

- Trait itself is generic: allow adding separate types
- Associated type: allow defining addition ouput on implementation


---
layout: default
---
# `impl std::ops::Add`

```rust
pub struct BigNumber(u64);

impl std::ops::Add for BigNumber {
  type Output = Self;
  
  fn add(self, rhs: Self) -> Self::Output {
      BigNumber(self.0 + rhs.0)
  }
}

fn main() {
  let res = BigNumber(1) + BigNumber(2);
}
```

What's the type of `res`?

---
layout: default
---
# `impl std::ops::Add` (2)

```rust
pub struct BigNumber(u64);

impl std::ops::Add<u32> for BigNumber {
  type Output = u128;
  
  fn add(self, rhs: Self) -> Self::Output {
      (self.0 as u128) + (rhs as u128)
  }
}

fn main() {
  let res = BigNumber(1) + 3u32;
}
```

What's the type of `res`?

---
layout: default
---
# Traits: Type Parameter vs. Associated Type

### Type parameter (input type)
*if trait can be implemented for many combinations of types*
```rust
// We can add both a u32 value and a u32 reference to a u32
impl Add<u32> for u32 {/* */}
impl Add<&u32> for u32 {/* */}
```

### Associated type (output type)
*to define a type for a single implementation*
```rust
impl Add<u32> for u32 {
  // Addition of two u32's is always u32
  type Output = u32;
}
```

---
layout: section
---
# Common traits from `std`

<!-- TODO --->


---
layout: section
---
# Dynamic dispatch


---
layout: default
---
# Summary
- Traits describe functionality
- Generics allow writing in terms of traits
- Traits can be generic, too
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