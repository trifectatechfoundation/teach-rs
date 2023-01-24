# Course outline

This course is divided into separate modules, detailed below. Module A1-A4 are introductory, and contain what's needed to learn about the contents of the other modules. Therefore, you're advised to teach modules A1-A4 at the start of the course, but you can pick and mix the other modules.

## Learning objectives
At the end of the course, the student is able to:
- Describe the problems Rust aims to solve, why, and how (A1)
- Identify the design goals of Rust (A1)
- Decide whether or not Rust is a good fit for a project given requirements (A1)
- Describe how Rust impacts professional contexts (A1)
- Solve ownership errors using knowledge about the borrow checker model (A1, A3)
- Use various tools to improve software quality and programming experience (A2)
- Find their way around various Rust documentation sources (docs.rs, doc.rust-lang.org) and crate (dependency) indices (lib.rs) (A2)
- Set up custom Rust application and library projects using Rusts build system and package manager (A2, B)
- Implement small to medium-sized command-line applications using Rust and contribute to existing Rust projects (B)
- Make informed choices on whether or not to use dependencies to implement functionality, and make a choice based on requirements (B)
- Implement concurrent applications using threads and async/await (C)
- Use various standard patterns to design Rust applications (D)
- Implement web servers using Rust with several dependencies and frameworks (E)
- Identify the rules needed to keep in mind when implementing features that cannot be checked by the Rust compiler by using `unsafe` (F)
- Implement applications/libraries that interoperate with applications/libraries written in C (G)

## Course modules

### 0 - Course introduction
- Goals
- Structure
- Schedule
- Project introduction
- Tools
- Contact info

### A - Introduction to Rust
#### A1 - Language basics
- **'Why' of Rust language**
    - Problems Rust intends to solve
    - The fields it operates in
    - Rust design goals
    - Why Rust is considered secure
    - How learning Rust impacts your career
- **When to use Rust**
    - Where Rust really shines
    - Where Rust maturity lacks
    - What Rust wasn't designed for
- **Introduction to ownership**
    - Clones vs copies
    - Move semantics
    - Value ownership
    - Types of references
    - Borrowing rules
- **basic syntax and operators**
    - Types: primitives/struct/enum/union/slice
    - Control flow
    - Scopes, blocks, statics
    - Expressions
    - Functions
    - Pattern matching
    - Loops
    - ...
- **Structure of a Rust application**
    - imports
    - main function
    - modules
- **Conversions**
    - casting/`as` and pitfalls
    -`.into()`, `.try_into()` `T::from()`, `T::try_from()`, but not yet the traits they originate from
- **Panicking: explicit/unwrap/overflow**
    - What happens on panic
    - `no_panic`
    - When panicking is OK, and when it's not

**Exercises**
*TBD*


#### A2 - Ecosystem and tools
- **Cargo**
    - configuration
    - dependencies
    - cross-compilation
    - rustup
    - rustfmt
- **Build profiles**
    - debug vs. release
    - Opt-levels
    - LTO
    - ...
- **Tour through crate index and API docs**
    - Docs.rs
    - Lib.rs (unofficial)
    - Crates.io
- **Widely used tools**
    - debug
    - test 
    - security
    - bench (Criterion)
- **Rust versions**
    - Release cycle
    - stable vs beta vs nightly
    - Unstable features
    - editions
- **More resources:**
    - TRPL
    - cheats.rs
    - reference
    - rustonomicon

**Exercises**
*TBD*


#### A3 - Advanced Syntax, Ownership, References
- **Advanced syntax**
    - Types: /String/Vec/Box/Option/Result
    - Impl blocks
    - Coercion
    - Closures
- **Pointers vs references, reference meta**
- **Copy, clone, moves**
- **Ownership, borrowing, lifetimes**
    - NLL: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes
- **Lifetime annotation, elision**
    - Why needed
    - Syntax

**Exercises**
*TBD*

