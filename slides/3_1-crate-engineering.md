---
theme: "teach-rs"
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 3.1: Crate Engineering"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 3.1: Crate Engineering"
---

# Rust programming

Module 3: Crate Engineering

## Unit 1

Crate Engineering

---

# Learning objectives



---
layout: cover
---
# Creating a nice API

<!--
    A big part of developing a larger project is to define a nice, readable API that clarifies intent. Let's have a look at some guidelines Rust specifies in order to make crates useful for others or even future you.
-->
---
layout: two-cols
---

# Rust API guidelines

- Defined by Rust project
- [Checklist available](https://rust-lang.github.io/api-guidelines/checklist.html) (Link in exercises)

::right::
<img src="/images/B-api-guidelines.png" style="margin-top: 20%; margin-left:5%;max-width: 100%; max-height: 90%;">


**Read the checklist, use it!**

<!--
To improve consistency between crates, Rust defines a whole lot of guidelines. There's even a checklist available. In this section, we'll focus on some guidelines. However, make sure to use the checklist for all code you write in exercises!
-->

---
layout: default
---

# General recommendations

Make your API
- Unsurprising
- Flexible
- Obvious

**Next up: Some low-hanging fruits**

<!--
In general, you want to make your API unsurprising, flexible and obvious.
- An unsurprising API uses patterns that are broadly used, allowing the user to guess how the interface is structured.
- Flexible APIs are suitable for many applications, and only as restrictive as they inherently need to be.
- Make your API obvious to enable users to quickly understand rules of your library.
-->

---
layout: cover
---
Make your API
# Unsurprising

<!--
So, how to make your API unsurprising?
-->
---
layout: default
---
# Naming your methods

```rust {all|7-10|12-15}
pub struct S {
    first: First,
    second: Second,
}

impl S {
    // Not get_first.
    pub fn first(&self) -> &First {
        &self.first
    }

    // Not get_first_mut, get_mut_first, or mut_first.
    pub fn first_mut(&mut self) -> &mut First {
        &mut self.first
    }
}
```

Other example: conversion methods `as_`, `to_`, `into_`, name depends on:
- Runtime cost
- Owned &harr; borrowed


<!--
An easy way of making your API unsurprising is by adhering to naming conventions.
- For example, the guidelines specify a naming convention for getters. Note that getter names do not start with 'get', and that the mutable getter ends with 'mut'.
- Another example is the way conversion methods are named, based on their runtime cost and whether the conversion is between references, owned values, or from reference to owned and vice-versa. 
-->

---
layout: two-cols
---

# Implement/derive common traits


*As long as it makes sense* public types should implement:

- `Copy`
- `Clone`
- `Eq`
- `PartialEq`
- `Ord`
- `PartialOrd`

::right::

<div style="margin-top: 150px"></div>

- `Hash`
- `Debug`
- `Display`
- `Default`
- `serde::Serialize`
- `serde::Deserialize`

<!--
Here's a list of common traits to implement or derive automatically, making your types more useful to others
-->

---
layout: cover
---
Make your API
# Flexible
<!--
Now, let's take a look at some ways to make your API flexible
-->

---
layout: default
---

# Use generics

```rust {all|1-3|5-9}
pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

/// Adds two values that implement the `Add` trait,
/// returning the specified output
pub fn add_generic<O, T: std::ops::Add<Output = O>>(x: T, y: T) -> O {
    x + y
}
```

<!--
An great way to lift restrictions on your API is to write your functions in terms of traits. That is, use generics to describe what exactly is needed to perform a certain action.
- In this example, the first function only accepts `u32`, whereas there are many other numeric types for which addition makes sense.
- The second example, though, accepts anything for which the `Add` trait is implemented. It is even generic over the addition output.
-->

---
layout: default
---

# Accept borrowed data if possible

- User decides whether calling function should own the data
- Avoids unnecessary moves
- Exception: non-big array `Copy` types

```rust {all|6-9|11-14}
/// Some very large struct
pub struct LargeStruct {
    data: [u8; 4096],
}

/// Takes owned [LargeStruct] and returns it when done
pub fn manipulate_large_struct(mut large: LargeStruct) -> LargeStruct {
    todo!()
}

/// Just borrows [LargeStruct]
pub fn manipulate_large_struct_borrowed(large: &mut LargeStruct) {
    todo!()
}
```

<!--
An even neater way to make your API flexible is by allowing the user to decide whether the data that is passed to your function is owned by the calling function or not. This is done by accepting borrowed data wherever possible. This avoids unnecessary moves. An exception to this guideline is for `Copy` types, as they are cheap to clone. 
- The first function in the example moves the `LargeStruct` into its own scope, and then moves it out again. That may be costly and requires ownership from calling function!
- The second function merely borrows the `LargeStruct`, which is cheap and flexible.
-->

---
layout: cover
---
Make your API
# Obvious

<!--
Lastly, we'll make our APIs obvious
-->

---
layout: two-cols
---

# Write Rustdoc

- Use 3 forward-slashes to start a doc comment
- You can add code examples, too

```rust
/// A well-documented struct.
/// ```rust
/// # // lines starting with a `#` are hidden
/// # use ex_b::MyDocumentedStruct;
/// let my_struct = MyDocumentedStruct {
///     field: 1,
/// };
/// println!("{:?}", my_struct.field);
/// ```
pub struct MyDocumentedStruct {
    /// A field with data
    pub field: u32,
}
```

*To open docs in your browser:*
```bash
$ cargo doc --open
```
::right::
<img src="/images/B-rustdoc.png" style="margin-left:5%;max-width: 100%; max-height: 90%;">

<!--
Lots of respect for authors of good documentation! If you find writing documentation hard, keep in mind that you may be writing this for your future self.

- A doc comment in Rust starts with three forward-slashes
- You can add code examples, which can even be tested automatically to ensure they're not out of date after a refactor.
- Using `cargo doc --open`, you can render documentation locally and open it in your browser. Here's how it looks: [see image]

-->

---
layout: default
---

# Include examples

Create examples to show users how to use your library

```txt{all}
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── examples
│   └── say_hello.rs
└── src
    └── lib.rs
$ cargo run --example say_hello
   Compiling my_app v0.1.0 (/home/henkdieter/tg/edu/my_app)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/examples/say_hello`
Hello, henkdieter!

```

<!--
- If you're writing a library, adding a couple of examples helps your users get started. In fact, many libraries are accompanied with examples defined in their Git repositories.
- Run examples with the `--example` option, specifying the binary
-->


---
layout: default
---

# Use semantic typing (1)

Make the type system work for you!

```rust {all|1-2|7-8}
/// Fetch a page from passed URL
fn load_page(url: &str) -> String {
    todo!("Fetch");
}

fn main() {
  let page = load_page("https://teach-rs.tweede.golf");
  let crab = load_page("🦀"); // Ouch!
}
```

*`&str` is not restrictive enough: not all `&str` represent correct URLs*

<!--
Rusts type system is awesome. Use it to you advantage by embedding semantics into your types.
- As an example, the `load_page` function takes a string slice, indicating the URL of the page that it should load.
- At the call site of load_page, it's unclear what a page even is (memory page? remote content?)
- `load_page` accepts all strings, even strings that do not represent valid URLs
-->

---
layout: two-cols
---

# Use semantic typing (2)

```rust{all|1-3,14-16|5-12,22-24|18-20|all}
struct Url<'u> {
  url: &'u str,
}

impl<'u> Url<'u> {
  fn new(url: &'u str) -> Self {
    if !valid(url) {
      panic!("URL invalid: {}", url);
    }
    Self { url }
  }
}

fn load_page(remote: Url) -> String {
    todo!("load it");
}
fn main() {
    let content = load_page(Url::new("🦀")); // Not good
}
fn valid(url: &str) -> bool {
    url != "🦀" // Far from complete
}
```
::right::

<v-click>
<div style="padding-left:10px; padding-top: 50px;">
```txt
   Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 2.90s
     Running `target/debug/playground`
thread 'main' panicked at 'URL invalid: 🦀', src/main.rs:11:7
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- Clear intent
- Input validation: security!

*Use the [`url`](https://lib.rs/url) crate*
</div>

</v-click>

<!--
 - The `Url` struct defined here, wraps a string slice, but has a name that clarifies intent at the call site
 - what's more, the `Url` struct can only be instantiated with strings that represent valid URLs
-->

---
layout: quote
---

# Use Clippy and Rustfmt for all your projects!

```bash
$ cargo clippy
$ cargo fmt
```

<!--
Use Clippy and Rustfmt to help adhering to the guidelines
-->
---
layout: cover
---

# Design patterns in Rust

---
layout: default
---

# Why learn design patterns?

- Common problems call for common, tried an tested solutions
- Make crate architecture more clear
- Speed up development
- Rust does some patterns ever-so-slightly differently

*Learning common Rust patterns makes understanding new code easier*

---
layout: default
---

# What we'll do

```rust
const PATTERNS: &[Pattern] = &[
    Pattern::new("Newtype"),
    Pattern::new("RAII with guards"),
    Pattern::new("Typestate"),
    Pattern::new("Strategy"),
];
fn main() {
    for pattern in PATTERNS {
        pattern.introduce();
        pattern.show_example();
        pattern.when_to_use();
    }
}
```

---
layout: cover
---

# 1. The Newtype pattern
a small but useful pattern

---
layout: default
---

# Newtype: introduction
&nbsp;

### Wrap an external type in a new local type

```rust
pub struct Imei(String)
```

That's it!

---
layout: default
---

# Newtype: example

```rust
pub enum ValidateImeiError { /* - snip - */}

pub struct Imei(String);

impl Imei {
    fn validate(imei: &str) -> Result<(), ValidateImeiError> {
        todo!();
    }
}

impl TryFrom<String> for Imei {
    type Error = ValidateImeiError;

    fn try_from(imei: String) -> Result<Self, Self::Error> {
        Self::validate(&imei)?;
        Ok(Self(imei))
    }
}

fn register_phone(imei: Imei, label: String) {
    // We can certain `imei` is valid here
}
```

---
layout: default
---

# Newtype: when to use

Newtype solves some problems:
- Orphan rule: no `impl`s for external `trait`s on external types
- Allow for semantic typing (`url` example from mod B)
- Enforce input validation

---
layout: cover
---

# 2. The RAII guard pattern
More robust resource handling

---
layout: default
---

# RAII Guards: introduction

- Resource Acquisition Is Initialization (?)
- Link acquiring/releasing a resource to the lifetime of a variable
- Guard constructor initializes resource, destructor frees it
- Access resource through the guard

*Do you know of an example?*

---
layout: two-cols
---

# RAII Guards: example

```rust
pub struct Transaction<'c> {
    connection: &'c mut Connection,
    did_commit: bool,
    id: usize,
}

