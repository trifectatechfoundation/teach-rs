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

# Pattern matching
To extract data from enums we can use pattern matching using the
`if let [pattern] = [value]` statement

```rust
fn accept_ipv4(ip: IpAddress) {
  if let IpAddress::Ipv4(a, b, _, _) = ip {
    println!("Accepted, first octet is {} and second is {}", a, b);
  }
}
```

* `a` and `b` introduce local variables within the body of the if that contain
  the values of those fields
* The underscore (`_`) can be used to accept any value

---

# Match
But pattern matching is very powerful if combined with the match statement

```rust
fn accept_home(ip: IpAddress) {
  match ip {
    IpAddress::Ipv4(127, 0, 0, 1) => {
      println!("You are home!");
    },
    IpAddress::Ipv6(0, 0, 0, 0, 0, 0, 0, 1) => {
      println!("You are in your new home!");
    },
    _ => {
      println!("You are not home");
    },
}
```

* Every part of the match is called an arm
* A match is exhaustive, which means that all values must be handled by one of
  the match arms
* You can use a catch-all `_` arm to catch any remaining cases if there are any
  left

---

# Match as an expression
The match statement can even be used as an expression

```rust
fn get_first_byte(ip: IpAddress) {
  let first_byte = match ip {
    IpAddress::Ipv4(a, _, _, _) => a,
    IpAddress::Ipv6(a, _, _, _, _, _, _, _) => a / 256 as u8,
  };
  println!("The first byte was: {}", first_byte);
}
```

* The match arms can return a value, but their types have to match
* Note how here we do not need a catch all `_` arm because all cases have
  already been handled by the two arms

---

# Generics
Enums become even more powerful if we introduce a little of generics

```rust
struct PointFloat(f64, f64);
struct PointInt(i64, i64);
```

We are repeating ourselves here, what if we could write a datastructure for
both of these cases?

<v-click>

```rust
struct Point<T>(T, T);

fn main() {
  let float_point: Point<f64> = Point(10.0, 10.0);
  let int_point: Point<i64> = Point(10, 10);
}
```

Generics are much more powerful, but this is all we need for now

</v-click>

<!--
* The upper case letter between the angled brackets introduces a generic type
  parameter.
* We can now use that generic type variable we introduced as a type name
* Then at the point of using the type we can specify which actual type we
  want to use
* Generics are much more powerful, but this is enough for now
-->

---

# Option
A quick look into the basic enums available in the standard library

* Rust does not have null, but you can still define variables that optionally
  do not have a value
* For this you can use the `Option<T>` enum

```rust
enum Option<T> {
  Some(T),
  None,
}

fn main() {
  let some_int = Some(42);
  let no_string: Option<String> = None;
}
```

<!--
* Note how Rust can infer the type of `some_int`, but we have to specify what
  the type of the Option is in the None case, because it cannot possibly know
  what kind of values you could put in that Option
* Also not that for normal enums we have to import the variants, but Option
  is so common that the variants are available by default without needing to
  prefix them with `Option::`
-->

---

# Result
Another really powerful enum is the result, to understand the usage of this
enum we have to think about error handling

```rust
fn divide(x: i64, y: i64) -> i64 {
  if y == 0 {
    // what to do now?
  } else {
    x / y
  }
}
```

---

# Result
Another really powerful enum is the result, to understand the usage of this
enum we have to think about error handling

```rust
fn divide(x: i64, y: i64) -> Option<i64> {
  if y == 0 {
    None
  } else {
    Some(x / y)
  }
}
```

---

# Result
Another really powerful enum is the result, to understand the usage of this
enum we have to think about error handling

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}

enum DivideError {
  DivisionByZero,
  CannotDivideOne,
}

fn divide(x: i64, y: i64) -> Result<i64, DivideError> {
  if x == 1 {
    Err(DivideError::CannotDivideOne)
  } else if y == 0 {
    Err(DivideError::DivisionByZero)
  } else {
    Ok(x / y)
  }
}
```

---

# Result and the try operator
Results are so common that there is a special operator associated with them, the
try operator

```rust
fn can_fail() -> Result<i64, Error> {
  let intermediate_result = match some_failable_operation() {
    Ok(ir) => ir,
    Err(e) => return Err(e);
  };

  match some_secondary_operation(intermediate_result) {
    Ok(sec) => Ok(sec * 2),
    Err(e) => Err(e),
  }
}
```

<v-click>

Look how this function changes if we use the try operator

```rust
fn can_fail() -> Result<i64, Error> {
  let intermediate_result = some_failable_operation()?;
  Ok(some_secondary_operation(intermediate_result)?)
}
```

</v-click>

---

# Result and the try operator

```rust
fn can_fail() -> Result<i64, Error> {
  let intermediate_result = some_failable_operation()?;
  Ok(some_secondary_operation(intermediate_result)?)
}
```

* The try operator does an implicit match, if there is an error, that error
  is then immediately returned and the function returns early
* If the result is `Ok()` then the value is extracted and we can continue right
  away

---

# Lifetimes
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

---

# String literals

---

# Slices

---

# Vec(tor)

---

# Box

---

# Pointers

---

# Ownership

---

# Borrowing

---

# Lifetimes

---

# Lifetime annotations

---

# Panic, another kind of error

---

# When (not) to panic
