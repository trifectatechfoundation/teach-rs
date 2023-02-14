---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - X: Y"
drawings:
  persist: false
fonts:
  mono: Fira Mono
layout: cover
title: 'Rust - X: Y'
---
# Rust programming
Module X: description
<!-- Start with welcome, students entering -->
<!-- TODO add subject code -->

---
layout: three-slots
---
## Who am i?
::left::
- Ferris
- I Love Rust

::right::
<img src="https://rustacean.net/assets/rustacean-orig-noshadow.png" alt="Photo Ferris" width="300" />
<!-- Optionally quickly introduce yourself, add photo -->

---
layout: default
---
# Last time...
- Ownership model
- Move semantics
<!-- Recap on content from last time that current subject builds on -->

---
layout: section
---
# Quick questions

Any questions on last time's subject?
<!-- Keep it short. Any longer explanations can be deferred to tutorial -->

---
layout: section
---
# Recap Quiz

## [Link to quiz here]

---
layout: iframe
url: http://your-quiz-url-here
---
<!-- insert URL to quiz roundup in slide option `url` -->

---
layout: default
---
# In this module
<!-- Introduce today's subject -->


---
layout: default
---
# Learning objectives
<!-- List this module's learning objectives -->

- Working with C from Rust and vice versa 
- be familiar with the C representation 
- be familiar with the C calling convention
- Work with `cargo bindgen`
- Make nice rust APIs around C libraries

---
layout: default
---
# Content overview

- The end of Moore's law
- Concurrency & Parallelism
- data parallelism with Rayon
- thread-based concurrency

<!-- Give an overview of the subjects covered in this lecture -->
<!-- Incorporate any breaks as well -->

---
layout: center
---
<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Moore%27s_Law_Transistor_Count_1970-2020.png/1280px-Moore%27s_Law_Transistor_Count_1970-2020.png" class="h-130 rounded shadow" />

---
layout: default
---
# The end of Moore's law 

- No more free performance  
- Software Development as a profession originates around this time 
- Many dynamic languages (Python, Perl, Ruby, JavaScript) are from this time

---
layout: default
---
# A "solution": just duplicate the hardware 

- We get computers with multiple cores 
- On a per-dollar basis, that means we still get more compute
- But comes with all sorts of problems
- Programming Languages have not done a great job of fixing those problems

---
layout: default
---
# Concurrency vs. Parallelism 

| **Concurrency**  | **Parallelism**   |
| -------          | ------------      |
| Interleaves work | Parallelizes work |
| 1 or more cores  | 2 or more cores   |
| <img src="https://tienda.bricogeek.com/6417-thickbox_default/sparkfun-thing-plus-esp32-wroom.jpg" class="h-40 center" /> | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d3/IBM_Blue_Gene_P_supercomputer.jpg/1920px-IBM_Blue_Gene_P_supercomputer.jpg" class="h-40 center" /> |
| Waiting for events | Waiting for computation |


---
layout: default
---
# Pleasantly Parallel Problems 

- Some problems are "embarrasingly parallel"
- A common architecture is MapReduce:

---
layout: default
---
# TF–IDF 

An algorithm for searching in a big collection of text documents

- term frequency–inverse document frequency 
- TF: "how often does a word occur in a particular document" 
- IDF: "how rare is the word across all documents"

Problem:

- how do we aggregate the results?


---
layout: default
---
# TF–IDF in Rayon

```rust
use std::collections::HashMap;
use rayon::prelude::*;

fn document_frequency(documents: &[&str]) -> HashMap<&str, usize> {
    documents
        .par_iter()
        .map(|document| term_occurence(document))
        .reduce(HashMap::default, combine_occurences);
}

/// Map each word in the document to the value 1
fn term_occurence(document: &str) -> HashMap<&str, usize> {
    todo!()
}


/// combine the counts from maps a and b. 
fn combine_occurences<'a>(
    a: HashMap<&'a str, usize>,
    b: HashMap<&'a str, usize>,
) -> HashMap<&'a str, usize> {
    todo!()
}
```

---
layout: default
---

# `a + b + c + d = (0 + a + b) + (0 + c + d)`

```rust

// for each word, how often it occurs across all documents
documents
    .par_iter()
    .map(|document| count_words(document))
    .reduce(HashMap::default, combine_documents);
```

- this idea is called a "monoid"


---
layout: default
---

# So far

- Rayon makes data-parallel programming in rust extremely convenient

---
layout: default
---

# Fearless concurrency 

- a process can spawn multiple threads of execution. These run concurrently (may run in parallel)  

```rust
use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
```

- Question: what is the output of this program?

---
layout: default
---

# Expected output

maybe 

```
Hello from another thread!
This is my thread id: ThreadId(411)
Hello from another thread!
This is my thread id: ThreadId(412)
Hello from the main thread.
```

or

```
Hello from another thread!
This is my thread id: ThreadId(412)
Hello from another thread!
This is my thread id: ThreadId(411)
Hello from the main thread.
```

---
layout: default
---

# Expected output

but most likely 

```
Hello from the main thread.
```

The process exits when the main thread is done!


- `.join()` can be used to block the main thread until the child is done

```rust
fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();
}
```

- `.join()` turns a panic in the thread into an `Err` 

