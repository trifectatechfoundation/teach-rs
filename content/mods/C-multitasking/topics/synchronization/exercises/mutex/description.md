The basic mutex performs a spin-loop while waiting to take the lock. That is terribly inefficient. Luckily, your operating system is able to wait until the lock becomes available, and will just put the thread to sleep in the meantime. 

This functionality is exposed in the [atomic_wait crate](https://docs.rs/atomic-wait/latest/atomic_wait/index.html). The [section on implementing a mutex](https://marabos.nl/atomics/building-locks.html#mutex) from "Rust Atomics and Locks" explains how to use it.

- change the `AtomicBool` for a `AtomicU32`
- implement `lock`. Be careful about spurious wakes: after `wait` returns, you must stil check the condition
- implement unlocking (`Drop for MutexGuard<T>` using `wake_one`.

The linked chapter goes on to further optimize the mutex. This is technically out of scope for this course, but we won't stop you if you try (and will still try to help if you get stuck)!
