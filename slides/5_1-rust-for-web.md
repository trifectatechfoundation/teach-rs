---
theme: "teach-rs"
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - 5.1: Rust for Web"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - 5.1: Rust for Web"
---

# Rust programming

Module 5: Rust for Web

## Unit 1

Rust for Web

---

# Learning objectives



---
layout: default
---

# [Are we web yet?](https://www.arewewebyet.org/)

- "Yes! And it's freaking fast!"
- Several web frameworks exist
  - [`rocket`](https://rocket.rs/)
  - [`actix-web`](https://actix.rs/)
  - [`warp`](https://github.com/seanmonstar/warp)
  - [`axum`](https://github.com/tokio-rs/axum)
  - ...lots more
- Several DB drivers and ORMs
- Much more!

*Tip: have a look if you want to do web stuff in your final project*
---
layout: default
---

# Axum demo: setting up server

```rust
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // set up shared, mutable state.
    let app_state = Arc::new(Mutex::new(Vec::new()));
    // build our application with a route
    let app = Router::new()
        .route("/:name", get(handler))
        .with_state(app_state);
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---
layout: default
---
# Axum demo: request hander

```rust
/// A very long type name warrants a type alias
type AppState = State<Arc<Mutex<Vec<String>>>>;

async fn handler(
    Path(name): Path<String>,
    State(past_names): State<AppState>,
) -> Html<String> {
    let mut response = format!("<h1>Hello, {name}!</h1>");

    // Of course, locking here is not very fast
    let mut past_names = past_names.lock().await;

    if !past_names.is_empty() {
        response += "<h2>Names we saw earlier:</h2>";
        past_names
            .iter()
            .for_each(|name| response += &format!("<p>{name}</p>"))
    }

    past_names.push(name);

    Html(response)
}
```
---

# To do

Issue: [tweedegolf/teach-rs#76](https://github.com/tweedegolf/teach-rs/issues/76)
---

# To do

Issue: [tweedegolf/teach-rs#77](https://github.com/tweedegolf/teach-rs/issues/77)
---

# To do

Issue: [tweedegolf/teach-rs#75](https://github.com/tweedegolf/teach-rs/issues/75)


---

# Summary
