# Contributing

Great that you want to contribute to Rust 101!

# Improving the material
If you see anything you think can easily be improved with a small pull request, such as fixing typos, fixing links or re-formulating sentences, please open a PR directly.

If you want to propose a structural change to one or more modules, please open an issue first, so we can discuss your ideas.
Please clearly state your proposed changes and your motivation for suggesting the changes. In case an improvement proposal is accepted, a PR with the changes referring to the relevant issue can be made.

# Adding a module
Rust 101 is intended to be used as a basis for custom courses that either are about Rust itself or use Rust to teach some other concept.
To that end, the Rust 101 project aims to incorporate modules on diverse topics.
If you feel Rust 101 should cover a topic that is currently not covered, you can propose adding a module by opening an issue.
State the main goal and the learning objectives of the module, as well as the covered topics and proposed exercises.
In case an addition proposal is accepted, a PR with the proposed changes referring to the relevant issue can be opened.
Please use `slides/format-lectures.md` as a format for the module's lecture slides.
The exercise description should be added in a new book chapter, by adding an entry to `exercises/book/SUMMARY.md`, pointing to a new markdown file.
Any exercise scaffolding can be added as a new Cargo project in `exercises/[your-module]/[exercise-name]`.
Prefix the exercise scaffolding project folders with the exercise number.

Have a look at the structure of the current slides and exercises to get a better understanding of how modules are organized.
