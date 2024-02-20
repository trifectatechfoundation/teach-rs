# Teach-rs ModMod

`modmod` is a tool that stitches together the teach-rs content into a collection of exercise scaffolding, the exercise decription book, and Sli.dev slides.
Provide it with a track definition TOML file, and it will collect everything that's needed to run your custom course.

## Usage
```txt
Usage: modmod [OPTIONS] --output <OUTPUT_DIR> <TRACK_TOML_PATH>

Arguments:
  <TRACK_TOML_PATH>  

Options:
  -o, --output <OUTPUT_DIR>  The folder the output will be written to
  -c, --clear                Clear the output folder
  -h, --help                 Print help
```

For instance, you can run the following to render the Rust intro track into `./target/course`:

```bash
cargo run -- -o target/course -c ../content/rust-intro.track.toml
````

Have a look at file the structure and the TOML files in [content](../content) to get an idea of how the input is structured.

## Output

ModMod outputs a file structure that looks like this:

```txt
$ tree -L 2
.
├── book
│   ├── book.toml
│   └── src
├── exercises
│   ├── 1-course-introduction
│   ├── 2-foundations-of-rust
│   └── 3-crate-engineering
└── slides
    ├── 1-introduction.md
    ├── 2-foundations-of-rust.md
    ├── 3-crate-engineering.md
    ├── images
    └── package.json
````
Note that many subfolders were excluded in previous example. You can run `tree` yourself in the course output folder to see the structure deeper down.

The `book` folder contains definition of the MdBook containing the exercise descriptions. You can build it using [MdBook](https://github.com/rust-lang/mdBook):
```bash
# install mdbook using Cargo
cargo install mdbook

# Move to book path
cd /path/to/course/book

# build the book
mdbook build

# serve the book
mdbook serve
````

The `exercises` folder contains the scaffolding of the included exercises as referred to by the exercise description book.
The `slides` folder contains a package of the unit slides, which you can render using [Slidev](https://sli.dev).
```bash
# Move to slides path
cd /path/to/course/slides

# Install dependencies
npm install

# Build the slides of unit 1
npm build-1

# Render the slides of unit 1
npm run dev-1
```
