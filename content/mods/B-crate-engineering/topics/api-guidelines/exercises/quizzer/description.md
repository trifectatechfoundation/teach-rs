In this exercise, you will create a Rust crate that adheres to the guidelines that were pointed out during the lecture. Additionally, you will add and use dependencies, create unit tests, and create some documentation. You can view this exercise as a stepping stone to the final project.

*This exercise should be done in groups of 2 people*

# #[modmod:exercise_ref].A Setting up ⭐
Create a new project using `cargo new --name quizzer`. Make sure it acts as both a binary and a library. That means there will be both a `src/lib.rs` and a `src/bin/quizzer/main.rs` file in your crate, where `quizzer` is the name of the binary:

```bash
$ tree
.
├── Cargo.toml
├── quiz.json
└── src
    ├── bin
    │   └── quizzer
    │       └── main.rs
    └── lib.rs

```

Add the following dependencies to your `Cargo.toml` file. Below items contain links to their page on lib.rs. Make sure you get a general idea of what these crates are for and how they can be used. Don't dive too deep just yet.
   -  [`anyhow` 1.0](https://lib.rs/crates/anyhow)
   -  [`clap` 4.0](https://lib.rs/crates/clap) Also, skim over <https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html>
   -  [`serde-json` 1.0](https://lib.rs/crates/serde_json)
   -  [`serde` 1.0](https://lib.rs/crates/serde)

Your `Cargo.toml` should look like this:

```toml
[package]
name = "quizzer"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0.66"
clap = { version = "4.0.18", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.87"
```

For `clap` and `serde`, the non-standard `derive` feature of each these crates is enabled. For `clap`, it allows us to derive the `Parser` trait, which greatly simplifies creating a CLI. The `derive` feaure from `serde` allows us to derive the `Serialize` and `Deserialize` traits on any struct we wish to serialize or deserialize using `serde` and its backends, in our case `serde_json`.

# #[modmod:exercise_ref].B Quizzer ⭐⭐⭐
This exercise is about both design and finding information. You'll have to figure out a model to represent your quiz questions, as well as a means to store them into a JSON file, and load them yourself. Also, you will have to find out how to parse the program arguments.

We will use the project we just set up to write a quiz game creator and player. You may add other dependencies as needed. It has the following functional requirements:
 - It runs as a command-line tool in your terminal.
 - It has two modes: question-entering mode and quiz mode. The mode is selected with a subcommand, passed as the first argument to the program.
   - Question-entering mode: Allows for entering multiple-choice quiz questions, with 4 possible answers each, exactly 1 of them being correct. The questions are stored on disk as a JSON file.
   - Quiz mode: Loads stored questions from the JSON file, presents the questions one-by-one to the player, reads and verifies the player input, and presents the score at the end of the game.
 - Errors are correctly handled, i.e. your application does not panic if it encounters any unexpected situation. Use `anywhow` and the question-mark (`?`) operator to make error-bubbling concise. You can read about the `?`-operator here: <https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator>
 - Logic concerning creating, storing, and loading quiz questions is defined in the library part of your crate.
 - Functionality regarding user input (arg parsing, reading from stdin) is defined in the application code, not in your library.
 - Logical units of your crate are divided up into modules.

Before you start coding, make sure you've listed all open questions and found answers to them. You're also encouraged to draw a simple diagram of the module structure of your application, annotating each module with its responsibilities.
