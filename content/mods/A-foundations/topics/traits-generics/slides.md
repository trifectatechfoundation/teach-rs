---
layout: section
---

# Traits and generics

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

No-one likes repeating themselves

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

An example
```rust
fn add<T>(lhs: T, rhs: T) -> T { /* - snip - */}
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
<br/>
Some open points:

- What can we do with a `T`?
- What should the body be?
</div>
</v-click>

---
layout: default
---
# Bounds on generic code
&nbsp;

We need to provide information to the compiler:
- Tell Rust what `T` can do
- Tell Rust what `T` is accepted
- Tell Rust how `T` implements functionality

---
layout: default
---

# `trait`
&nbsp;

Describe what the type can do
```rust
trait MyAdd {
    fn my_add(&self, other: &Self) -> Self;
}
```

---
layout: default
---
# `impl trait`
&nbsp;

Describe how the type does it

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

```rust{all|1-2|5-6|7-9|10-12}
// Import the trait
use my_mod::MyAdd

fn main() {
  let left: u32 = 6;
  let right: u32 = 8;
  // Call trait method
  let result = left.my_add(&right);
  assert_eq!(result, 14);
  // Explicit call
  let result = MyAdd::my_add(&left, &right);
  assert_eq!(result, 14);
}
```

- Trait needs to be in scope
- Call just like a method
- Or by using the explicit associated function syntax

---
layout: default
---
# Trait bounds

```rust{all|1-3,5|5,7-11}
fn add_values<T: MyAdd>(this: &T, other: &T) -> T {
  this.my_add(other)
}

// Or, equivalently

fn add_values<T>(this: &T, other: &T) -> T 
  where T: MyAdd
{
  this.my_add(other)
}
```

Now we've got a *useful* generic function!

English: *"For all types `T` that implement the `MyAdd` `trait`, we define..."*

---
layout: default
---
# Limitations of `MyAdd`
What happens if...

- We want to add two values of different types?
- Addition yields a different type?

---
layout: default
---

# Making `MyAdd` itself generic
&nbsp;

Add an 'Input type' `O`:

```rust{all|1-3|5-9}
trait MyAdd<O> {
    fn my_add(&self, other: &O) -> Self;
}

impl MyAdd<u16> for u32 {
    fn my_add(&self, other: &u16) -> Self {
      *self + (*other as u32)
    }
}
```

We can now add a `u16` to a `u32`.

---
layout: default
---

# Defining output of `MyAdd`

- Addition of two given types always yields in one specific type of output
- Add *associated type* for addition output

```rust{all|2-3|7-9|6-20}
trait MyAdd<O> {
    type Output;
    fn my_add(&self, other: &O) -> Self::Output;
}

impl MyAdd<u16> for u32 {
    type Output = u64;

    fn my_add(&self, other: &u16) -> Self::Output {
      *self as u64 + (*other as u64)
    }
}

impl MyAdd<u32> for u32 {
    type Output = u32;

    fn my_add(&self, other: &u32) -> Self::Output {
      *self + *other
    }
}
```

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

- Default type of `Self` for `Rhs`

---
layout: default
---
# `impl std::ops::Add`

```rust
use std::ops::Add;
pub struct BigNumber(u64);

impl Add for BigNumber {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
      BigNumber(self.0 + rhs.0)
  }
}

fn main() {
  // Call `Add::add`
  let res = BigNumber(1).add(BigNumber(2));
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
  
  fn add(self, rhs: u32) -> Self::Output {
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
layout: default
---

# `#[derive]` a `trait`

```rust
#[derive(Clone)]
struct Dolly {
  num_legs: u32,
}

fn main() {
  let dolly = Dolly { num_legs: 4 };
  let second_dolly = dolly.clone();
  assert_eq!(dolly.num_legs, second_dolly.num_legs);
}
```

- Some traits are trivial to implement
- Derive to quickly implement a trait
- For `Clone`: derived `impl` calls `clone` on each field 

---
layout: default
---
# Orphan rule

*Coherence: There must be **at most one** implementation of a trait for any given type*

Trait can be implemented for a type **iff**:
- Either your crate defines the trait
- Or your crate defines the type

Or both, of course

---
layout: default
---

# Compiling generic functions

```rust
impl MyAdd for i32 {/* - snip - */}
impl MyAdd for f32 {/* - snip - */}

fn add_values<T: MyAdd>(left: &T, right: &T) -> T
{
  left.my_add(right)
}

fn main() {
  let sum_one = add_values(&6, &8);
  assert_eq!(sum_one, 14);
  let sum_two = add_values(&6.5, &7.5);
  println!("Sum two: {}", sum_two); // 14
}
```

Code is <em>monomorphized</em>:
 - Two versions of `add_values` end up in binary
 - Optimized separately and very fast to run (static dispatch)
 - Slow to compile and larger binary
