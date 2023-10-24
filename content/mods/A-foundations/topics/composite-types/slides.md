
---
layout: section
---

# Composite types

---

# Types redux
We have previously looked at some of the basic types in the Rust typesystem

- Primitives (integers, floats, booleans, characters)
- Compounds (tuples, arrays)
- Most of the types we looked at were `Copy`
- Borrowing will make more sense when we look at some more ways we can type
  our data

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

```rust{all|2|all}
enum IpAddressType {
  Ipv4,
  Ipv6,
}
```

<v-click>

* An enumeration (listing) of different *variants*
* Each variant is an alternative value of the enum, you pick a single value to
  create an instance
* Each variant has a discriminant (hidden by default)
  * a numeric value (`isize` by default, can be changed by using `#[repr(numeric_type)]`) used to determine the variant that the enumeration holds 
  * one cannot rely on the fact that the discriminant is an `isize`, the compiler may always decide to optimize it

</v-click>

<v-click>

```rust
fn main() {
  let ip_type = IpAddressType::Ipv4;
}
```

</v-click>

---

# Enumerations
Enums get more powerful, because each variant can have associated data with
it

```rust
enum IpAddress { 
  Ipv4(u8, u8, u8, u8),                           // = 0 (default discriminant)
  Ipv6(u16, u16, u16, u16, u16, u16, u16, u16),   // = 1 (default discriminant)
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

* An enum always is as large as the largest variant plus the size of the discriminant

<div style="margin-left:auto; margin-right:auto; display:block; width:100%;">

<LightOrDark>
    <template #dark>
        <div style="padding: 20px; background-color:#1b1b1b; border-radius: var(--slidev-code-radius) !important;">
            <img src="/images/A2-enum-memory-dark.drawio.svg"/>
        </div>
    </template>
    <template #light>
        <div style="padding: 20px; background-color:#F8F8F8; border-radius: var(--slidev-code-radius) !important;">
          <img src="/images/A2-enum-memory-light.drawio.svg"/>
        </div>
    </template>
</LightOrDark>

</div>
