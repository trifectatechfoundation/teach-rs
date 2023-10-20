
---
layout: section
---

# Move Semantics

---
layout: two-cols
---
# Memory management

- Most of what we have seen so far is stack-based and small in size
- All these primitive types are `Copy`: create a copy on the stack every time
we need them somewhere else
- We don't want to pass a copy all the time
- Large data that we do not want to copy
- Modifying original data
- What about data structures with a variable size?

::right::

<Transform scale="0.9">

![Memory Layout](/images/A1-memory-expanded.svg)

</Transform>

---
layout: default
---
# Memory

- A computer program consists of a set of instructions
- Those instructions manipulate some memory
- How does a program know what memory can be used?

<!--
* A program is not just the code that is running, it is also the current state
of that program (the memory).
* But central here is the question: when does a program know when it can use
a specific part of that memory, when is it available?
-->

---

# Fundamentals

There are two mechanisms at play here, generally known as the stack and the heap

<div class="grid grid-cols-2">
    <div class="flex flex-col rounded-md p-1 bg-teal-100 text-center w-md h-250px">
        <div class="bg-red-100 rounded-t-md flex flex-col">
            <div class="bg-red-200 rounded-t-md p-1 border-red-500 border">Frame 1</div>
            <div class="bg-red-200 p-1 border-red-500 border border-t-0">Frame 2</div>
        </div>
        <div class="bg-blue-100 flex-1 align-middle flex flex-col">
            <div class="text-gray-500 p-1">Free memory</div>
        </div>
        <div class="bg-yellow-100 rounded-b-md h-130px flex flex-col">
            <div class="text-gray-500 p-2">Heap</div>
            <div class="bg-yellow-300 mb-3 h-7">Allocated</div>
            <div class="bg-yellow-300 mb-1 h-9"></div>
            <div class="bg-yellow-300 h-4"></div>
        </div>
    </div>
    <div>
        <div class="relative top-12 left-6">🠔 Stack pointer</div>
    </div>
</div>

<!--
* In this simplified view we see the stack mechanism and the heap mechanism
* The stack is a growing stack of used memory, where the only way to remove
memory from being used is by removing it from the top of the stack and the
only way to add is to put it on top of the stack.
* Somehow, as with a lot of CS stuff, we like to turn things around and think
of stacks growing down instead of up in the real world. That is because they are
at the end of the virtual memory address range. So if the stack grows, the stack
pointer (to the current stack frame) is decreased.
-->

---

# Fundamentals

There are two mechanisms at play here, generally known as the stack and the heap

<div class="grid grid-cols-2">
    <div class="flex flex-col rounded-md p-1 bg-teal-100 text-center w-md h-350px">
        <div class="bg-red-100 rounded-t-md flex flex-col">
            <div class="bg-red-200 rounded-t-md p-1 border-red-500 border">Frame 1</div>
            <div class="bg-red-200 p-1 border-red-500 border border-t-0">Frame 2</div>
            <div class="bg-red-200 p-1 border-red-500 border border-t-0">Frame 3</div>
        </div>
        <div class="bg-blue-100 flex-1 align-middle flex flex-col">
            <div class="text-gray-500 p-1">Free memory</div>
        </div>
        <div class="bg-yellow-100 rounded-b-md h-130px flex flex-col">
            <div class="text-gray-500 p-2">Heap</div>
            <div class="bg-yellow-300 mb-3 h-7">Allocated</div>
            <div class="bg-yellow-300 mb-1 h-9"></div>
            <div class="bg-yellow-300 h-4"></div>
        </div>
    </div>
    <div>
        <div class="relative top-19 left-6">🠔 Stack pointer</div>
        <div class="relative pl-7 top-20">
            A stack frame is allocated for every function call. It contains exactly
            enough space for all local variables, arguments and stores where the
            previous stack frame starts.
        </div>
    </div>
</div>

<!--
* We create a new part of the stack, called stack frame, every time we enter a function, meanwhile
we have a small special bit of memory, register, where the current top of the stack is
recorded.
-->

---

# Fundamentals

There are two mechanisms at play here, generally known as the stack and the heap

