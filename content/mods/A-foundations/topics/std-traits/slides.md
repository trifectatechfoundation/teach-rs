
---
layout: section
---
# Common traits from `std`

---
layout: default
---
# Operator overloading: `std::ops::Add<T>` et al.

- Shared behavior

```rust{all|13-14}
use std::ops::Add;
pub struct BigNumber(u64);

impl Add for BigNumber {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
      BigNumber(self.0 + rhs.0)
  }
}

fn main() {
  // Now we can use `+` to add `BigNumber`s!
  let res: BigNumber = BigNumber(1) + (BigNumber(2));
}
```

- Others: `Mul`, `Div`, `Sub`, ..

---
layout: default
---
# Markers: `std::marker::Sized`

- Marker traits

```rust
/// Types with a constant size known at compile time.
/// [...]
pub trait Sized { }
```

*`u32` is `Sized`*

*Slice `[T]`, `str` is **not** `Sized`*

*Slice reference `&[T]`, `&str` is `Sized`*

Others:
- `Sync`: Types of which references can be shared between threads
- `Send`: Types that can be transferred across thread boundaries

---
layout: default
---
# Default values: `std::default::Default`

```rust{all|5|10-17}
pub trait Default: Sized {
    fn default() -> Self;
}

#[derive(Default)] // Derive the trait
struct MyCounter {
  count: u32,
}

// Or, implement it
impl Default for MyCounter {
  fn default() -> Self {
    MyCounter {
      count: 1, // If you feel so inclined
    }
  }
}
```

---
layout: default
---

# Duplication: `std::clone::Clone` & `std::marker::Copy`
```rust{all|9|4-6}
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) {
      *self = source.clone()
    }
}

pub trait Copy: Clone { } // That's it!
```

- Both `Copy` and `Clone` can be `#[derive]`d
- `Copy` is a marker trait
- `trait A: B` == "Implementor of `A` must also implement `B`"
- `clone_from` has default implementation, can be overridden

---
layout: default
---

# Conversion: `Into<T>` & `From<T>`
```rust{all|1-3|5-7|9-15}
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}

impl <T, U> Into<U> for T
  where U: From<T>
{
    fn into(self) -> U {
      U::from(self)
    }
}
```

- Blanket implementation

*Prefer `From` over `Into` if orphan rule allows to*

---
layout: default
---
# Reference conversion: `AsRef<T>` & `AsMut<T>`

```rust
pub trait AsRef<T: ?Sized>
{
    fn as_ref(&self) -> &T;
}

pub trait AsMut<T: ?Sized>
{
    fn as_mut(&mut self) -> &mut T;
}
```

- Provide flexibility to API users
- `T` need not be `Sized`, e.g. slices `[T]` can implement `AsRef<T>`, `AsMut<T>`

---
layout: default
---
# Reference conversion: `AsRef<T>` & `AsMut<T>` (2)

```rust{all|1-2|10-11|13-14}
fn print_bytes<T: AsRef<[u8]>>(slice: T) {
  let bytes: &[u8] = slice.as_ref();
  for byte in bytes {
    print!("{:02X}", byte);
  }
  println!();
}

fn main() {
  let owned_bytes: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
  print_bytes(owned_bytes);

  let byte_slice: [u8; 4] = [0xFE, 0xED, 0xC0, 0xDE];
  print_bytes(byte_slice);
}
```

*Have user of `print_bytes` choose between stack local `[u8; N]` and heap-allocated `Vec<u8>`*

---
layout: default
---
# Destruction: `std::ops::Drop`

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

- Called when owner goes out of scope


---
layout: two-cols
---
# Destruction:`std::ops::Drop`

```rust{all|1-7|9-17|19-22}
struct Inner;

impl Drop for Inner {
  fn drop(&mut self) {
    println!("Dropped inner");
  }
}

struct Outer {
  inner: Inner,
}

impl Drop for Outer {
  fn drop(&mut self) {
    println!("Dropped outer");
  }
}

fn main() {
  // Explicit drop
  std::mem::drop(Outer { inner: Inner });
}
```
::right::

# &nbsp;
<v-click>

<div class="no-line-numbers">
<br/>
Output:
```text
Dropped outer
Dropped inner
```
</div>

- Destructor runs *before* members are removed from stack
- Signature `&mut` prevents explicitly dropping `self` or its fields in destructor
- Compiler inserts `std::mem::drop` call at end of scope

```rust
// Implementation of `std::mem::drop`
fn drop<T>(_x: T) {}
```

*Question: why does `std::mem::drop` work?*

</v-click>