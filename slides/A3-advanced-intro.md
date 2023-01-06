---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - A3: Advanced Syntax, Ownership, References"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - A3: Advanced Syntax, Ownership, References"
---

# In this module

<!-- Introduce today's subject -->
Advanced Rust syntax


---

# Types redux
We have previously looked at some of the basic types in the Rust typesystem

- Primitives (integers, floats, booleans, characters)
- Compounds (tuples, arrays)

---
# Structuring data
Rust has two important ways to structure data

* structs
* enums
* ~~unions~~

<!--
- We have unions in Rust, but almost everywhere you will use enums instead.
  Unions become relevant once we start talking about FFI and unsafe Rust code.
-->

---

# Structs
A struct is similar to a tuple, but this time the combined type gets its own name

```rust
struct ControlPoint(f64, f64, bool);
```

<v-click>

This is an example of a *tuple struct*. You can access the fields in the struct
the same way as with tuples:

```rust
fn main() {
  let cp = ControlPoint(10.5, 12.3, true);
  println!("{}", cp.0); // prints 10.5
}
```

</v-click>

<!--
- Note that two tuples with the same fields in the same order are always the
  same type, whereas two structs with different names but the same fields are
  different types.
-->

---

# Structs
Much more common though are structs with named fields

```rust
struct ControlPoint {
  x: f64,
  y: f64,
  enabled: bool,
}
```

* We can add a little more purpose to each field
* No need to keep our indexing up to date when we add or remove a field

<v-click>


```rust {all|2-6|7}
fn main() {
  let cp = ControlPoint {
    x: 10.5,
    y: 12.3,
    enabled: true,
  };
  println!("{}", cp.x); // prints 10.5
}
```

</v-click>

<!--
- Named fields are especially easier in usage, as a type alone will most of
  the time not be enough information to determine the full meaning, here we now
  now that the two floats meant the x and y coordinates and we know what the
  boolean indicated.
- To instantiate (create a value) of a struct we use the syntax shown
- Now, we can use the same `x.y` syntax as with tuples, but we have a nice
  name for referencing our fields instead of having to remember the exact
  field index.
-->

---

# Enumerations
One of the more powerful kinds of types in Rust are enumerations

```rust
enum IpAddressType {
  Ipv4,
  Ipv6,
}
```

* An enumeration (listing) of different *variants*
* Each variant is an alternative value of the enum, you pick a single value to
  create an instance

<v-click>

```rust
fn main() {
  let ip_type = IpAddressType::Ipv4;
}
```

</v-click>


---

# Enumerations
But enums get more powerful, because each variant can have associated data with
it

```rust
enum IpAddress {
  Ipv4(u8, u8, u8, u8),
  Ipv6(u16, u16, u16, u16, u16, u16, u16, u16),
}
```

* This way, the associated data and the variant are bound together
* Impossible to create an ipv6 address while only giving a 32 bits integer

```rust
fn main() {
  let ipv4_home = IpAddress::Ipv4(127, 0, 0, 1);
  let ipv6_home = IpAddress::Ipv6(0, 0, 0, 0, 0, 0, 0, 1);
}
```

---

# One more thing...
We've now discussed all ways we could store and structure our data in Rust

* Combining our primitive types, our basic compounds, and by structuring data
  using structs and enums we should be able to model almost any data we
  encounter.
* But there is one more type you really need to know about, and it concerns the
  Rust ownership system.

---

# Strings
We have already seen the `String` type being used before, but let's dive a
little deeper

* Strings are used to represent text
* In Rust they are always valid UTF-8
* Their data is stored on the heap
<v-click>

* A string consists of three pieces of information:
  * The _length_ of the string; they are **not** null terminated
  * The _capacity_: how much space was reserved for the string to grow into
  * A _pointer_ to where the actual string data is stored in memory

</v-click>

<!--
- We store data on the heap so we can easily have strings of variable sizes
  and grow and shrink them as needed when they are modified.
- In general we really don't care about the exact length of the string
-->

---

# Strings
Let's take a look at some strings

```rust
fn main() {
  let
}

```