<div class="grid grid-cols-2">
    <div class="flex flex-col rounded-md p-1 bg-teal-100 text-center w-md h-250px">
        <div class="bg-red-100 rounded-t-md flex flex-col">
            <div class="bg-red-200 rounded-t-md p-1 border-red-500 border">Frame 1</div>
            <div class="bg-red-200 p-1 border-red-500 border border-t-0">Frame 2</div>
        </div>
        <div class="bg-blue-100 flex-1 align-middle flex flex-col">
            <div class="text-gray-500 p-1">Free memory</div>
        </div>
        <div class="bg-yellow-100 rounded-b-md h-130px flex flex-col">
            <div class="text-gray-500 p-2">Heap</div>
            <div class="bg-yellow-300 mb-3 h-7">Allocated</div>
            <div class="bg-yellow-300 mb-1 h-9"></div>
            <div class="bg-yellow-300 h-4"></div>
        </div>
    </div>
    <div>
        <div class="relative top-12 left-6">🠔 Stack pointer</div>
        <div class="relative pl-7 top-13">
            Once a function call ends we just move back up, and everything below is
            available as free memory once more.
        </div>
    </div>
</div>

<!--
* And as we leave a function, we just put the stack pointer back down and we
just act as if everything above it doesn't exist.
* Also take a look at the heap memory instead, look at how there are many
differently sized blocks of memory scattered across the heap.
-->

---

# Stack limitations

The stack has limitations though, because it only grows as a result of a
function call.

* Size of items on stack frame must be known at compile time
* If I don't know the size of a variable up front: What size should my stack
frame be?
* How can I handle arbitrary user input efficiently?

<style>
    .footnotes-sep {
        margin-top: 45px;
    }

    .footnotes {
        @apply text-xs opacity-65;
    }

    .footnote-backref {
        display: none;
    }
</style>

<!--
* You can definitely do a lot with just a stack, but really there are some
scenarios that aren't possible, or can only be done very inefficient when
we can only ever push and pop from the top of the stack.
* Because stack frames (at least for low level compiled languages such as Rust,
C and C++) need to be known at compile time, we also have somewhat limited
capabilities for dynamic variable sizes and dynamic user input
* Note that stack based operations are very much a solved problem, and you can
very safely use stack based variables in C and C++, because you don't have to
worry about cleaning them up, there are no pointers.
-->

---

# The Heap

If the lifetime of some data needs to outlive a certain scope, it can not be placed on the stack.
We need another construct: the heap.

It's all in the name, the heap is just one big pile of memory for you to store
stuff in. But what part of the heap is in use? What part is available?

* Data comes in all shapes and sizes
* When a new piece of data comes in we need to find a place in the heap that
still has a large enough chunk of data available
* When is a piece of heap memory no longer needed?
* Where does it start? Where does it end?
* When can we start using it?

<!--
* Meanwhile on the other side of our memory the heap is an unstructured pile
of data just waiting to be used. But how do we know what to use, when to use,
when to stop using? We can't keep on adding more and more memory or we would
run into a runaway memory leak quickly.
* Let's take a look how Rust solves working with the heap for us.
-->

---

# Variable scoping (recap)

```rust
fn main() { // nothing in scope here
    let i = 10; // i is now in scope
    if i > 5 {
        let j = i; // j is now also in scope
        println!("i = {}, j = {}", i, j);
    } // j is no longer in scope, i still remains
    println!("i = {}", i);
} // i is no longer in scope
```

<v-click>

* `i` and `j` are examples containing a `Copy` type
* What if copying is too expensive?

</v-click>

<!--
* When looking at how Rust solves working with the heap, we have to know a little
bit about variable scoping.
* In Rust, every variable has a scope, that is, a section of the code that that
variable is valid for. Note that this isn't that much different to other
programming languages.
* In our example we have `i` and `j`. Note how we can just create a copy by
assigning `i` to `j`.
* Here the type of i and j is actually known as a `Copy` type
* But sometimes there is data that would be way too much to Copy around every
time, it would make our program slow.
-->

---
layout: four-square
---

# Ownership

::topleft::

```rust
let x = 5;
let y = x;
println!("{}", x);
```

::topright::

<div class="no-line-numbers">

<v-click>

