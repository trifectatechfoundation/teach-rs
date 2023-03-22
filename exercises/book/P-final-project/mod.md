# Module P - Final project

It is time to submit a proposal for the final project.

- Form groups of 2 or 3 people. Working solo is not permitted.
- Build a small rust project
- Up to 2 groups can work on the same topic

The proposal needs to be submitted by _30th of March, 2023_ by opening a Github repo, placing your proposal there, and granting access to [`@hdoordt`](https://github.com/hdoordt/).

The proposal must contain the following sections:
- Your names
- Introduction to your idea
- Requirements in brief
- The dependencies you want to use
- Optional: A rudimentary diagram of the architecture

Of course, if you want to discuss your idea before handing in your proposal, or if you have any other questions, please reach out via Discord.

Any reparations to the proposals must be handed in on the _6th of April 2023_


Af the end of the project following will be required (deadline is the _4th of May, 2023_)

- The source of your project (GitHub)
- A live 10 minute presentation, including a _short_ demonstration (and an additional 2 minutes for questions) during the final lecture
- A small report on what you did (3 pages max). It contains the following sections:
    - Introduction to your idea
    - Requirements in more detail (not too detailed, though)
    - Design diagram. Keep it high-level
    - Design choices. What choices did you make, what were alternatives, and why did you choose the way you did?
    - Dependencies and what they're used for
    - Evaluation. What went well? What went not so well? How does implmementing a bigger project in Rust feel compared to other languages?

## Project suggestions

You are encouraged to suggest your own project, here are some suggestions. We will add more ideas as they come up.

- Use a popular crate to build something
    - tokio (network applications)
    - bitvec (lowlevel binary protocols)
    - bevy (games)
    - a serializer/deserializer using Serde
    - RTIC or Embassy (embedded applications, H)
- Build a GUI application (https://www.areweguiyet.com/)
- Implement a more complex data structure
    - implement and benchmark a doubly linked list
    - benchmark the ntpd-rs ipfilter https://github.com/pendulum-project/ntpd-rs/blob/main/ntp-daemon/src/ipfilter.rs
    - add "seamless slices" to the rust implementation of `RocList` (ask Folkert)
    - Image renderer and editor (PNG, SVG)
    - Implement a simple HTTP1 static file server on raw TCP sockets
- Programming languages
    - an interpreter for False (https://strlen.com/false-language/)
    - an interpreter for (a subset of) webassembly
    - contribute to Roc (Folkert is a maintainer and will help you)
- Make an open source contribution
    NOTE: make sure your contribution has a good chance of being accepted; don't just create extra work for project maintainers 
    - update inkwell's kaleidoscope example so it also works with llvm 15 https://github.com/TheDan64/inkwell/blob/master/examples/kaleidoscope/main.rs#L199
