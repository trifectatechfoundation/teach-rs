# Teach-rs ModMod

`modmod` is a tool that stitches together the teach-rs content into a collection of exercise scaffolding, the exercise decription book, and Sli.dev slides.
Provide it with a track definition TOML file, and it will collect everything that's needed to run your custom course.

## Usage

To generate the course package, use modmod's `generate` subcommand:

```txt
Usage: modmod generate [OPTIONS] --output <OUT_DIR> <TRACK_TOML_PATH>

Arguments:
  <TRACK_TOML_PATH>

Options:
  -o, --output <OUT_DIR>
          The folder the output will be written to
  -c, --clear
          Clear the output folder
      --slide-url-base <SLIDE_URL_BASE>
          Use this as a base when deploying the slides to a web server [default: /]
  -p, --patch <PATCH_FILE>
          Generate patch file to update output dir at given path
      --theme <SLIDE_THEME>
          The name of the Slidev theme to use in generated slide decks [default: teach-rs]
      --json-stub <PACKAGE_JSON>
          The path of the package.json stub to use when generating the slide package
  -h, --help
          Print help
```

For instance, you can run the following to render the Rust intro track into `./target/course`:

```bash
cargo run -- generate -o target/course -c ../content/rust-intro.track.toml
```

Have a look at file the structure and the TOML files in [content](../content) to get an idea of how the input is structured.

To create stubs for new content, you can use modmod's `create` subcommand.

## Output

ModMod outputs a file structure that looks like this:

```txt
$ tree -L 3
.
├── book
│   ├── book.toml
│   └── src
│       ├── advanced-syntax.md
│       ├── basic-syntax.md
│       ├── closures-and-dynamic-dispatch.md
│       ├── crate-engineering.md
│       ├── interior-mutability.md
│       ├── introduction.md
│       ├── ownership-and-references.md
│       ├── SUMMARY.md
│       └── traits-and-generics.md
├── exercises
│   ├── 1-course-introduction
│   │   └── 1-introduction
│   ├── 2-foundations-of-rust
│   │   ├── 1-basic-syntax
│   │   ├── 2-ownership-and-references
│   │   ├── 3-advanced-syntax
│   │   ├── 4-traits-and-generics
│   │   ├── 5-closures-and-dynamic-dispatch
│   │   └── 6-interior-mutability
│   └── 3-crate-engineering
│       └── 1-crate-engineering
└── slides
    ├── 1_1-introduction.md
    ├── 2_1-basic-syntax.md
    ├── 2_2-ownership-and-references.md
    ├── 2_3-advanced-syntax.md
    ├── 2_4-traits-and-generics.md
    ├── 2_5-closures-and-dynamic-dispatch.md
    ├── 2_6-interior-mutability.md
    ├── 3_1-crate-engineering.md
    ├── images
    │   └── <FILES OMITTED FOR CONCISENESS>
    └── package.json
```

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
```

The `exercises` folder contains the scaffolding of the included exercises as referred to by the exercise description book.
The `slides` folder contains a package of the unit slides, which you can render using [Slidev](https://sli.dev).
```bash
# Move to slides path
cd /path/to/course/slides

# Install dependencies
npm install

# Build the slides of module 2, unit 1
npm run build-2_1

# Render the slides of module 2, unit 1
npm run dev-2_1
```
