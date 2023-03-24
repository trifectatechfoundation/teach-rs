---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - E: Async and Rust for Web"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: 'Rust - E: Async and Rust for Web'
---
# Rust programming
Module E: Async and Rust for Web
<!-- Start with welcome, students entering -->
<!-- TODO add subject code -->

---
layout: default
---
# Last time...
- Trait objects: dynamic dispatch
- Design patterns
- Anti-patterns

*Any questions?*

<!--
- Recap on content from last time that current subject builds on
- Keep it short. Any longer explanations can be deferred to tutorial
-->

---
layout: default
---
# In this module
- Introduction to concurrency
    - Why and when use it
    - Async vs threads
    - The Rust async ecosystem
    - Compatibility
- The `Future` type
    - What's a future?
        - More and more complex versions up until the real signature
    - Futures are lazy
    - Build your own future
- `async fn`s and `async` blocks
    - `async fn`s are really functions returning an `async` block
    - How `async` and `await` are desugared into state machines
- How futures are run with an executor
    - naive polling
    - wakers
    - Pin (self-referential)
- Trade offs when using `async`
- Intro to `futures` and `tokio`
- Introduction to Rocket with Postgres


---
layout: default
---
# Learning objectives
- Understand the mechanics of async/await from a high-level point of view
- Understand the reason of Rust's implementation of async
- Work with Futures using the Tokio

<!-- List this module's learning objectives -->

---
layout: cover
---
#  Module E
Async and Rust for Web
<!-- Start lecture content here -->

---
layout: default
---
# Content overview
- FFI types
- Types from std
---
layout: default
---
# Concurrency vs. Parallelism

| **Concurrency**    | **Parallelism**         |
| ------------------ | ----------------------- |
| Interleaves work   | Parallelizes work       |
| 1 or more cores    | 2 or more cores         |
| Waiting for events | Waiting for computation |
| <img src="https://tienda.bricogeek.com/6417-thickbox_default/sparkfun-thing-plus-esp32-wroom.jpg" class="h-40 center" /> | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d3/IBM_Blue_Gene_P_supercomputer.jpg/1920px-IBM_Blue_Gene_P_supercomputer.jpg" class="h-40 center" /> |

Today, we're focusing on concurrency: _asynchronous programming_

---
layout: default
---

# What's async?

- Concurrent programming model
- Very suitable for running large number of tasks
- Look and feel* of synchronous code through `async`/`await` syntax

*Well, not quite. We'll go into that

---
layout: default
---

# Async vs OS Threads

|                      | <span style="color: red">**Async**</span> | <span style="color: blue">**OS Threads**</span> |
| -------------------- | ----------------------------------------- | ----------------------------------------------- |
| Spawning & switching | Cheap                                     | Expensive                                       |
| Blocking is ok       | No                                        | Yes                                             |
| Usage                | I/O bound tasks (web servers)             | CPU-bound tasks (Number crunching)              |
| Reuse sync code      | No                                        | Yes                                             |

[What Color is Your Function? ](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/)

---
layout: default
---

# Async in Rust

- Revolve around `Future` trait (~like JS Promise)
- `Future`s are inert
- `async` is zero-cost
- No built-in runtime
- Single- or multithreaded
- Can be mixed with other concurrency models


---
layout: default
---

# Example: TODO

TODO simple showcase example of async vs multithreading

---
layout: default
---

# State of the `async` art
What you can expect doing `async` Rust

- Blazingly fast applications
- More interaction with advanced language features
- Compatibility issues (re: colored functions)
- Faster evolving ecosystem

*Work in progress*

---
layout: default
---

# Support of `async`

- Fundamental types and traits are in `std`
- `async`/`await` are native to the language
- Utilities/extensions in `futures` crate
- Async runtimes are third party

Example runtimes: `async-std`, `tokio`, `smol`




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