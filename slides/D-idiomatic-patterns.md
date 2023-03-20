---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - D: Trait objects and Rust patterns"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: 'Rust - D: Trait objects and Rust patterns'
---
# Rust programming
Module D: Trait objects and Rust patterns
<!-- Start with welcome, students entering -->

---
layout: cover
---
# In this module

- Introducing dynamic dispatch
- Work with design pattern in Rust
- Common anti-patterns to avoid

---
layout: default
---
# Learning objectives
<!-- List this module's learning objectives -->
- Understand the difference between static and dynamic dispatch
- Be able to create trait objects
- Understand the concept of 'object safety'
- Apply various commonly used design patterns
- Know which anti-patterns to avoid

---
layout: cover
---
#  Module D
Idiomatic Rust Patterns
<!-- Start lecture content here -->

---
layout: default
---
# Content overview
- Trait objects and dynamic dispatch
- Rust design patterns
- Anti-patterns

---
layout: section
---
# Trait objects & dynamic dispatch

---
layout: default
---

# Trait... Object?
- We learned about traits in module A3
- We learned about generics and `monomorphization`

There's more to this story though...

---
layout: default
---

# Dynamic dispatch
*What if don't know the concrete type implementing the trait at compile time?*

```rust{all|1-8|10-12|14-23|17-20}
use std::io::Write;
use std::path::PathBuf;

struct FileLogger { log_path: PathBuf }
impl Write for FileLogger { /* - snip -*/}

struct StdOutLogger;
impl Write for StdOutLogger { /* - snip -*/}

fn log<L: Write>(entry: &str, logger: &mut L) {
    write!(logger, "{}", entry);
}

fn main() {
    let log_file: Option<PathBuf> = 
        todo!("read args");
    let mut logger = match log_file {
        Some(log_path) => FileLogger { log_path },
        Nome => StdOutLogger,
    };
    
    log("Hello, world!🦀", &mut logger);
}
```

---
layout: default
---
# Error!


```txt
error[E0308]: `match` arms have incompatible types
  --> src/main.rs:19:17
   |
17 |       let mut logger = match log_file {
   |  ______________________-
18 | |         Some(log_path) => FileLogger { log_path },
   | |                           ----------------------- this is found to be of type `FileLogger`
19 | |         Nome => StdOutLogger,
   | |                 ^^^^^^^^^^^^ expected struct `FileLogger`, found struct `StdOutLogger`
20 | |     };
   | |_____- `match` arms have incompatible types
```

*What's the type of `logger`?*

---
layout: default
---

# Heterogeneous collections
*What if we want to create collections of different types implementing the same trait?*

```rust{all|1-13|15-21}
trait Render {
    fn paint(&self);
}

struct Circle;
impl Render for Circle {
    fn paint(&self) { /* - snip - */ }
}

struct Rectangle;
impl Render for Rectangle {
    fn paint(&self) { /* - snip - */ }
}

fn main() {
    let mut shapes = Vec::new();
    let circle = Circle;
    shapes.push(circle);
    let rect = Rectangle;
    shapes.push(rect);
    shapes.iter().for_each(|shape| shape.paint());
}
```

---
layout: default
---

# Error again!
```txt
   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
  --> src/main.rs:20:17
   |
20 |     shapes.push(rect);
   |            ---- ^^^^ expected struct `Circle`, found struct `Rectangle`
   |            |
   |            arguments to this method are incorrect
   |
note: associated function defined here
  --> /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library/alloc/src/vec/mod.rs:1836:12

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to previous error
```

*What is the type of `shapes`?*

---
layout: default
---
# Trait objects to the rescue

- Opaque type that implements a set of traits
- Type description: `dyn T: !Sized` where `T` is a `trait`
- Like slices, Trait Objects always live behind pointers (`&dyn T`, `&mut dyn T`, `Box<dyn T>`, `...`)
- Concrete underlying types are erased from trait object