#### A4 - Traits and generics
- **Traits**
- **Commonly used traits from std**
    - Into/From/TryFrom/TryInto (referring to A1)
    - Copy/Clone
    - Debug/Display
    - Iterator/IntoIter/FromIter
    - FromString
    - AsRef/AsMut
    - Deref/DerefMut
    - PartialEq/Eq/Add/Mul/Div/Sub/PartialOrd/Ord
    - Drop
- **Generics, trait objects, object safety, const generics**
    - Static vs dynamic dispatch
    - when to use generics vs trait objects
- **Orphan rule**


**Exercises**
*TBD*


### B - Application programming
This module is about learning to write actual Rust applications. At the end of this module, the student is able to:
  - Set up a Rust application and library crate
  - Add dependencies to a crate
  - Use the Rust module system to divide a program into logical parts
  - Write applications that follow the Rust API guidelines
  - Set up Rust tests and benchmarks
  - Work with some commonly used crates for logging, argument parsing, deserialization, and testing (Exercise)
  - Use various methods to improve compile time (Exercise)

**Topics**
- **Setting up a Rust crate, bin vs lib**
  - Commands
  - Cargo.toml entries
- **Structure of a Rust project**
  - Module system
  - Unit tests/integration tests/benchmarks
  - Examples
  - Multiple binaries
  - Cargo workspaces
