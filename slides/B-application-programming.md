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
Module B: Application programming (pilot)
<!-- Start with welcome, students entering -->


---
layout: three-slots
---

## Who am i?
::left::
## Henk Oordt
- Embedded software engineer
- Rust for Embedded/IoT
- Rust-lang trainer

::right::
![Photo Henk](https://tweedegolf.nl/images/hd_sept_21_small.jpg)
<style>
h2 {
  text-align: center;
  margin-bottom: 50px;
}

ul {
  width: 70%;
  display: block;
  margin: auto;
}

img {
  display: block;
  margin: auto;
}

</style>
<!--
Optionally quickly introduce yourself, add photo
-->

---
layout: default
---
# Today
- Introduce Rust application programming
- Work on exercises
- Have fun!

*Any questions?*

---
layout: quote
---

# Please interrupt me if you have any questions or remarks
### 15 minute break every 45 minutes

---
layout: default
---
# Introduce yourself

- Name
- Your experience with Rust
- Favorite Rust feature

---
layout: default
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
- Set up your own Rust application and library
- Divide your code into logical parts with modules
- Create a nice API
- Test and benchmark your code
- Use common crates (tutorial)

---
layout: section
---
# Mindmap

What do you know already about this subject?

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
- Project structure
- API guidelines
- Testing and benchmarking
<!-- Give an overview of the subjects covered in this lecture -->


---
layout: section
---
# Rust Project structure

---
layout: default
---

# Terminology

- **Crate:** A package containing Rust source code. Library or binary.
- **Module:** Logical part of crate, containing items.
- **Workspace:** Set of related crates.

<!--
Let's get started with some terminology involving Rust application structure.

- A Module is a container in which items are defined, such as types, traits and functions.
- In Rust, packages of source code are called 'crates'. They can define a library, or binary, or even both. Crates contain modules
- Then there are workspaces, bundles of related crates that may or may not share dependencies. For example, you may split a large application into several crates. Those crates can still be loosely connected using a workspace. This helps avoiding issues with dependency versions and may improve build times.

-->

---
layout: default
---

# Setting up a crate

Setting up a new crate is easy:

```bash {all|1-2|3-7}
$ cd /path/to/your/projects
$ cargo new my-first-app --bin
$ tree my-first-app
.
├── Cargo.toml
└── src
    └── main.rs
```

*Pass `--lib` instead of `--bin` to create a library*

<!--
- The way you set up a crate is by running `cargo-new`. You can pass it a name, and indicate whether you want to set up a library or an application.
- This  will generate a `Cargo.toml` file as well as a demo  source file.
- To create a library crate, pass `--lib` instead of `--bin`

-->
---
layout: default
---
# Adding a crate as dependency

To add a dependency from crates.io:
```bash {all|1|3-13|11-13}
$ cargo add tracing tracing-subscriber
[...]
$ cat Cargo.toml
[package]
name = "my-first-app"
version = "0.1.0"
edition = "2021"

# -snip-

[dependencies]
tracing = "0.1.37"
tracing-subscriber = "0.3.16"
```

<!--
- In Rust, adding crates as dependencies is made easy using Cargo. That way, you can import all kinds of libraries that help you with developing your application.
- By default, dependencies are pulled from crates.io, Rusts global crate registry.
- In order to add a dependency to your project, you can use `cargo add`. Pass it the names of the dependencies you'd like to use.
- `cargo add` will add the dependencies to your `Cargo.toml` file, making it available for your code to use.
-->

---
layout: default
---

# Using dependencies

Dependencies from Cargo.toml can be:
  - imported with a `use`
  - qualified directly using path separator `::`

```rust {all|3-4,16-17|7-10}
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

<!--
 - Other sources Cargo can pull dependencies from are your local file system or Git.
 - To specify a dependency that lives on your file system, use the `path` key.
 - Dependencies from Git can be included using the `git` key, with which you specify a Git URL. Use `rev`, `branch` or `tag` to further specify a version.
-->

---
layout: default
---

# Modules

- Logical part of a crate
- Public or private visibility
- Defined in blocks or files

*Mod structure `!=` file structure*

<!--
- Rust crates are split up into several modules, which help you keep your code organized.
- Modules may have public or private visibility.
- These modules can defined in blocks, or in separate files. Note that the module file structure may not correspond to the module structure!

Let's take a look at how defining modules work.

-->

---
layout: default
---
# Module block
```rust {all|1-3,20|4-6,13|8-12,18|15-19}
// Public module
// Accessible from outside
pub mod my_pub_mod {
    // Private module
    // Only accessible from parent module
    mod private_mod {

        // Public struct
        // Accessible wherever `private_mod` is
        pub struct PubStruct {
            field: u32,
        }
    }

    // Private struct
    // Only accessible from current and child modules
    struct PrivStruct {
        field: private_mod::PubStruct,
    }
}
```

<!--
Here's an example containing some modules defined with module blocks, using the `mod` keyword and braces.
- The outer module, `my_pub_mod`, can be accessed from ancestor modules. It is decorated with the `pub` keyword.
- It contains a private mod: `my_private_mod`. Note the absence of the `pub` keyword. This module is only visible for the module it is defined in.
- The struct `PubStruct` that is defined in `my_private_mod` is visible from wherever `my_private_mod` is, and can thus be referred to from `my_pub_mod`
- `PrivStruct` is only accessible from current and child modules.
-->

---
layout: default
---

# Module files

Content specified in
- Either `some_mod.rs`
- Or `another_mod/mod.rs`

```bash {all|7|3-4}
$ tree src
.
├── another_mod
│   └── mod.rs
├── lib.rs
├── main.rs
└── some_mod.rs
```

<!--
Apart from blocks, modules can be defined in separate files.
- You can either create a file `some_mod.rs` directly,
- Or keep related modules together in a separate directory. Of the modules defined in the directory, `mod.rs` is the parent.
-->

---
layout: default
---

# Module files

Mod structure defined in other modules:


**lib.rs**
```rust {all|1-2|3-4|5-6}
// Points to ./some_mod.rs
mod some_mod;
// Points to ./another_mod/mod.rs
mod another_mod;
// Imports an item defined in ./another_mod/mod.rs
use another_mod::Item;
```

```bash
$ tree src
.
├── another_mod
│   └── mod.rs
├── lib.rs
├── main.rs
└── some_mod.rs
```

<!--
And here's how you declare the module structure.
-->

---
layout: default
---

# Module files vs blocks

- Use blocks for small (private) modules
- Use files for larger (public) modules
- Group related module files in folder

*If your file gets unwieldy, move code to new module file*

<!--
How to decide whether to declare modules in files or blocks? That again depends on context. Make your code readable and your intent clear.
-->

---
layout: default
---

# Binaries and examples

- Use multiple binaries if you are creating
  - multiple similar executables 
  - that share code
- Create examples to show users how to use your library

<!--
- A crate can contain multiple binaries. This is useful if you're creating multiple applications that share significant parts of code.
- If you're writing a library, adding a couple of examples helps your users get started. In fact, many libraries are accompanied with examples defined in their Git repositories.
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

# Use semantic typing

Make the type system work for you!

```rust {all|1-3,14,15,17|5-12,14,16,17}
fn enable_led(enabled: bool) {
    todo!("Enable it")
}

enum LedState {
    Enabled,
    Disabled
}

fn set_led_state(state: LedState) {
    todo!("Enable it")
}

fn do_stuff_with_led() {
    enable_led(true);
    set_led_state(LedState::Enabled)
}
```

<!--
Rusts type system is awesome. Use it to you advantage by embedding semantics into your types.
- As an example, the `enable_led` method takes a `bool`. The calling code does not express intent as much as it could.
- The `set_led_state`, however, takes a `LedState` variant, which clearly expresses what the developer was trying to do.
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

- Correctness
  - Unit tests
  - Integration tests
- Performance
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
layout: section
---

# Exercise time!

## &rarr; 101-rs.tweede.golf &larr;

---
layout: end
---
