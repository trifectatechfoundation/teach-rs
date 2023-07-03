

---
layout: section
---

# `async` and `await`

---
layout: three-slots
---

# Expanding `async`
&nbsp;  

*" `Futures` are cool, but why didn't I see them in the web scraper example?"*

<v-click>
<div>
<br/>

`async fn`s and `async` blocks are syntactic sugar generating `Future`s
```rust
async fn foo() -> u8 { 5 }
```
</div>
</v-click>
::left::
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
::right::
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
async move {            // <-- generated Future takes ownership of referenced variables
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

- Rust generates state machines out of `async` blocks that implement `Future`
- You can `await` `Future`s
- Every `await` point introduces a new state
- Generated code may become very complex, but original is easy to follow
