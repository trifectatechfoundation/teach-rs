
---
layout: section
---

# Ownership and borrowing

---

# Ownership
We previously talked about ownership

* In Rust there is always a single owner for each stack value
* Once the owner goes out of scope any associated values should be cleaned up
* Copy types creates copies, all other types are *moved*

<!--
- Note once more that the idea of moving is something that exists in the Rust
  world, but not necesarrily every move actually copies bytes around, these are
  all things where Rust's model is an abstraction over what the compiled code
  actually does.
-->

---

# Moving out of a function
We have previously seen this example


```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: String) -> usize {
    s.len()
}
```

* This does not compile because ownership of `s1` is moved into
 `calculate_length`, meaning it is no longer available in `main` afterwards
* We can use `Clone` to create an explicit copy
* We can give ownership back by returning the value
* What about other options?

---

# Borrowing
- We can make an analogy with real life: if somebody owns something you can
  borrow it from them, but eventually you have to give it back
- If a value is borrowed, it is not moved and the ownership stays with the
  original owner
- To borrow in Rust, we create a *reference*

```rust {all|3|7|all}
fn main() {
    let x = String::from("hello");
    let len = get_length(&x);
    println!("{}: {}", x, len);
}

fn get_length(arg: &String) -> usize {
    arg.len()
}
```

---

# References (immutable)

```rust
fn main() {
    let s = String::from("hello");
    change(&s);
    println!("{}", s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

<v-click>

<div class="no-line-numbers">

```text
   Compiling playground v0.0.1 (/playground)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `playground` due to previous error
```

</div>

</v-click>

<!--
- Note how we cannot modify the referenced value through an immutable reference
-->

---

# References (mutable)

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

<v-click>

<div class="no-line-numbers">

```text
   Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 2.55s
     Running `target/debug/playground`
hello, world
```

</div>

</v-click>

<v-click>

- A mutable reference can even fully replace the original value
- To do this, you can use the dereference operator (`*`) to modify the value:

```rust
*some_string = String::from("Goodbye");
```

</v-click>

<!--
- We can use a mutable reference here to allow us to modify a borrowed value
- Note that you may also sometimes have to use the deref operator to access
  the value when reading it, but most of the time the Rust compiler will do
  this automatically and you need not worry about it.
-->

---


# Rules for borrowing and references

- You may only ever have **one mutable reference** at the same time
- You may have **any number of immutable references** at the same time **as long as
  there is no mutable reference**
- References cannot *live* longer than their owners
- A reference will always at all times *point to a valid value*

These rules are enforced by Rust's borrow checker.

<!--
- Rust tries to be smart about enforcing these rules, such that you don't notice
  them that often in real life usage, but there may be some cases that clearly
  appear valid, but Rust won't allow. There are generally pretty easy workarounds
  though
- Again: references are not pointers, but in practice of course they do look
  similar and are implemented the same way, but Rust's memory model is not the
  same as that of C/C++ and implementation is not the same as our model.
-->

---

# Borrowing and memory safety
Combined with the ownership model we can be sure that whole classes of errors
cannot occur.

* Rust is memory safe without having to use any runtime background process such
  as a garbage collector
* But we still get the performance of a language that would normally let you
  manage memory manually

<!--
- Memory bugs such as: null pointer dereferences, data races, dangling pointers,
  use after free.
-->

---

# Reference example

```rust
fn main() {
    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    let s3 = &mut s;
    println!("{} - {} - {}", s1, s2, s3);
}
```

<v-click>

<div class="no-line-numbers">

```text
   Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:14
  |
3 |     let s1 = &s;
  |              -- immutable borrow occurs here
4 |     let s2 = &s;
5 |     let s3 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
6 |     println!("{} - {} - {}", s1, s2, s3);
  |                              -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `playground` due to previous error
```

</div>

</v-click>

---

# Returning references

You can return references, but the value borrowed from must exist at least as
long

```rust
fn give_me_a_ref() -> &String {
    let s = String::from("Hello, world!");
    &s
}
```

<v-click>

<div class="no-line-numbers">

```md {8}
   Compiling playground v0.0.1 (/playground)
error[E0106]: missing lifetime specifier
 --> src/lib.rs:1:23
  |
1 | fn give_me_a_ref() -> &String {
  |                       ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
1 | fn give_me_a_ref() -> &'static String {
  |                       ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `playground` due to previous error
```

</div>

</v-click>

---

# Returning references

You can return references, but the value borrowed from must exist at least as
long

```rust
fn give_me_a_ref(input: &(String, i32)) -> &String {
    &input.0
}
```

<v-click>

```rust
fn give_me_a_value() -> String {
    let s = String::from("Hello, world!");
    s
}
```

</v-click>
