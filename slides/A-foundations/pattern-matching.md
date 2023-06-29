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
Pattern matching is very powerful if combined with the match statement

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
Structs become even more powerful if we introduce a little of generics

```rust
struct PointFloat(f64, f64);
struct PointInt(i64, i64);
```

We are repeating ourselves here, what if we could write a data structure for
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
