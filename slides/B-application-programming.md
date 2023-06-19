---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: 'Rust - B: Application programming'
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: 'Rust - B: Application programming'
---

# Rust programming
Module B: Application programming
<!-- Start with welcome, students entering -->


---
layout: default
---
# Last time...
- Generic code
- Traits
- Trait generics & associated types
- Lifetime annotations

*Any questions?*

---
layout: cover
---

# In this module

Learn how to use Rust for writing high quality applications

<!--
- Introduce today's subject
- The module is about actually *using* Rust. You're not going to be productive writing real-world Rust applications
if you've only been introduced to the Rust syntax or have been implementing some common algorithms (although, of course, that does help). Starting with this module, we will put your Rust knowledge into practise.
-->

---
layout: default
---

# Learning objectives
<!-- List this module's learning objectives -->

- Work with `crate` dependencies
- Create your own crate with a nice API
- Test and benchmark your code

**During tutorial:**
- Divide your code into logical parts with modules
- Use common crates
- Set up your own Rust application and library

---
layout: cover
---
#  Module B
Application programming
<!-- Start lecture content here -->

---
layout: default
---

# Content overview
- Working with `crate`s
- API guidelines
- Testing and benchmarking
<!-- Give an overview of the subjects covered in this lecture -->

---
layout: section
---
# Creating Rust projects

---

# Cargo
Most daily usage of Rust will involve using cargo in one way or another.

Some of the more common tasks are:

* Creating new projects
* Managing dependencies
* Building projects
* Executing the resulting binaries
* Running tests and benchmarks
* Generating and viewing local documentation

---

# Cargo configuration
Cargo is managed through the `Cargo.toml` configuration file. Toml is an easy
to read configuration file fairly similar to ini files.

```toml
[package]
name = "example"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
```

<!--
- The package part of the configuration file is used to configure the package
- The dependencies section is used to add external dependencies
- The configuration file should look familiar to users of tools such as NPM,
  RubyGems or Composer.
- Note the capital C at the start of the cargo.toml file
-->

---

# Adding dependencies
You can add dependencies to `Cargo.toml` in multiple ways

<v-click>

## Add a line in cargo.toml

```toml
[package]
name = "example"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
itertools = "0.10"
```

</v-click>

<v-click>
<br>

## Use `cargo add`

```bash
cargo add itertools
```

</v-click>

<!--
- Note: cargo add is a relatively new addition to cargo, previously this used to
  require a plugin called cargo-edit
-->

---

# Dependencies? Crates! 📦
The crate is the compilation unit for Rust

* Binary crates: 
  * Result in a compiled binary program that you can execute.
  * Binaries have a `main` function as entrypoint of the program
* Library crates: 
  * define functionality that can be used by other crates. 
  * No specific `main` function

Each crate in Rust has a root file. For binary crates this typically is
`main.rs`, but for libraries this typically is `lib.rs`.


---
layout: default
---

# Using a `crate`

Crates included in `Cargo.toml` can be:
  - imported with a `use`
  - qualified directly using path separator `::`

```rust {all|3-4,15-17|7-10}
// Import an item from this crate, called `my_first_app`
use my_first_app::add;
// Import an item from the `tracing` dependency
use tracing::info;

fn main() {
    // Use qualified path
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let x = 4;
    let y = 6;

    // Use imported items
    let z = add(x, y);
    info!("Let me just add {x} and {y}: {z}")
}

```

<!--
- To refer to an item from a dependency, you'll either have to import it, or qualify it.
- Importing an item is done with the `use` keyword like on the top of this file. You can now use the imported item without further qualification
- Qualifying a path is done using the `::` path separator
- Which method of referring to an item depends on how readable it makes your code in a certain context.
-->

---
layout: default
---

# Other dependency sources

- Local
- Git

```bash {all|4|5}
$ cat Cargo.toml
# -snip-
[dependencies]
my_local_dependency = { path = "../path/to/my_local_dependency" }
my_git_dependency = { git = "<Git SSH or HTTPS url>", rev="<commit hash or tag>", branch = "<branch>" }
```

- Private crate registries are WIP

<!--
 - Other sources Cargo can pull dependencies from are your local file system or Git.
 - To specify a dependency that lives on your file system, use the `path` key.
 - Dependencies from Git can be included using the `git` key, with which you specify a Git URL. Use `rev`, `branch` or `tag` to further specify a version.
-->

---
layout: section
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
layout: section
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
layout: section
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
layout: section
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
- Run exaples with the `--example` option, specifying the binary
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
  let page = load_page("https://101-rs.tweede.golf");
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
layout: section
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
layout: default
---
# Summary
- Set up your own Rust application and library
  - Using `cargo new`
- Divide your code into logical parts with modules
  - Modules
  - Workspaces
- Create a nice API
  - Unsurprising, Flexible, Obvious
  - API guidelines
- Test and benchmark your code
  - Unit tests, integration tests, benchmarks


<!-- Very quickly go over the learning objectives and how they were covered -->

---
layout: default
---
# Tutorial time!
<!-- Use this slide to announce any organizational information -->

- Exercises A3 recap
- Exercises B in 101-rs.tweede.golf
- Live code on ex B1 and first part of B2

*Don't forget to `git pull`!*


---
layout: end
---
