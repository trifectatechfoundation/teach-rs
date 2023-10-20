---
layout: default
---

# Moving ownership between threads

- Some values should never be shared or moved between threads

The `Send` and `Sync` marker traits enforce this:

```rust
pub unsafe auto trait Send { /* no method */ }
pub unsafe auto trait Sync { /* no method */ }
```

- `Send`: A type is Send if it can be sent to another thread. In other words, if ownership of a value of that type can be transferred to another thread
- `Sync`: A type is Sync if it can be shared with another thread. In other words, a type T is Sync if and only if a shared reference to that type `&T` is Send


---
layout: default
---

# `Send`

- A type is Send if it can be sent to another thread. In other words, if ownership of a value of that type can be transferred to another thread


```rust
impl<T: ?Sized> !Send for MutexGuard<'_, T>
impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T>
```

- On certain OS's, only the thread that locked a mutex may unlock it again!