- **[Rust API guidelines](https://rust-lang.github.io/api-guidelines/about.html)**
    - (doc) Comments
    - Casing conventions
- **Widely used crates: logging/argparse/(de)serialization/testing**

**Exercises**
- Set up a custom project that:
  - Acts both as a library and a binary crate
  - Contains some examples
  - Is divided up into modules
  - Has unit tests, integration tests, and benchmarks
  - Uses dependencies for argument parsing, logging, serialization, and benchmarking
  - Has doc comments according to the Rust API guidelines
- Generate documentation with `cargo doc`
- Find and try various methods to improve the project build time, comparing the improvements with `cargo build --timings`


### C - Multitasking
- **Fearless concurrency**
- **Rayon**
- **Atomic types**
- **Multithreading: Send/Sync/Channel....**
- **How the borrow checker helps us**
- **Smart pointers, std::sync**
- **Helpful types**
    - Cell/RefCell/Rc/Mutex

**Exercises**
*TBD*

### D - Idiomatic Rust patterns
- **Newtype**
- **Typestate**
- **Builder**
- **Composition over inheritance**
- **Error handling: enum/anyhow/thiserror**
- **Anti patterns**
- **Cool patterns from std**
    - https://www.reddit.com/r/rust/comments/x1mo16/is_there_any_part_of_the_standard_library_that/

**Exercises**
*TBD*

### E - Rust for web
- **Rust web crates**
    - Hyper
    - Rocket
    - ORM
    - ...
- **std::net**

**Exercises**
*TBD*

### F - Safe Unsafe Rust
- **Why safe vs unsafe**
- **Undefined behavior**
- **Unsafe keyword**
- **Added functionality**
- **Abstract machine**
- **Optimization**
- **MaybeUninit**
- **Drop check, ManuallyDrop**
- **Type memory layout**
- **MIRI**
- [Unsafe code guidelines](https://rust-lang.github.io/unsafe-code-guidelines/introduction.html)

**Exercises**
*TBD*

### G - FFI and Dynamic modules
- **FFI in Rust, extern "C"**
- **`sys`-crates**
- **std::ffi**
- **`catch_unwind`**
- **bindgen**
- **Cxx/PyO3**
- **Panics and catch_unwind**
- **`libloading` crate**
- **WASI with wasmtime**

**Exercises**
*TBD*


### P Final project

#### Ideas
- **Scientific programming**
    - nalgebra
    - polars
- **Game development: ggez, bevy**
    - https://www.arewegameyet.com/
- **GUI application**
    - https://www.areweguiyet.com
- **Doubly linked list**
- **Embedded programming**
    - Arduino with https://book.avr-rust.com/
    - Raspberry pi pico with https://crates.io/crates/rp-pico
    - Raspberry pi with https://crates.io/crates/rppal
- **OS dev**
    - https://os.phil-opp.com/
    - https://github.com/gjf2a/pluggable_interrupt_os
- **Some audio filtering and streaming software**
- **Contribute to an open source project**
- **SIMD**
- **Simple programming language**
    - https://craftinginterpreters.com/
    - Brainfuck interpreter/compiler

#### Structure
- Work in teams of 2
- Hand in proposal in week 7
- Write small report (2-3 pages), to be handed in in week 12
    - Introduction
    - Requirements
    - Design diagram
    - Design choices
    - Dependencies
    - Evaluation
- Present project in about 5 minutes in week 12


## Lecture format (90 minutes)
### Rationale
During lectures, new content is provided to students. The idea is to keep engagement high using interaction an by extensively activating prior knowledge. We take some time for questions and discussion during the lecture and are aware of the facts that many concepts are outright confusing to beginners. During discussion, we encourage students to answer questions of fellow students. However, we actively make sure that discussions don't divert from the subject. 

To activate prior knowledge, we start each lecture with a recap on the subject of the last lecture with quiz questions. Once that's done, we relate content of the current subject with content of prior lectures where possible. 

We also relate content to other programming languages, taking into account the intermediate C++ knowledge students have. This can be done by asking questions such as 'How would you solve this problem in your favourite programming language'? However, as relating to

other languages may take a lot of time, we only do this sparingly, and with more confusing subjects.

Apart from that, we focus on the 'why' of each concept, as it helps students to better internalize the contents, and it allows them to creatively apply the gained knowledge in doing exercises and in the final project.

During the lecture, we actively measure how well the content is being picked up by doing rounds of online multiple-choice quiz questions. The measurement feedback dictates the pace in which the content is gone over. Therefore, we provide enough theory for a high pace, but take into account that we may need to go to a lower pace and this skip some contents or details.

The lecture slides are available online, and contain links to the [Rust playground](https://play.rust-lang.org/) for each code example. The playground allows students to interact with the code, providing a means of further internalizing the examples. The slides also contain further reading resource links on the lecture subject.


### Schedule
 - (2m) Start with welcome, students entering
 - (10m) Recap on content from last time that current subject builds on
     - (2m) Recap overview 
     - (3m) Short round of questions
     - (5m) 1 set of quiz questions
 - (3m) Introduce lecture subject and learning objectives
 - (3m) Round up: What do you (think you) know about today's subject?
 - (2m) Lecture content overview
 - (60m) Lecture content
     - (15m) Quiz questions and discussion
     - (45m) Subject theory
     - (5m) Questions
 - (5m) Quick recap, practical announcements

## Tutorial format (90m)
### Rationale
In the tutorials, the focus lies on applying the content of the prior lecture by doing exercises. During these tutorials, exercises from the last tutorial are reviewed and new exercises are introduced. Moreover, in the tutorials, students are encouraged to ask questions on the lecture content. Students are expected to work on the exercises briefly introduced at the beginning of the tutorial in small groups (2 or 3 students). These groups are formed during the first tutorial. The tutor will be available for questions and tips on the exercises during the tutorial. Students can hand in their results for feedback during the tutorial.

### Schedule
- (2m) Start with welcome, students entering
- (23m) Review last weeks exercises
    - (18m) go over model exercise answers
    - (5m) questions
- (5m) Introduce new exercises
- (60m) Work on exercises in small groups


## Course schedule
| Week | Date | Module | Notes                                     |
| ---- | ---- | ------ | ----------------------------------------- |
| 1    |      | 0, A1  | Course intro                              |
| 2    |      | A2, A3 |                                           |
| 3    |      | A4     |                                           |
| 4    |      | B      |                                           |
| 5    |      | C      |                                           |
| 6    |      | D      | Project proposal reminder                 |
| 7    |      | E      | Deadline project proposal                 |
| 8    |      | F      | Project proposal resubmission             |
| 9    |      | G      | Start final project                       |
| 10   |      | P      |                                           |
| 11   |      | P      |                                           |
| 12   |      | P      | Final project submission and presentation |