---
layout: default
---

# Thread lifetime

- a more typical example

```rust
let numbers = Vec::from_iter(0..=1000);

let t = thread::spawn(move || {
    let len = numbers.len();
    let sum = numbers.iter().sum::<usize>();
    sum / len
});

let average = t.join().unwrap();

println!("average: {average}");
```

- `numbers` must be `move`d into the closure!

---
layout: default
---

# Thread lifetime

- otherwise `numbers` might be dropped while the thread is still using it! 

```rust
let numbers = Vec::from_iter(0..=1000);

let t = thread::spawn(|| {
    let len = numbers.len();
    let sum = numbers.iter().sum::<usize>();
    sum / len
});

drop(numbers); // oh no

let average = t.join().unwrap();

println!("average: {average}");
```

---
layout: default
---

# Thread lifetime: make it known

- explicitly bound the lifetime with a scope 

```rust
let numbers = Vec::from_iter(0..=1000);

let average = thread::scope(|s| {
    s.spawn(|| {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    }).join().unwrap()
});

println!("average: {average:?}");
```

---
layout: default
---

- of course, borrowing rules still apply 

```rust
let mut numbers = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|| {
        numbers.push(1);
    });
    s.spawn(|| {
        numbers.push(2); // Error!
    });
});
```

```txt
error[E0499]: cannot borrow `numbers` as mutable more than once at a time
 --> example.rs:7:13
  |
4 |     s.spawn(|| {
  |             -- first mutable borrow occurs here
5 |         numbers.push(1);
  |         ------- first borrow occurs due to use of `numbers` in closure
  |
7 |     s.spawn(|| {
  |             ^^ second mutable borrow occurs here
8 |         numbers.push(2);
  |         ------- second borrow occurs due to use of `numbers` in closure
```

---
layout: default
---

# Fearless concurrency

- restrictions on multiple mutable borrows prevent data races: it is never the case that one thread is modifying data that another thread is looking at

---
layout: default
---

# Re-defining references 

- `&T`: (possibly) shared reference
- `&mut T`: exclusive reference


for safe mutation, we need exclusive *access*, which we can get in multiple ways:

- we have an exclusive reference to the value
- we own the value (we can exclusively borrow from ourselves)
- access is inherently exclusive



---
layout: default
---

# Mutual Exclusion 

- `Mutex` allows mutation of a `T` through a shared `&Mutex<T>` reference

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let n = Mutex::new(String::from("foo"));
    thread::scope(|s| {
        s.spawn(|| {
            n.lock().unwrap().push_str("bar");
        });

        s.spawn(|| {
            n.lock().unwrap().push_str("baz")
        });
    });
    println!("{}", n.into_inner().unwrap());
}
```

- threads lock the mutex, but there is no `unlock` ?!

---
layout: default
---

# Sharing ownership between threads 

```rust
impl<T> Mutex<T> {
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> { 
        ...
    }
}
```

- Acquires a mutex, blocking the current thread until it is able to do so
- Returns a `PoisonError` if a thread panicked while holding the lock
- Returns a `MutexGuard`, proof to the type checker that we hold the lock
- `MutexGuard<'_, T>` implements `DerefMut<Target = T>`, so we can use it like a mutable reference
- dropping the `MutexGuard` unlocks the mutex

---
layout: default
---

# Sharing ownership between threads 

- with `Arc`, ownership can be shared between threads

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let n = Arc::new(Mutex::new(String::from("foo")));

    thread::scope(|s| {
        let n2 = n.clone();
        s.spawn(|| {
            n2.lock().unwrap().push_str("bar");
        });

        s.spawn(|| {
            n.lock().unwrap().push_str("baz")
        });
    });

    // n has moved and cannot be used here
}
```

- `Arc` stands for "atomically reference counted"
- useful for resolving lifetime issues 
- `.clone()` is very cheap: it just increments the reference count

---
layout: default
---

# Reinventing the Mutex 

- `Mutex` allows mutation through a shared `&T` reference

```
struct Mutex<T> {}

impl<T> Mutex<T> { 
    fn lock(&self) -> MutexGuard<'_, T> {
    }
}
``` 

---
layout: default
---

# Orchestrating Threads 

- MPSC: many producer, single consumer

```rust
use std::thread;
use std::sync::mpsc::channel;

fn main() { 
    // Create a shared channel that can be sent along from many threads
    // where tx is the sending half (tx for transmission), and rx is the receiving
    // half (rx for receiving).
    let (tx, rx) = channel();

    thread::scope(|s| { 
        for (i, tx) in std::iter::repeat(tx).take(10).enumerate() {
            s.spawn(move || {
                tx.send(i).unwrap();
            });
        }

        s.spawn(move || { 
            while let Ok(msg) = rx.recv() { 
                println!("{msg}");
            }
        });
    });

    println!("done");
}
``` 

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
<!-- Below are example slides you can use -->

---
layout: playground
---
# Code example

```rust
fn main() {
  println!("Hello world!");
}
```
<!-- Slide for code examples with a link to Rust playground -->

---
layout: iframe
url: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main()%20%7B%0A%20%20println!(%22Hello%20world!%22)%3B%0A%7D
---
<!-- Iframe slide containing Rust playground -->