impl<'c> Transaction<'c> {
    pub fn begin(connection: &'c mut Connection)
     -> Self {
        let id = 
            connection.start_transaction();
        Self {
            did_commit: false,
            id,
            connection,
        }
    }

    pub fn query(&self sql: &str) { /* - snip - */}

    pub fn commit(self) {
        self.did_commit = true;
    }
}
```
::right::
<div style="padding-left:10px; padding-top: 50px;">

```rust
impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if self.did_commit {
            self
                .connection
                .commit_transaction(self.id);
            
        } else {
            self
                .connection
                .rollback_transaction(self.id);
        }
    }
}
```
</div>

---
layout: default
---

# RAII Guards: when to use

- Ensure a resource is freed at some point
- Ensure invariants hold while guard lives

---
layout: cover
---

# 3. The Typestate pattern
Encode state in the type

---
layout: default
---

# Typestate: introduction

- Define uninitializable types for each state of your object
```rust
pub enum Ready {} // No variants, cannot be initialized
```
<v-click>

- Make your type generic over its state using `std::marker::PhantomData`
- Implement methods only for relevant states
- Methods that update state take owned `self` and return instance with new state

*👻 `PhantomData<T>` makes types act like they own a `T`, and takes no space*
</v-click>
---
layout: three-slots
---

# Typestate: example

::left::

```rust
pub enum Idle {} // Nothing to do
pub enum ItemSelected {} // Item was selected
pub enum MoneyInserted {} // Money was inserted

