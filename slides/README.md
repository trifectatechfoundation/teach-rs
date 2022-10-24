# Rust 101 lecture and tutorial slides

In this folder, you'll find the source of the slides used in the course lectures and tutorials. These slides are rendered using [Slidev](https://sli.dev/). Slidev can render the slides as PDF, as well as set up a web app to do the presentation in.

## Installing
Install [Node.js](https://nodejs.org) and [Yarn](https://yarnpkg.com/). Navigate to `/path/to/101-rs/slides` and run

```bash
yarn
```

This will download all tools to render the slides.

## Running
Refer to the `scripts` array in [package.json](./package.json) for available commands. For example, run the following command in you terminal to set up a dev server for the module 0 lecture slides:

```bash
yarn run dev-0
```

This command will output the instructions on how to open the slides in your browser.

Refer to the [Slidev guides](https://sli.dev/guide/) for more commands and syntax.
