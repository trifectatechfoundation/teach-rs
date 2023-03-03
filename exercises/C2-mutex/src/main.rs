use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

struct Mutex<T> {
    cell: UnsafeCell<T>,
    locked: AtomicBool,
}

// TODO implement Sync for Mutex<T>.
//
// Implementing `Sync` is an assertion that `Mutex<T>` is safe to share between threads, which is
// equivalent to saying that `&Mutex<T>` implement `Send`.
//
// Conceptually a mutex can be used to send a value from one thread to another. If `T` is not
// `Send`, can `Mutex<T>` implement `Sync`?
unsafe impl<T> Sync for Mutex<T> where T: Send {}

struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Mutex {
            cell: UnsafeCell::new(value),
            locked: AtomicBool::new(false),
        }
    }

    fn block_until_you_lock(&self) {
        // loop until `locked` becomes false, then set it to `true`
        while self.locked.swap(true, Ordering::Acquire) {
            // a hint to the OS that it should maybe prioritise other threads
            std::hint::spin_loop();
        }
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }

    pub fn lock(&self) -> MutexGuard<T> {
        // TODO: implement lock()
        self.block_until_you_lock();

        MutexGuard { mutex: self }
    }

    pub fn into_inner(self) -> T {
        // TODO: implement into_inner()
        self.block_until_you_lock();

        self.cell.into_inner()
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // unsafe code will be covered in module F. The standard API for `UnsafeCell` is not
        // sufficient to implement this function, even though it does not break any of rust's rules.
        // We explicitly take on the task of verifying correctness here, and promis to the compiler
        // the operation below is valid.
        //
        // SAFETY: we have a shared reference to the mutex guard,
        // and therefore have (shared) access to the value protected by the mutex
        unsafe { &*self.mutex.cell.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // unsafe code will be covered in module F. The standard API for `UnsafeCell` is not
        // sufficient to implement this function, even though it does not break any of rust's rules.
        // We explicitly take on the task of verifying correctness here, and promis to the compiler
        // the operation below is valid.
        //
        // SAFETY: we have an exclusive reference to the mutex guard,
        // and therefore have exclusive access to the value protected by the mutex
        unsafe { &mut *self.mutex.cell.get() }
    }
}

// TODO: implement a `Drop` for MutexGuard that unlocks the mutex
impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock()
    }
}

// imaginary bonus points: use the atomic_wait crate https://docs.rs/atomic-wait/latest/atomic_wait/index.html
// to replace the spin loop with something more efficient. This section https://marabos.nl/atomics/building-locks.html#mutex of
// "Rust Atomics and Locks" explains how to do it (and has lots of other good stuff too)

fn main() {
    let n = Mutex::new(String::from("foo"));
    std::thread::scope(|s| {
        s.spawn(|| n.lock().push_str("bar"));
        s.spawn(|| n.lock().push_str("baz"));
    });
    println!("{}", n.into_inner());
}