pub struct CoffeeMachine<S> {
    _state: PhantomData<S>,
}
impl<CS> CoffeeMachine<CS> {
    /// Just update the state
    fn into_state<NS>(self) -> CoffeeMachine<NS> {
        CoffeeMachine {
            _state: PhantomData,
        }
    }
}
impl CoffeeMachine<Idle> {
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }
}
```

::right::
<div style="padding-left:10px; padding-top: 0;">

```rust
impl CoffeeMachine<Idle> {
    fn select_item(self, item: usize) -> CoffeeMachine<ItemSelected> {
        println!("Selected item {item}");
        self.into_state()
    }
}

impl CoffeeMachine<ItemSelected> {
    fn insert_money(self) -> CoffeeMachine<MoneyInserted> {
        println!("Money inserted!");
        self.into_state()
    }
}

impl CoffeeMachine<MoneyInserted> {
    fn make_beverage(self) -> CoffeeMachine<Idle> {
        println!("There you go!");
        self.into_state()
    }
}
```
</div>

---
layout: default
---

# Typestate: when to use

- If your problem is like a state machine
- Ensure *at compile time* that no invalid operation is done

---
layout: cover
---

# 4. The Strategy pattern
Select behavior dynamically

---
layout: default
---

# Strategy: introduction

- Turn set of behaviors into objects
- Make them interchangeble inside context
- Execute strategy depending on input

*Trait objects work well here!*

---
layout: two-cols
---

# Strategy: example

```rust

