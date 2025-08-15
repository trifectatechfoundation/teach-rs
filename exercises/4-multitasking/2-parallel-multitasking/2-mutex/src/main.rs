// In this exercise we build a basic Mutex, a synchronization primitive that guarantees safe access
// to a piece of shared mutable state. In the implementation, we must guarantee that only one
// thread can modify the value within the mutex at any one time.
//
// This exercise uses "unsafe" - something that we will look at in more detail in a later lecture.
// use of the "unsafe" keyword does not necessarily mean that the code is really
// 'unsafe' (in the general sense of the word), but it does mean that you as a programmer have to take on
// responsibility of making sure the code is not doing any "funny business", as you would in C/C++.
//
// Some background: the formal term for "funny business" is "undefined behaviour (UB)"; the most visible type of
// undefined behaviour is that your program crashes in a dramatic and unexpected way such as a segmentation fault.
// But it may can also have more destructive effects. (Note that "panic" may be a drastic way to end a program, but
// since a programmer put it in the code, it is not "unexpected"). In ordinary code, Rust's type system and borrow
// checker ensure that no UB can occur.
use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

struct Mutex<T> {
    cell: UnsafeCell<T>,
    locked: AtomicBool,
}

// TODO implement Send for Mutex<T>.
//
// Implementing `Sync` is an assertion that `Mutex<T>` is safe to move between threads, which is
// equivalent to saying that `&Mutex<T>` implement `Send`.
//
// Conceptually a mutex can be used to send a value from one thread to another. If `T` is not
// `Send`, can `Mutex<T>` implement `Sync`?

// even with a reference to `Mutex<T>`, we can actually move a value of type T between threads. But
// moving values of type T is only allowed if `T: Send`
unsafe impl<T: Send> Sync for Mutex<T> {
    /* no methods to implement */
}

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
        todo!()
    }

    pub fn into_inner(self) -> T {
        // TODO: implement into_inner()
        // hint: look at the available functions on UnsafeCell
        // question: do you need to `block_until_you_lock`?
        todo!()
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // unsafe code will be covered in module F. The standard API for `UnsafeCell` is not
        // sufficient to implement this function, even though it does not break any of rust's rules.
        // We explicitly take on the task of verifying correctness here, and promise to the compiler
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
        // We explicitly take on the task of verifying correctness here, and promise to the compiler
        // the operation below is valid.
        //
        // SAFETY: we have an exclusive reference to the mutex guard,
        // and therefore have exclusive access to the value protected by the mutex
        unsafe { &mut *self.mutex.cell.get() }
    }
}

// TODO: implement a `Drop` for MutexGuard that unlocks the mutex
// use the `unlock` method that is already defined for `Mutex`

// The function main() should execute cleanly and normally, i.e. without entering a deadlock
// situation and certainly not causing any undefined behaviour.
//
// imaginary bonus points: use the atomic_wait crate https://docs.rs/atomic-wait/latest/atomic_wait/index.html
// to replace the spin loop with something more efficient. This section https://marabos.nl/atomics/building-locks.html#mutex of
// "Rust Atomics and Locks" explains how to do it (and has lots of other good stuff too)

fn main() {
    let n = Mutex::new(String::from("threads: "));
    std::thread::scope(|s| {
        s.spawn(|| n.lock().push_str("0"));
        s.spawn(|| n.lock().push_str("1"));
        s.spawn(|| n.lock().push_str("2"));
        s.spawn(|| n.lock().push_str("3"));
        s.spawn(|| n.lock().push_str("4"));
        s.spawn(|| n.lock().push_str("5"));
        s.spawn(|| n.lock().push_str("6"));
        s.spawn(|| n.lock().push_str("7"));
        s.spawn(|| n.lock().push_str("8"));
        s.spawn(|| n.lock().push_str("9"));
        s.spawn(|| n.lock().push_str("a"));
        s.spawn(|| n.lock().push_str("b"));
        s.spawn(|| n.lock().push_str("c"));
        s.spawn(|| n.lock().push_str("d"));
        s.spawn(|| n.lock().push_str("e"));
        s.spawn(|| n.lock().push_str("f"));
    });
    println!("{}", n.into_inner());
}
