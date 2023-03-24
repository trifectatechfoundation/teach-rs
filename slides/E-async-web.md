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
<!-- Give an overview of the subjects covered in this lecture -->
<!-- Incorporate any breaks as well -->

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