trait PaymentStrategy {
    fn pay(&self);
}

struct CashPayment;
impl PaymentStrategy for CashPayment {
    fn pay(&self) {
        println!("🪙💸");
    }
}

struct CardPayment;
impl PaymentStrategy for CardPayment {
    fn pay(&self) {
        println!("💳");
    }
}
```
::right::

<div style="padding-left:10px; padding-top: 50px;">

```rust

fn main() {
    let method: &str 
        = todo!("Read input");
    let strategy: &dyn PaymentStrategy 
        = match method {
        "card" => &CardPayment,
        "cash" => &CashPayment,
        _ => panic!("Oh no!"),
    };
    strategy.pay();
}
```

</div> 

---
layout: default
---

# Strategy: when to use

- Switch algorithms based on some run-time parameter (input, config, ...)

---
layout: cover
---

# Anti-patterns
What *not* to do

---
layout: cover
---

# Deref polymorphism

A common pitfall you'll want to avoid

---
layout: two-cols
---

# Deref polymorphism: Example

```rust
use std::ops::Deref;

struct Animal {
    name: String,
}

impl Animal {
    fn walk(&self) {
        println!("Tippy tap")
    }
    fn eat(&self) {
        println!("Om nom")
    }
    fn say_name(&self) {
        // Animals generally can't speak
        println!("...")
    }
}
```
::right::
```rust
struct Dog {
    animal: Animal
}
impl Dog {
    fn eat(&self) {
        println!("Munch munch");
    }
    fn bark(&self) {
        println!("Woof woof!");
    }
}
impl Deref for Dog {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}
fn main (){ 
    let dog: Dog = todo!("Instantiate Dog");
    dog.bark();
    dog.walk();
    dog.eat();
    dog.say_name();
}
```

---
layout: default
---

# The output

```txt
Woof woof!
Tippy tap
Munch munch
...
```

*Even overloading works!*

---
layout: default
---

# Why is it bad?

- This is no 'real' inheritance: `Dog` is no subtype of `Animal`
- Traits implemented on `Animal` are not implemented on `Dog` automatically
- `Deref` and `DerefMut` are intended 'pointer-to-`T`' to `T` conversions
- Deref coercion by `.` 'converts' `self` from `Dog` to `Animal`
- Rust favours explicit conversions for easier reasoning about code

*It will only add confusion: for OOP programmers it's incomplete, for Rust programmers it is unidiomatic*

## ⚠️ Don't do OOP in Rust!

---
layout: default
---

# What to do instead?

- *Move away from OOP constructs*
- Compose your structs
- Use facade methods
- Use `AsRef` and `AsMut` for explicit conversion

---
layout: default
---

# More anti-patterns

- Forcing dynamic dispatch in libraries
- `clone()` _to satisfy the borrow checker_
- `unwrap()` or `expect()` _to handle conditions that are recoverable or not impossible_
---
layout: cover
---
# Testing your crate

<!--
Next up: testing your crate. In bigger projects, automatic testing is key if you want to keep bugs away. In this section we will discuss some Rust functionalities that will help you test your application.
-->

---
layout: default
---

# Testing methods

- Testing for correctness
  - Unit tests
  - Integration tests
- Testing for performance
  - Benchmarks

<!--
Automatic testing can help you verify the correctness of your code, as well as test performance. 
- A common of testing correctness are by setting up unit tests, which test a small piece of functionality, a unit.
- If you want to test the correctness of interaction between those units, you can set up integration test.
- However, if you want to test performance, you can use benchmarking.
Let's go over how Rust supports these various testing methods.
-->

---
layout: default
---

# Unit tests

- Tests a single function or method
- Live in child module
- Can test private code

To run:

```bash
$ cargo test
[...]
running 2 tests
test tests::test_swap_items ... ok
test tests::test_swap_oob - should panic ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
[..]
```

*Rust compiles your test code into binary using a **test harness** that itself has a CLI*:


```bash
# Don't capture stdout while running tests
$ cargo test -- --nocapture
```

<!--
- Unit tests are great for testing behavior of a single function or method.
- In Rust, they live in child modules, allowing them to access private functionality
- Once set up, a `cargo test` is sufficient to build and run the tests
-->

---
layout: full
---

```rust {all|1-6|8-11,28|13-19|18|21-27}
/// Swaps two values at the `first` and `second` indices of the slice
fn slice_swap_items(slice: &mut [u32], first: usize, second: usize) {
    let tmp = slice[second];
    slice[second] = slice[first];
    slice[first] = tmp;
}

