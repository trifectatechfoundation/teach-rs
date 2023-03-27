---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - E: Async and Rust for Web"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: 'Rust - E: Async and Rust for Web'
---
# Rust programming
Module E: Async and Rust for Web
<!-- Start with welcome, students entering -->
<!-- TODO add subject code -->

---
layout: default
---
# Last time...
- Trait objects: dynamic dispatch
- Design patterns
- Anti-patterns

*Any questions?*

<!--
- Recap on content from last time that current subject builds on
- Keep it short. Any longer explanations can be deferred to tutorial
-->

---
layout: default
---
# In this module
- Introduction to concurrency
    - Why and when use it
    - Async vs threads
    - The Rust async ecosystem
    - Compatibility
- The `Future` type
    - What's a future?
        - More and more complex versions up until the real signature
    - Futures are lazy
    - Build your own future
- `async fn`s and `async` blocks
    - `async fn`s are really functions returning an `async` block
    - How `async` and `await` are desugared into state machines
- How futures are run with an executor
    - naive polling
    - wakers
    - Pin (self-referential)
- Trade offs when using `async`
- Intro to `futures` and `tokio`
- Introduction to Rocket with Postgres


---
layout: default
---
# Learning objectives
- Understand the mechanics of async/await from a high-level point of view
- Understand the reason of Rust's implementation of async
- Work with Futures using the Tokio runtime
- Apply knowledge on `async` Rust in a web context

<!-- List this module's learning objectives -->

---
layout: cover
---
#  Module E
Async and Rust for Web
<!-- Start lecture content here -->

---
layout: default
---
# Content overview
TODO
---
layout: default
---
# Concurrency vs. Parallelism

| **Concurrency**    | **Parallelism**         |
| ------------------ | ----------------------- |
| Interleaves work   | Parallelizes work       |
| 1 or more cores    | 2 or more cores         |
| Waiting for events | Waiting for computation |
| <img src="https://tienda.bricogeek.com/6417-thickbox_default/sparkfun-thing-plus-esp32-wroom.jpg" class="h-40 center" /> | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d3/IBM_Blue_Gene_P_supercomputer.jpg/1920px-IBM_Blue_Gene_P_supercomputer.jpg" class="h-40 center" /> |

Today, we're focusing on concurrency: _asynchronous programming_

---
layout: default
---

# What's async?

- Concurrent programming model
- Very suitable for running large number of tasks
- Look and feel* of synchronous code through `async`/`await` syntax

**Well, not perfectly. We'll go into that*

---
layout: default
---

# Async vs OS Threads

|                      | <span style="color: red">**Async**</span> | <span style="color: blue">**OS Threads**</span> |
| -------------------- | ----------------------------------------- | ----------------------------------------------- |
| Spawning & switching | Cheap                                     | Expensive                                       |
| Blocking is ok       | No                                        | Yes                                             |
| Usage                | I/O bound tasks (web servers)             | CPU-bound tasks (Number crunching)              |
| Reuse sync code      | No                                        | Yes                                             |

[What Color is Your Function? ](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/)

---
layout: default
---

# What `async` looks like in Rust
To get an idea

```rust
/// An async function
async fn run() -> anyhow::Result<()> {
    /// Await loading and parsing config file
    let config = load_config(CONFIG_PATH).await?;
    /// Await scraping job
    let data = scrape(&config.urls).await?;
    data.report();
    Ok(())
}

fn main() {
    // Set up a `tokio` runtime with default configurations
    let runtime = tokio::runtime::Runtime::new().unwrap();
    // Run a Future to completion
    runtime.block_on(run());
}
```

*Question: What stands out to you? What strikes you as odd?*

---
layout: default
---

# Async in Rust

- Revolve around `Future` trait (~like JS Promise)  
  &rarr; `async fn`s return `Future`s

- `Future`s are inert
- `async` is zero-cost
- No built-in runtime
- Single- or multithreaded
- Can be mixed with other concurrency models
- `async` is relatively new and lacks some features

---
layout: default
---

# State of the `async` art
What you can expect doing `async` Rust

- Blazingly fast applications
- More interaction with advanced language features
- Compatibility issues (re: colored functions)
- Faster evolving ecosystem
- `async fn` in Traits not stable

*Work in progress*

---
layout: default
---

# Support of `async`

- Fundamental types and traits are in `std`
- `async`/`await` are native to the language
- Utilities/extensions in `futures` crate
- Async runtimes are third party

Example runtimes: `async-std`, `tokio`, `smol`

---
layout: section
---
# The `Future` trait
Foundation of async

---
layout: two-cols
---

# A `VerySimpleFuture`
```rust
trait VerySimpleFuture {
    type Output;
    /// Do work and check if task is completed.
    /// Returns [Poll::Ready], containing the 
    /// `Output` if task is ready,
    /// [Poll::Pending] if not
    fn poll(&mut self) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```
::right::
<v-click>

```rust
struct VerySimpleAlarm {
    alarm_time: Option<Instant>,
}

impl VerySimpleFuture for VerySimpleAlarm {
    type Output = ();

    fn poll(&mut self) -> Poll<()> {
        match self.alarm_time {
            None => Poll::Ready(()),
            Some(alarm_time) 
                if Instant::now() > alarm_time => {
                    self.alarm_time.take();
                    Poll::Ready(())
            },
            Some(_) => Poll::Pending,
        }
    }
}
```
</v-click>

---
layout: two-cols
---

