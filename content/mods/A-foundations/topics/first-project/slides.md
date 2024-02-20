
---
layout: section
---
# Meeting Rust

---
layout: default
---

# A new project

```bash
$ cargo new hello-world
```

<v-click>

```bash
$ cd hello-world
$ cargo run
```

</v-click>

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/teach-rs/Projects/hello-world)
Finished dev [unoptimized + debuginfo] target(s) in 0.74s
Running `target/debug/hello-world`
Hello, world!
```

</div>

</v-click>


---

# Hello, world!

```rust {all|1-3|2|5-11|6-10|7,9|all}
fn main() {
    println!("Hello, world! fib(6) = {}", fib(6));
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/teach-rs/Projects/hello-world)
Finished dev [unoptimized + debuginfo] target(s) in 0.28s
Running `target/debug/hello-world`
Hello, world! fib(6) = 8
```

</div>

</v-click>

<!--
- `fn main()` is the entrypoint of your program
- `println!` (output something to stdout)
- Note the call syntax `fib(6)` with comma separated parameters
- exclamation mark is a macro (we'll see later)
- `fn` short for function, declare a function
- we see our first types here, we'll see more about them later
- `u64` unsigned integer types, all integers have an explicit size, 64 bits in
this case
- `if-else` is without parenthesis for the expression, but with required braces
for the blocks
- no explicit return keyword (will get back to that)
-->
