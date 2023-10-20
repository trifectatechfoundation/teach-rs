Channels are a very useful way to communicate between threads and `async` tasks. They allow for decoupling your application into many tasks. You'll see how that can come in nicely in exercise E.2. In this exercise, you'll implement two variants: a oneshot channel and a multi-producer-single-consumer (MPSC) channel. If you're up for a challenge, you can write a broadcast channel as well.

# #[modmod:exercise_ref].A MPSC channel ⭐⭐
A multi-producer-single-consumer (MPSC) channel is a channel that allows for multiple `Sender`s to send many messages to a single `Receiver`.

Open `#[modmod:exercise_dir]` in your editor. You'll find the scaffolding code there. For part A, you'll work in `src/mpsc.rs`. Fix the `todo!`s in that file in order to make the test pass. To test, run:

```bash
cargo test -- mpsc
```

If your tests are stuck, probably either your implementation does not use the `Waker` correctly, or it returns `Poll::Pending` where it shouldn't.

# #[modmod:exercise_ref].B Oneshot channel ⭐⭐⭐
A oneshot is a channel that allows for one `Sender` to send exactly one message to a single `Receiver`.

For part B, you'll work in `src/broadcast.rs`. This time, you'll have to do more yourself. Intended behavior:

- `Receiver` implements `Future`. It returns `Poll::Ready(Ok(T))` if `inner.data` is `Some(T)`, `Poll::Pending` if `inner.data` is `None`, and `Poll::Ready(Err(Error::SenderDropped))` if the `Sender` was dropped.
- `Receiver::poll` replaces `inner.waker` with the one from the `Context`.
- `Sender` consumes `self` on send, allowing the it to be used no more than once. Sending sets `inner.data` to `Some(T)`. It returns `Err(Error::ReceiverDropped(T))` if the `Receiver` was dropped before sending.
- `Sender::send` wakes `inner.waker` after putting the data in `inner.data`
- Once the `Sender` is dropped, it marks itself dropped with `inner`
- Once the `Receiver` is dropped, it marks itself dropped with `inner`
- Upon succesfully sending the message, the consumed `Sender` is not marked as dropped. Instead `std::mem::forget` is used to avoid running the destructor.

To test, run:
```bash
cargo test -- broadcast
```

# #[modmod:exercise_ref].C Broadcast channel (bonus) ⭐⭐⭐⭐
A Broadcast channel is a channel that supports multiple senders and receivers. Each message that is sent by any of the senders, is received by every receiver. Therefore, the implemenentation has to hold on to messages until they have been sent to every receiver that has not yet been dropped. This furthermore implies that the message shoud be cloned upon broadcasting.

For this bonus exercise, we provide no scaffolding. Take your inspiration from the `mpsc` and `oneshot` modules, and implement a `broadcast` module yourself.
