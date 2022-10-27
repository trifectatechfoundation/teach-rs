# Module B - Application programming

## B.1 Serializing and deserializing of `String`s with `serde`
*This exercise is adapted from the [serde_lifetimes exercise](https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc) by Ferrous Systems*

Open `exercises/B1-my-serde-app/src/main.rs`. In there, you'll find some Rust code we will do this exercise with.

We used `todo!()` macros to mark places where you should put code to make the program run. Look at link:https://docs.rs/serde_json/latest/serde_json/#functions[`serde_json`] api for help.

> **Hint**  
> Serde comes with two traits: `Serializable` and `Deserializable`. These traits can be `derive` d for your `struct` or `enum` types. Other `serde-*` crates use these traits to convert our data type from and to corresponding representation (`serde-json` to JSON, `serde-yaml` to YAML, etc.).

> ***How come `main` returns an `anyhow::Result<()>`?***  
> By having `main` return a result, we can bubble errors up all the way to runtime. You can find more information about it in [Rust By Example](https://doc.rust-lang.org/rust-by-example/error/result.html#using-result-in-main). The `anyhow::Result` is a more flexible type of `Result`, which allows for easy conversion of error types.

> ***What is that `r#"...` thing?***  
> `r` in front of a string literal means it's a "raw" string. Escape sequences (`\n`, `\"`, etc.) don't work, and thus they are very convenient for things like regular expressions, JSON literals, etc.
>
> Optionally `r` can be followed by one or more symbols (like `#` in our case), and then your string ends when there's a closing double quote followed by the same number of the same symbols. This is great for cases when you want to have double quotes inside your string literal. For our example `r#" ... "#` works great for JSON. In rare cases you'd want to put two or more pound signs. Like, when you store CSS color values in your JSON strings:
```rust
// here `"#` would not terminate the string
r##"
    {
        "color": "#ff00ff"
    }
"##
```

## B.2 Your first Rust project

In this exercise, you will create a Rust crate that adheres to the guidelines that were pointed out during the lecture. Additionally, you will add and use dependencies, create unit tests, and create some documentation. You can view this exercise as a stepping stone to the final project.

*This exercise should be done in groups of 2 people*

### B.2 A Setting up
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

### B.2 B Quizzer
This exercise is about both design and finding information. You'll have to figure out s model to represent your quiz questions, as well as a means to store them into a JSON file, and load them yourself. Also, you will have to find out how to parse the program arguments.

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