/// This module is only compiled in `test` configuration
#[cfg(test)]
mod tests {
    use crate::slice_swap_items;

    // Mark function as test
    #[test] 
    fn test_swap_items() {
        let mut array = [0, 1, 2, 3, 4, 5];
        slice_swap_items(&mut array[..], 1, 4);
        assert_eq!(array, [0, 4, 2, 3, 1, 5]);
    }

    #[test]
    // This should panic
    #[should_panic] 
    fn test_swap_oob() {
        let mut array = [0, 1, 2, 3, 4, 5];
        slice_swap_items(&mut array[..], 1, 6);
    }
}
```

<!--
Here's an example of a function being tested.
-`slice_swap_items` takes a mutable slice, as well as two indices, and swaps the items at those indices.
- Below, we've defined a module called `tests`, which is decorated with the `#[cfg(test)]` attribute. This attribute makes sure the module is only compiled when running tests.
- Inside the `tests` module, we've defined two tests and imported the `slice_swap_items` function from the parent module. The first test, `test_swap_items`, sets up a slice, passes it to `slice_swap_items` along with two indices.
- `test_swap_items` uses the `assert_eq!` macro to compare the affected array with an expected array. This `assert_eq!` macro panics on inequality, making the test fail if the outcome is not as expected.
- The second test, `test_swap_oob` is decorated with the `#[should_panic]` macro, meaning this test should only pass if it panics.

Q: Why should `test_swap_oob` panic?
-->

---
layout: default
---

# Integration tests

- Tests crate public API
- Run with `cargo test`
- Defined in `tests` folder:

```bash {all|14-15}
$ tree
.
├── Cargo.toml
├── examples
│   └── my_example.rs
├── src
│   ├── another_mod
│   │   └── mod.rs
│   ├── bin
│   │   └── my_app.rs
│   ├── lib.rs
│   ├── main.rs
│   └── some_mod.rs
└── tests
    └── integration_test.rs
```

<!--
To test your application from the outside, you can set up integration tests. These integration tests test your crates public interface and are also executed by running `cargo test`.
- They are defined in a separate folder, called `tests`
-->

---

# Tests in your documentation
You can even use examples in your documentation as tests

```rust {all|5-10|6}
/// Calculates fibonacci number n
///
/// # Examples
///
/// ```
/// # use example::fib;
/// assert_eq!(fib(2), 1);
/// assert_eq!(fib(5), 5);
/// assert_eq!(fib(55), 55);
/// ```
pub fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```

```bash
cargo test --doc
```

<!--
- Note that doctests are executed as if they are in another crate
- Lines with a hash (#) in front of them are not outputted in the generated
  documentation
- Don't try and write all your tests in doc form, only use them if you really
  want to provide an example
-->

---
layout: default
---

# Benchmarks

- Test *performance* of code (vs. correctness)
- Runs a tests many times, yield average execution time

*Good benchmarking is **Hard***

- Beware of optimizations
- Beware of initialization overhead
- Be sure your benchmark is representative

## *More in exercises*

<!--
Lastly, we'll briefly look at benchmarks, which test code performance instead of correctness. Basically, a test is run many, many times, and statistics about the execution time are gathered and reported.
- Note that good benchmarking is hard. You have to make sure tested parts of your code are not optimized away when they shouldn't be. Also, be aware of overhead. But most of all: make sure you benchmark is representative depending on the intended use of your code.
- We'll go a bit deeper into benchmarking in the exercises
-->
---

# To do

Issue: [tweedegolf/teach-rs#70](https://github.com/tweedegolf/teach-rs/issues/70)
---

# To do

Issue: [tweedegolf/teach-rs#69](https://github.com/tweedegolf/teach-rs/issues/69)


---

# Summary