```text
Compiling playground v0.0.1 (/playground)
Finished dev [unoptimized + debuginfo] target(s) in 4.00s
Running `target/debug/playground`
5
```

</v-click>

</div>

::bottomleft::

<v-click>

```rust
// Create an owned, heap allocated string
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
```

</v-click>

<v-click at="4">

Strings store their data on the heap because they can grow

</v-click>

::bottomright::

<v-click at="3">

<div class="no-line-numbers">

```text
Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `s1`
--> src/main.rs:4:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
```

</div>

</v-click>

<!--
* Let's take the previous example and get rid of some scopes, instead we are
just going to assign x to y, and then print both x and y. What do we think
is going to happen?
* Now the same example again, but now with a String, "hello", we are just going
to assign it to another variable and then print both s1 and s2. What do we
think is going to happen now?
* See how this time the compiler doesn't even let us run the program. Hold on,
what's going on here?
* Actually, in Rust strings can grow, that means that we can no longer store
them on the stack, and we can no longer just copy them around by re-assigning
them somewhere else.
-->

---

<LightOrDark>
  <template #dark>
    <img src="/images/A1-i-own-this-dark.png" class="pl-30 h-90 float-right" />
  </template>
  <template #light>
    <img src="/images/A1-i-own-this-light.png" class="pl-30 h-90 float-right" />
  </template>
</LightOrDark>

# Ownership

- There is always ever only one owner of a stack value
- Once the owner goes out of scope (and is removed from the stack), any associated values on the
  heap will be cleaned up as well
- Rust transfers ownership for non-copy types: *move semantics*

<!--
* What we've just seen is the Rust ownership system in action.
* In Rust, every part of memory in use always has an owner variable. That
variable must always be the only owner, there can't be multiple owners.
* Once a scope that contains a variable ends we don't just pop the top from the
stack, but we also clean up any associated values on the heap.
* We can safely do this because we just said that this variable was the only
owner of that part of memory.
* Assigning a variable to another one actually moves ownership to the other
variable and removes it from the first variable, instead of aliasing it
(which is what C and C++ do)
-->

---

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `s1`
--> src/main.rs:4:43
  |
2 | let s1 = String::from("hello");
  |     -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 | let len = calculate_length(s1);
  |                            -- value moved here
4 | println!("The length of '{}' is {}.", s1, len);
  |                                       ^^ value borrowed here after move
```

</div>

</v-click>

<!--
* Moving also works when calling a function, the function takes ownership of
the variable that is passed to it
* That means that when the function ends it
will go out of scope and should be cleaned up
* What do you think that will happen in this case when we try and print the
string and the length of the string after the function call.
-->

---

# Moving out of a function

We can return a value to move it out of the function

```rust
fn main() {
    let s1 = String::from("hello");
    let (len, s1) = calculate_length(s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (usize, String) {
    (s.len(), s)
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling playground v0.0.1 (/playground)
Finished dev [unoptimized + debuginfo] target(s) in 5.42s
Running `target/debug/playground`
The length of 'hello' is 5.
```

</div>

</v-click>

<!--
* But what if we move a value into a function and we still want to use it
afterwards, we could choose to move it back at the end of the function, but
it really doesn't make for very nice code
* Note that Rust allows us to return multiple values from a function with
this syntax.
-->

---

# Clone

<img src="/images/A1-clone.jpg" class="float-right w-40" />

- Many types in Rust are `Clone`-able
- Use can use clone to create an explicit clone (in contrast to `Copy` which
  creates an implicit copy).
- Creating a clone can be expensive and could take a long time, so be careful
- Not very efficient if a clone is short-lived like in this example

```rust
fn main() {
    let x = String::from("hellothisisaverylongstring...");
    let len = get_length(x.clone());
    println!("{}: {}", x, len);
}

fn get_length(arg: String) -> usize {
    arg.len()
}
```

<!--
* There is something else in Rust
* Many types implement a way to create an explicit copy, such types are
clone-able. But note how we have to very explicitly say that we want a
clone.
* Such a clone is a full deep copy clone and can of course take a long
time, which is why Rust wants you to be explicit.
* Also in this example this is a really inefficient usage of our clone,
because it gets destroyed almost immediately after creation
-->