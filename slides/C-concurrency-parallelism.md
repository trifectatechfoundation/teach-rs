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

fn inverse_document_frequency(documents: &[&str]) -> HashMap<&str, f32> {
    // for each word, how often it occurs across all documents
    let document_frequency = documents
        .par_iter()
        .map(|document| count_words(document))
        .reduce(HashMap::default, combine_documents);

    // divide by the number of documents to get "rareness" score
    let idf = todo!();
}

fn count_words(document: &str) -> HashMap<&str, usize> {
    todo!()
}

fn combine_documents<'a>(
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
let document_frequency = documents
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
- Question: how is this different than data-parallel computation?

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
