In this exercise, you'll write a simple chat server and client based on [Tokio](https://lib.rs/crates/tokio). Open `#[modmod:exercise_dir]` in your editor. The project contains a `lib.rs` file, in which a type `Message` resides. This `Message` defines the data the chat server and clients use to communicate.

# #[modmod:exercise_ref].A Server ⭐⭐⭐
The chat server, which resides in `src/bin/server.rs` listens for incoming TCP connections on port 8000, and spawns two tasks (futures):

- `handle_incoming`: reads lines coming in from the TCP connection. It reads the username the client provides, and broadcasts incoming `Messages`, possibly after some modification.
- `handle_outgoing`: sends messages that were broadcasted by the `handle_incoming` tasks to the client over TCP.

Both `handle_incoming` and `handle_outgoing` contain a number to `todo`s. Fix them.

To start the server, run

```bash
cargo run --bin server
```

# #[modmod:exercise_ref].B Client ⭐⭐
The chat client, residing in `src/bin/client.rs` contains some todo's as well. Fix them to allow for registration and sending `Message`s to the server.

To start the client, run

```bash
cargo run --bin client
```

If everything works well, you should be able to run multiple clients and see messages sent from each client in every other.