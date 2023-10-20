
---
layout: section
---

# `impl` blocks

---

# `impl` blocks
To associate functions to `structs` and `enums`, we use `impl` blocks

```rust {3}
fn main() {
  let x = Some(42);
  let unwrapped = x.unwrap();
  println!("{}", unwrapped);
}
```

* The syntax `x.y()` looks similar to how we accessed a field in a struct
* We can define functions on our types using impl blocks
* Impl blocks can be defined on any type, not just structs (with some limitations)

---

# `impl` blocks

```rust {all|6,13|7-12|7|17}
enum IpAddress {
  Ipv4(u8, u8, u8, u8),
  Ipv6(u16, u16, u16, u16, u16, u16, u16, u16),
}

impl IpAddress {
  fn as_u32(&self) -> Option<u32> {
    match self {
      IpAddress::Ipv4(a, b, c, d) => a << 24 + b << 16 + c << 8 + d
      _ => None,_
    }
  }
}

fn main() {
  let addr = IpAddress::Ipv4(127, 0, 0, 1);
  println!("{:?}", addr.as_u32());
}
```

<!--
- Here we define the as_u32 method
- Note how the impl block is separate from the type definition
- In fact we can have multiple impl blocks for the same type, as long as
  function definitions do not overlap (not useful right now, but it will be
  once we get more into generics)
-->

---

# `self` and `Self`

- The `self` parameter defines how the method can be used.
- The `Self` type is a shorthand for the type on which the current
  implementation is specified.

```rust {all|4-6|8-14|16-18}
struct Foo(i32);

impl Foo {
  fn consume(self) -> Self {            // Takes `Foo` by value, returns `Foo`
    Self(self.0 + 1)
  }

  fn borrow(&self) -> &i32 {            // Takes immutable reference of `Foo`
    &self.0
  }

  fn borrow_mut(&mut self) -> &mut i32 { // Takes mutable reference of `Foo`
    &mut self.0
  }

  fn new() -> Self {                     // Associated function, returns `Foo`
    Self(0)
  }
}
```

---

# `impl` blocks, the `self` parameter
The self parameter is called the *receiver*.

* The `self` parameter is always the first and it always has the type on which it
  was defined
* We never specify the type of the `self` parameter
* We can optionally prepend `&` or `&mut ` to `self` to indicate that we take
  a value by reference
* Absence of a `self` parameter means that the function is an associated function
  instead

```rust
fn main () {
  let mut f = Foo::new();
  println!("{}", f.borrow());
  *f.borrow_mut() = 10;
  let g = f.consume();
  println!("{}", g.borrow());
}
```
