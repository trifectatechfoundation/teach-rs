Teacher's companion to teach-rs
===============================

If you have decided to try teach-rs for your students, you will probably run into two problems:

1. As an academic, you may feel your own practical knowledge of Rust is lacking.

2. You will have to make a selection of subjects to fit practical constraints.

So what parts of teach-rs should you invest time in to teach to your students? And how much time is required?

We assume you have a clear idea of your learning outcomes, and your target audience. Teach-rs can be used for first-year
students at university, for master's students, or even for an internal training for senior engineers at your software company, but obviously different
groups would require a different approach!

Teach-rs is a modular course
============================
We have defined particular *tracks*, which consists of selections of modules that go
well together given a certain learning outcome and target audience, for example teach-rs focussed on Web programming or teach-rs
focussed on Embedded Devices; you can see the full list of tracks [here](./README.md#pre-defined-tracks):

Finer-grained modularity
------------------------
If you want finer-grained control over content selection, we have structured every module into a few *topics*. A topics is defined by
a set of slides and recommended exercises. You can construct your own modules by selectiong topics. We have defined dependencies between
topics; for example, if you pick the `basic-syntax` topic you should also select the `why-rust` topic. These dependencies ensure that
you should still end up with a coherent course.

If you take this route, however, you have to take more responsibility that the study load remains balanced, as (unlike with modules), 
*topics* don't have a fixed study time associated with them. For example (again), the `why-rust` topic will require less time (and has no
partical exercises attached to it) than the `basic-syntax` topic. Since teach-rs is in active development, we cannot give
time estimates per topic and are focussing more on balancing the study load for the full course and the pre-defined *tracks*.

Overview of modules and topics
------------------------------
General modules of the Rust course can be divided into "common" and "specialized" modules. The
common ones will be useful for every track (for example, "Language Basics") whereas others
can be viewed as optional (for example, "Rust for Web").

Module 0 (introduction) is recommended in full for every course, since it outlines the motivation
for learning Rust, and broadly introduces its features. Module A contains all topics related to language features.

Reference material
==================
Several online resources exists that can provide valuable background material for you (or your students).

- [The Rust Programming Language](https://doc.rust-lang.org/book/index.html), online book, also available [as paperback](https://nostarch.com/rust-programming-language-2nd-edition)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/), online reference with idiomatic examples of Rust code
- [Rust for Rustaceans](https://nostarch.com/rust-rustaceans), book that assumes some prior knowledge and dives into more advanced topics
- [Learning Rust with Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/), focussing on Rust's ownership and borrowing rules
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/), a collection of idiomatic Rust patterns (and anti-patterns)
- [The Rust Reference](https://doc.rust-lang.org/reference/index.html), a reference suitable for finding explanations for finer points of syntax and semantics.
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/), a reference that is specific to low-level aspects, application binary interfaces and `unsafe` code.

Exercise solutions
------------------
Teach-rs is provided without answers to exercises. If you have need of those, please [contact us](mailto:henk@tweedegolf.com).