```rust{all|5-7}
fn main() {
    let log_file: Option<PathBuf> = 
        todo!("read args");
    // Create a trait object that implements `Write`
    let logger: &mut dyn Write = match log_file {
        Some(log_path) => &mut FileLogger { log_path },
        Nome => &mut StdOutLogger,
    };
}
```
---
layout: two-cols
---

# Layout of trait objects 

```rust
/// Same code as last slide
fn main() {
    let log_file: Option<PathBuf> = 
        todo!("read args");
    // Create a trait object that implements `Write`
    let logger: &mut dyn Write = match log_file {
        Some(log_path) => &mut FileLogger { log_path },
        Nome => &mut StdOutLogger,
    };

    log("Hello, world!🦀", &mut logger);
}
```
<v-click>

- *💸 Cost: pointer indirection via vtable &rarr; less performant*
- *💰 Benefit: no monomorphization &rarr; smaller binary & shorter compile time!*
</v-click>

::right::
<!-- TODO switch out this JPEG for an SVG that works both in dark and light theme -->
<img src="/images/D-trait-object-layout.jpg" style="margin-left:5%; margin-top: 50px; max-width: 100%; max-height: 90%;">


---
layout: default
---

# Fixing dynamic logger

- Trait objects `&dyn T`, `Box<dyn T>`, ... implement `T`!

```rust{all|9-12|1-2}
// We no longer require L be `Sized`, so to accept trait objects
fn log<L: Write + ?Sized>(entry: &str, logger: &mut L) {
    write!(logger, "{}", entry);
}

fn main() {
    let log_file: Option<PathBuf> = 
        todo!("read args");
    // Create a trait object that implements `Write`
    let logger: &mut dyn Write = match log_file {
        Some(log_path) => &mut FileLogger { log_path },
        Nome => &mut StdOutLogger,
    };

    log("Hello, world!🦀", logger);
}
```
And all is well!

---
layout: default
---

# Forcing dynamic dispatch

Sometimes you want to enforce API users (or colleagues) to use dynamic dispatch

```rust{all|1}
fn log(entry: &str, logger: &mut dyn Write) {
    write!(logger, "{}", entry);
}

fn main() {
    let log_file: Option<PathBuf> = 
        todo!("read args");
    // Create a trait object that implements `Write`
    let logger: &mut dyn Write = match log_file {
        Some(log_path) => &mut FileLogger { log_path },
        Nome => &mut StdOutLogger,
    };


    log("Hello, world!🦀", &mut logger);
}
```

---
layout: default
---

# Fixing the renderer

```rust
fn main() {
    let mut shapes = Vec::new();
    let circle = Circle;
    shapes.push(circle);
    let rect = Rectangle;
    shapes.push(rect);
    shapes.iter().for_each(|shape| shape.paint());
}
```
<v-click>
Becomes

```rust{all|2,3,5}
fn main() {
    let mut shapes: Vec<Box<dyn Render>> = Vec::new();
    let circle = Box::new(Circle);
    shapes.push(circle);
    let rect = Box::new(Rectangle);
    shapes.push(rect);
    shapes.iter().for_each(|shape| shape.paint());
}
```

All set!
</v-click>

---
layout: default
---

# Trait object limitations

- Pointer indirection cost
- Harder to debug
- Type erasure
- Not all traits work:

*Traits need to be 'Object Safe'*


---
layout: default
---

# Object safety

In order for a trait to be object safe, these conditions need to be met:

- If `trait T: Y`, then`Y` must be object safe
- trait `T` must not be `Sized`
- No associated constants allowed
- No associated types with generic allowed
- All associated functions must either be dispatchable from a trait object, or explicitly non-dispatchable

Details in [The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#object-safety). Read them!

---
layout: default
---

# So far...

- Trait objects allow for dynamic dispatch and heterogeneous 
- Trait objects introduce pointer indirection
- Traits need to be object safe to make trait objects out of them

---
layout: section
---

# Design patterns in Rust

---
layout: default
---
# Summary
<!-- Very quickly go over the learning objectives and how they were covered -->

---
layout: default
---
# Practicalities
<!-- Use this slide to announce any organizational information -->

---
layout: end
---
<!-- Below are example slides you can use -->
