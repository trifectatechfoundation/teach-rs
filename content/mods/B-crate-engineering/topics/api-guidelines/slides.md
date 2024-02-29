
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