# Module 0 - Installing the tools
In this file you'll find instructions on how to install the tools we'll use during the course.

All of these tools are available for Linux, macOS and Windows users.
We'll need the tools to write and compile our Rust code, and allow for remote mentoring.
*Important: these instructions are to be followed at home, before the start of the first tutorial.*
*If you have any problems with installation, contact the lecturers! We won't be addressing installation problems during the first tutorial.*

## Rust and Cargo
First we'll need `rustc`, the standard Rust compiler.
`rustc` is generally not invoked directly, but through `cargo`, the Rust package manager.
`rustup` takes care of installing `rustc` and `cargo`.

This part is easy: go to <https://rustup.rs> and follow the instructions.
Please make sure you're installing the latest default toolchain.
Once done, run

```bash
rustc -V && cargo -V
```

The output should be something like this:

```bash
rustc 1.67.1 (d5a82bbd2 2023-02-07)
cargo 1.67.1 (8ecd4f20a 2023-01-10)
```

Using Rustup, you can install Rust toolchains and components. More info: 
- <https://rust-lang.github.io/rustup>
- <https://doc.rust-lang.org/cargo>

## Rustfmt and Clippy
To avoid discussions, Rust provides its own formatting tool, Rustfmt.
We'll also be using Clippy, a collection of lints to analyze your code, that catches common mistakes for you.
You'll notice that Rusts Clippy can be a very helpful companion.
Both Rustfmt and Clippy are installed by Rustup by default.

To run Rustfmt on your project, execute:

```bash
cargo fmt
```

To run clippy:

```bash
cargo clippy
```

More info:
- Rustfmt: <https://github.com/rust-lang/rustfmt>
- Clippy: <https://github.com/rust-lang/rust-clippy>

## Visual Studio Code
During the course, we will use Visual Studio Code (vscode) to write code in.
Of course, you're free to use your favorite editor, but if you encounter problems, you can't rely on support from us.
Also, we'll use vscode to allow for remote collaboration and mentoring during tutorial sessions.

You can find the installation instructions here: <https://code.visualstudio.com/>.

We will install some plugins as well.
The first one is Rust-Analyzer.
Installation instructions can be found here <https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer>.
Rust-Analyzer provides a lot of help during development and in indispensable when getting started with Rust.

Another plugin we'll install is Live Share.
We will use the plugin to share screens and provide help during remote tutorial sessions.
The extension pack also contains the Live Share Audio plugin, which allows for audio communication during share sessions.
Installation instructions can be found here: <https://marketplace.visualstudio.com/items?itemName=MS-vsliveshare.vsliveshare-pack>

The last plugin we'll use is CodeLLDB.
This plugin enables debugging Rust code from within vscode.
You can find instructions here: <https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb>.


More info:
- <https://rust-analyzer.github.io/>
- <https://code.visualstudio.com/learn/collaboration/live-share>

## Git
We will use Git as version control tool.
If you haven't installed Git already, you can find instructions here: <https://git-scm.com/book/en/v2/Getting-Started-Installing-Git>.
If you're new to Git, you'll also appreciate GitHubs intro to Git <https://docs.github.com/en/get-started/using-git/about-git> and the Git intro with vscode, which you can find here: <https://www.youtube.com/watch?v=i_23KUAEtUM>.

More info: <https://www.youtube.com/playlist?list=PLg7s6cbtAD15G8lNyoaYDuKZSKyJrgwB->

## Course code
Now that everything is installed, you can clone the source code repository.
The repository can be found here: <https://github.com/tweedegolf/101-rs>.

To clone the repository, you'll need an account on GitHub.
Make sure you have one, it will serve you well, even outside this workshop.

Instructions on cloning the repository can be found here: <https://docs.github.com/en/get-started/getting-started-with-git/about-remote-repositories#cloning-with-https-urls>

# Trying it out
Now that you've got the code on your machine, navigate to it using your favorite terminal and run:

```
cd exercises/0-intro
cargo run
```

This command may take a while to run the first time, as Cargo will first fetch the crate index from the registry.
It will compile and run the `intro` package, which you can find in `exercises/0-intro`.
If everything goes well, you should see some output:

```
   Compiling intro v0.1.0 (/home/henkdieter/tg/edu/101-rs/exercises/0-intro)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/intro`
🦀 Hello, world! 🦀
You've successfully compiled and run your first Rust project!
```
If Rust-Analyzer is set up correctly, you can also click the '▶️ Run'-button that is shown in `exercises/0-intro/src/main.rs`.
With CodeLLDB installed correctly, you can also start a debug session by clicking 'Debug', right next to the '▶️ Run'-button.
Play a little with setting breakpoints by clicking on a line number, making a red circle appear and stepping over/into/out of functions using the controls.
You can view variable values by hovering over them while execution is paused, or by expanding the 'Local' view under 'Variables' in the left panel during a debug session.
