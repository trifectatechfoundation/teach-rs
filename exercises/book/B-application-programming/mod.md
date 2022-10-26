# Module B - Application programming

## B.1 Your first Rust project

In this exercise, you will create a Rust crate that adheres to the guidelines that were pointed out during the lecture. Additionally, you will add and use dependencies, create unit tests, and create some documentation. You can view this exercise as a stepping stone to the final project.

### B.1 A Setting up
Create a new project using `cargo new --name quizzer`. Make sure it acts as both a binary and a library. That means there will be both a `src/main.rs` and a `src/lib.rs` file in your crate:

```bash
$ tree
.
├── Cargo.toml
└── src
    ├── lib.rs
    └── main.rs
```

Add some dependencies using `cargo add <name>@<version>`:
   -  [`clap` 4.0](https://lib.rs/crates/clap) Also, read <https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html>
   -  [`serde-json` 1.0](https://lib.rs/crates/serde_json)
   -  [`anyhow` 1.0](https://lib.rs/crates/anyhow)

**Before items contain links to their page on lib.rs. Make sure you get a general idea of what these crates are for. Don't dive too deep just yet**

Also, add `serde`, which acts as a backend for `serde-json`. We will use its `derive` feature, which will allow you to derive the powerful `Serialize` and `Deserialize` traits. Run the following command:

```bash
cargo add serde@1.0 --features derive
```

We will use these dependencies to write a quiz game creator and player. You may add other dependencies as needed. It has the following functional requirements:
 - It runs as a command-line tool in your terminal.
 - It has two modes: question-entering mode, and quiz mode. The mode is selected with a subcommand
   - Question-entering mode: Allows for entering multiple-choice quiz questions, with 4 possible answers each, exactly 1 of them being correct. The questions are stored on disk as a JSON file.
 - Quiz mode: The app can present the questions loaded from the JSON file to the user
 - 