# Executing `VerySimpleFuture`
 ```rust
fn main() {
    let mut first_alarm = VerySimpleAlarm::new(
        Instant::now() + Duration::from_secs(3)
    );
    let mut snooze_alarm = VerySimpleAlarm::new(
        Instant::now() + Duration::from_secs(5)
    );

    loop {
        if let Poll::Ready(_) = first_alarm.poll() {
            println!("Beep beep beep");
        }
        if let Poll::Ready(_) = snooze_alarm.poll() {
            println!("You're late for work!")
        }
    }
}
 ```
::right::
<v-click>
<div>

```txt
[pause...]
Beep beep beep
Beep beep beep
[... a few moments later...]
You're late for work!
Beep beep beep
You're late for work!
Beep beep beep
You're late for work!
[...ad infinitum]
```

## It works! 🎉

*Question: How can `VerySimpleFuture` be improved?*
</div>
</v-click>

---
layout: default
---
# Limitation

- Busy waiting
- How to signal the executor the future is *actually* ready to be polled?

<v-click>
<br/>

## ⏰ Introduce a Waker

General idea: 
- Run some callback to notify executor
- Have executor implement some job queue

</v-click>

---
layout: default
---
# A `SimpleFuture`

```rust
trait SimpleFuture {
    type Output;

    fn poll(&self, wake: fn()) -> Poll<Self::Output>;
}

pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() { // Does syscall
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake); // Does syscall
            Poll::Pending
        }
    }
}
```

*Adapted from [Asynchronous programming in Rust](https://rust-lang.github.io/async-book/02_execution/02_future.html)*


---
layout: two-cols
---

# Joining `SimpleFuture`s

```rust
pub struct Join<FutureA, FutureB> {
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();

```

*Adapted from [Asynchronous programming in Rust](https://rust-lang.github.io/async-book/02_execution/02_future.html)*
::right::
```rust
    fn poll(&mut self, wake: fn()) 
        -> Poll<Self::Output> 
    {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take(); // Drop future A
            }
        }
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take(); // Drop future B
            }
        }
        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(()) // Both futures dropped
        } else {
            Poll::Pending // A future is pending
        }
    }
}
```


---
layout: default
---
# And then...

```rust
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                Poll::Ready(()) => self.first.take(),
                Poll::Pending => return Poll::Pending,
            };
        }
        self.second.poll(wake)
    }
}
```

*Adapted from [Asynchronous programming in Rust](https://rust-lang.github.io/async-book/02_execution/02_future.html)*

---
layout: default
---

# `SimpleFuture` takeaways

- Composing `SimpleFuture`s requires no heap allocations
- Composing `SimpleFuture`s requires no deeply nested callbacks

---
layout: default
---

# The `Future` is now!

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

*Question: what stands out to you?*
<v-click>
<div>

- `&mut self` &rarr; `Pin<&mut Self>`: makes `Self` immovable
- `wake: fn()` &rarr; `cx: &mut Context<'_>`: contains a `Waker`

*More on `Pin<&mut Self>` later*
</div>
</v-click>

---
layout: section
---

# `async` and `await`

---
layout: default
---

# Expanding `async`
&nbsp;  

**" `Futures` are cool, but why didn't I see them in the web scraper example?"*

`async fn`s and `async` blocks are syntactic sugar generating `Future`
<v-click>
<div>

```rust
async fn foo() -> u8 { 5 }
```
</div>
</v-click>
<v-click>
<div>

is equivalent to:
```rust
fn foo() -> impl Future<Output=u8> {
    async {
        5
    }
}
```
</div>
</v-click>
<v-click>
<div>

which is equivalent to:
```rust
fn foo() -> impl Future<Output=u8> {
    /// Create a future that is immediately ready with a value.
    futures::future::ready(5)
}
```
</div>
</v-click>

---
layout: default
---

# Expanding `async` and `await`

```rust
let fut_one = /* ... */;
let fut_two = /* ... */;
async move {
    fut_one.await;
    fut_two.await;
}
```
<v-click>
<div>
Generates an opaque type implementing `Future`:

```rust
struct AsyncFuture {
    fut_one: FutOne,
    fut_two: FutTwo,
    state: State,
}
enum State {
    AwaitingFutOne,
    AwaitingFutTwo,
    Done,
}
```

*This and the following is not the actually generated code, but it's a good mental model*
</div>
</v-click>


---
layout: default
---

# Expanding `async` and `await` (2)

```rust
impl Future for AsyncFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        loop {
            match self.state {
                State::AwaitingFutOne => match self.fut_one.poll(/* - snip - */) {
                    Poll::Ready(()) => self.state = State::AwaitingFutTwo,
                    Poll::Pending => return Poll::Pending,
                }
                State::AwaitingFutTwo => match self.fut_two.poll(/* - snip - */) {
                    Poll::Ready(()) => self.state = State::Done,
                    Poll::Pending => return Poll::Pending,
                }
                State::Done => return Poll::Ready(()),
            }
        }
    }
}   
```

Kind of looks like `AndThenFut`!

*Adapted from [Asynchronous programming in Rust](https://rust-lang.github.io/async-book/03_async_await/01_chapter.html)*

---
layout: default
---

# `async`/`await` expansion takeaways

- Rust generates state machines out of `await`s
- Generated code may become very complex, but original is easy to follow


---
layout: default
---

# * `async` and lifetime elision
&nbsp;
`async fn`s which accept references, return a `Future` bound by argument lifetime:

```rust
async fn foo(x: &u8) -> u8 { *x }
```

is equivalent to:
```rust
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}
```

- `async move` takes ownership of any variables it references
  - i.e. `x`, which itself is a *reference*
- The returned `impl Future` internally holds the references
- The returned `impl Future` must be `await`ed within `'a`


---
layout: default
---
# Summary
<!-- Very quickly go over the learning objectives and how they were covered -->

---
layout: default
---
# Practicalities
<!-- Use this slide to announce any organizational information -->

---
layout: end
---