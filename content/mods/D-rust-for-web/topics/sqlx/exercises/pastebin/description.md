This exercise is about writing a simple [pastebin](https://en.wikipedia.org/wiki/Pastebin) web server. Like the quizzer app, you will need to set up the project yourself. This webserver will be powered by [`axum`](https://lib.rs/crates/axum).

- Data is kept in memory. Bonus if you use a database or `sqlite`, but first make the app function properly without.
- Expose a route to which a POST request can be sent, that accepts some plain text, and stores it along with a freshly generated UUID. The UUID is sent in the response. You can use the [`uuid` crate](https://docs.rs/uuid/latest/uuid/) to generate UUIDs.
- Expose a route to which a GET request can be sent, that accepts a UUID and returns the plain text corresponding to the UUID, or a 404 error if it doesn't exist.
- Expose a route to which a DELETE request can be sent, that accepts a UUID and deletes the plain text corresonding to that UUID.
