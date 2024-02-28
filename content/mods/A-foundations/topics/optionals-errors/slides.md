
---
layout: section
---

# Optionals and Error handling
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
  let some_int = Option::Some(42);
  let no_string: Option<String> = Option::None;
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

---

# Error handling
What would we do when there is an error?

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

# Error handling
What would we do when there is an error?

```rust
fn divide(x: i64, y: i64) -> i64 {
  if y == 0 {
    panic!("Cannot divide by zero");
  } else {
    x / y
  }
}
```

* A panic in Rust is the most basic way to handle errors
* A panic error is an all or nothing kind of error
* A panic will immediately stop running the current thread/program and instead
  immediately work to shut it down, using one of two methods:
  * Unwinding: going up through the stack and making sure that each value
    is cleaned up
  * Aborting: ignore everything and immediately exit the thread/program
* Only use panic in small programs if normal error handling would also exit
  the program
* Avoid using panic in library code or other reusable components

<!--
* Unwinding has its usages, mainly to clean up resources that you previously
  opened.
* An unwind can be stopped, but this is highly unusual to do and very expensive
* In a multithreaded program unwinding is essential to make sure that any
  memory owned by that thread is freed, making sure you don't have any memory
  leaks
* Rust programs are compiled such that if a panic does not occur, it doesn't
  add any extra cost, but that does mean that if a panic does occur, it isn't
  very fast
* Generally panicking should be avoided as much as possible
* The panic! macro is not the only way to trigger a panic, so beware, we will
  see some ways we can also trigger a panic very soon
* Note that if the main thread panics, the entire program will always exit
-->

---

# Error handling
What would we do when there is an error? We could try and use the option enum
instead of panicking

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
Another really powerful enum is the result, which is even more useful if we
think about error handling

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

# Handling results
Now that we have a function that returns a result we have to think about how
we handle that error at the call-site

```rust
fn div_zero_fails() {
  match divide(10, 0) {
    Ok(div) => println!("{}", div),
    Err(e) => panic!("Could not divide by zero"),
  }
}
```

* We made the signature of the `divide` function explicit in how it can fail
* The user of the function can now decide what to do, even if it is panicking
* Note: just as with `Option` we never have to use `Result::Ok` and
  `Result::Err` because they have been made available globally


<!--
- Note how in this case the error still causes a panic, but at least we get a
  choice of what we do
-->

---

# Handling results
Especially when writing initial prototyping code you will often find yourself
wanting to write error handling code later, Rust has a useful utility function
to help you for both `Option` and `Result`:

```rust
fn div_zero_fails() {
  let div = divide(10, 0).unwrap();
  println!("{}", div);
}
```

* Unwrap checks if the Result/Option is `Ok(x)` or `Some(x)` respectively and
  then return that `x`, otherwise it will panic your program with an error
  message
* Having unwraps all over the place is generally considered a bad practice
* Sometimes you can ensure that an error won't occur, in such cases `unwrap`
  can be a good solution

---

# Handling results
Especially when writing initial prototyping code you will often find yourself
wanting to write error handling code later, Rust has a useful utility function
to help you for both `Option` and `Result`:

```rust
fn div_zero_fails() {
  let div = divide(10, 0).unwrap_or(-1);
  println!("{}", div);
}
```

Besides unwrap, there are some other useful utility functions

- `unwrap_or(val)`: If there is an error, use the value given to unwrap_or
  instead
- `unwrap_or_default()`: Use the default value for that type if there is an
  error
- `expect(msg)`: Same as unwrap, but instead pass a custom error message
- `unwrap_or_else(fn)`: Same as unwrap_or, but instead call a function that
  generates a value in case of an error

<!--
* unwrap_or_else is mainly useful if generating a default value is an expensive
  operation
-->

---

# Result and the `?` operator
Results are so common that there is a special operator associated with them, the
`?` operator

```rust
fn can_fail() -> Result<i64, DivideError> {
  let intermediate_result = match divide(10, 0) {
    Ok(ir) => ir,
    Err(e) => return Err(e),
  };

  match divide(intermediate_result, 0) {
    Ok(sec) => Ok(sec * 2),
    Err(e) => Err(e),
  }
}
```

<v-click>

Look how this function changes if we use the `?` operator

```rust
fn can_fail() -> Result<i64, DivideError> {
  let intermediate_result = divide(10, 0)?;
  Ok(divide(intermediate_result, 0)? * 2)
}
```

</v-click>

---

# Result and the `?` operator

```rust
fn can_fail() -> Result<i64, DivideError> {
  let intermediate_result = divide(10, 0)?;
  Ok(divide(intermediate_result, 0)? * 2)
}
```

* The `?` operator does an implicit match, if there is an error, that error
  is then immediately returned and the function returns early
* If the result is `Ok()` then the value is extracted and we can continue right
  away
