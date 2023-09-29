---
theme: default
class: text-center
highlighter: shiki
lineNumbers: true
info: "Rust - A1: Language basics"
drawings:
    persist: false
fonts:
    mono: Fira Mono
layout: cover
title: "Rust - A1: Language basics"
---

# Rust programming

Module A1: Language basics
<!-- Start with welcome, students entering -->
<!-- TODO add subject code -->

---
layout: default
---

# In this module

<!-- Introduce today's subject -->
An introduction to the Rust language and basic concepts

---
layout: default
---

# Learning objectives

<!-- List this module's learning objectives -->

- Get acquainted with Rust and its goals
- Introduction to ownership model
- Learn basic syntax and operators

---
layout: section
---

# Anyone has experience with Rust?

---
layout: cover
---

# Module A1

Language basics
<!-- Start lecture content here -->

---
layout: default
---

# Content overview

- Why learn Rust?
- Basic Rust syntax (quickly)
- The ownership model

---
layout: section
---

# Why learn Rust?

---
layout: default
---
# How to choose a language

What characteristics do you want?
<!-- Try asking the audience first -->

<v-click>

1. Efficiency          <!-- not just fast code, also saving the planet -->
2. Safety              <!-- preventing mistakes, but also failing gracefully -->
3. Elegance            <!-- this is not a subjective quality -->
4. Practical relevance <!-- this excludes favourite toy languages -->

Most languages tick two of these boxes, if you are lucky you get three.

</v-click>

---
layout: default
---
# What Rust promises

1. Pedal to the metal
2. Comes with a warranty
3. Beautiful code
4. Rust is practical

---
layout: default
---
# Pedal to the metal

- Compiled language, not interpreted
- State-of-the-art code generation using LLVM
- No garbage collector getting in the way of execution
- Usable in embedded devices, operating systems and demanding websites

---
layout: default
---
# Rust comes with a warranty

- Strong type system helps prevent silly bugs
- Explicit errors instead of exceptions
- Type system tracks lifetime of objects
	* No more *"null pointer exception"*
- Programs don't trash your system accidentally
	* Warranty *can* be voided (`unsafe`)

*"If it compiles, it is more often correct."*

---
layout: default
---
# Rust code is elegant

- Data types can capture many problem domains
- Orthogonal, expression-oriented language
- Combine declarative and imperative paradigms
- Concise syntax instead of boilerplate
- Toolchain that suggests improvements to your code

---
layout: default
---
# Rust is practical

- Can interface with legacy C code
- Supported on many platforms
- Active user base maintains a healthy ecosystem
- Adoption by Microsoft, Amazon, Google, ...

---
layout: default
---
# Why should *you* learn Rust?

- Learning a new language teaches you new tricks
	* You will also write better C/C++ code

- Rust is a young, but quickly growing platform
	* You can help shape its future
	* Demand for Rust programmers will increase!

---
layout: section
---

# Basic Syntax

---
layout: default
---

# A new project

```bash
$ cargo new hello-world
```

<v-click>

```bash
$ cd hello-world
$ cargo run
```

</v-click>

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/101-rs/Projects/hello-world)
Finished dev [unoptimized + debuginfo] target(s) in 0.74s
Running `target/debug/hello-world`
Hello, world!
```

</div>

</v-click>


---

# Hello, world!

```rust {all|1-3|2|5-11|6-10|7,9|all}
fn main() {
    println!("Hello, world! fib(6) = {}", fib(6));
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/101-rs/Projects/hello-world)
Finished dev [unoptimized + debuginfo] target(s) in 0.28s
Running `target/debug/hello-world`
Hello, world! fib(6) = 8
```

</div>

</v-click>

<!--
- `fn main()` is the entrypoint of your program
- `println!` (output something to stdout)
- Note the call syntax `fib(6)` with comma separated parameters
- exclamation mark is a macro (we'll see later)
- `fn` short for function, declare a function
- we see our first types here, we'll see more about them later
- `u64` unsigned integer types, all integers have an explicit size, 64 bits in
this case
- `if-else` is without parenthesis for the expression, but with required braces
for the blocks
- no explicit return keyword (will get back to that)
-->

---

# Variables

```rust {all|2|all}
fn main() {
    let some_x = 5;
    println!("some_x = {}", some_x);
    some_x = 6;
    println!("some_x = {}", some_x);
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/101-rs/Projects/hello-world)
error[E0384]: cannot assign twice to immutable variable `some_x`
--> src/main.rs:4:5
  |
2 |     let some_x = 5;
  |         ------
  |         |
  |         first assignment to `some_x`
  |         help: consider making this binding mutable: `mut some_x`
3 |     println!("some_x = {}", some_x);
4 |     some_x = 6;
  |     ^^^^^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `hello-world` due to previous error
```

</div>

</v-click>

<!--
- By convention Rust uses snake case (i.e. all lowercase with underscores) for
variable names
- The immutable variable cannot be mutated in any way (exceptions apply)
-->

---

# Variables

```rust
fn main() {
    let mut some_x = 5;
    println!("some_x = {}", some_x);
    some_x = 6;
    println!("some_x = {}", some_x);
}
```

<v-click>

<div class="no-line-numbers">

```text
Compiling hello-world v0.1.0 (/home/101-rs/Projects/hello-world)
Finished dev [unoptimized + debuginfo] target(s) in 0.26s
Running `target/debug/hello-world`
some_x = 5
some_x = 6
```

</div>

</v-click>

<!--
- We declare a mutable variable by adding `mut`, we can update the value for
that variable
-->

---

# Assigning a type to a variable

```rust
fn main() {
    let x: i32 = 20;
}
```

- Rust is strongly and strictly typed
- Variables use type inference, so no need to specify a type
- We can be explicit in our types (and sometimes have to be)

---
layout: two-cols
---

# Integers

| Length        | Signed  | Unsigned |
|---------------|---------|----------|
| 8 bits        | `i8`    | `u8`     |
| 16 bits       | `i16`   | `u16`    |
| 32 bits       | `i32`   | `u32`    |
| 64 bits       | `i64`   | `u64`    |
| 128 bits      | `i128`  | `u128`   |
| pointer-sized | `isize` | `usize`  |

- Rust prefers explicit integer sizes
- Use `isize` and `usize` sparingly

::right::

<v-click>

# Literals

```rust
fn main() {
    let x = 42; // decimal as i32
    let y = 42u64; // decimal as u64
    let z = 42_000; // underscore separator

    let u = 0xff; // hexadecimal
    let v = 0o77; // octal
    let w = 0b0100_1101; // binary
    let q = b'A'; // byte syntax (stored as u8)
}
```

</v-click>

<!--
- Use isize and usize mostly when working with indexing or other things
that need to have a specific size for your platform
-->

---

# Floating points and floating point literals

```rust
fn main() {
    let x = 2.0; // f64
    let y = 1.0f32; // f32
}
```

- `f32`: single precision (32-bit) floating point number
- `f64`: double precision (64-bit) floating point number
- `f128`: 128-bit floating point number

<!--
- Rust uses f64 by default
- Similar to integers you can append the type of float to indicate a specific
literal type
-->

---

# Numerical operations

```rust
fn main() {
    let sum = 5 + 10;
    let difference = 10 - 3;
    let mult = 2 * 8;
    let div = 2.4 / 3.5;
    let int_div = 10 / 3; // 3
    let remainder = 20 % 3;
}
```

<v-click>

- These expressions do overflow/underflow checking in debug
- In release builds these expressions are wrapping, for efficiency
- You cannot mix and match types here, not even between different integer
types

```rust
fn main() {
    let invalid_div = 2.4 / 5;          // Error!
    let invalid_add = 20u32 + 40u64;    // Error!
}
```

</v-click>

<!--
- Rust has your typical operations, just as with other C-like languages
-->

---

# Booleans and boolean operations

```rust
fn main() {
    let yes: bool = true;
    let no: bool = false;
    let not = !no;
    let and = yes && no;
    let or = yes || no;
    let xor = yes ^ no;
}
```

---

# Comparison operators

```rust
fn main() {
    let x = 10;
    let y = 20;
    x < y; // true
    x > y; // false
    x <= y; // true
    x >= y; // false
    x == y; // false
    x != y; // true
}
```

Note: as with numerical operators, you cannot compare different integer and
float types with each other

```rust
fn main() {
    3.0 < 20; // invalid
    30u64 > 20i32; // invalid
}
```

<!--
- Boolean operators short-circuit: i.e. if in `a && b`, a is already false,
then the code for b is not executed
-->

---

# Characters

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

- A character is a 32-bit unicode scalar value
- Very much unlike C/C++ where char is 8 bits

<!--
- The final scalar type is the character, but it isn't often seen.
- Note that it is not the same as u8 (a byte) in Rust, and cannot be used
interchangeably.
- We'll see later that strings do not use chars, but are encoded as UTF-8
instead.
-->

---

# Strings
```rust
    // Owned, heap-allocated string *slice*
    let s1: String = String::from("Hello, 🌍!");
```

- Rust strings are UTF-8-encoded
- Unlike C/C++: *Not null-terminated*
- Cannot be indexed like C strings
- Actually many types of strings in Rust

<!--
- Rusts strings are complicated, because all strings are complicated
- Rusts strings are UTF-8 encoded sequences. Not null terminated unlike C/C++
- Literal strings are static by default, called string *slices*, being pointers to their start, accompanied with the length
-->

---
layout: three-slots
---
# Tuples

::left::

```rust
fn main() {
    let tup: (i32, f32, char) = (1, 2.0, 'a');
}
```

- Group multiple values into a single compound type
- Fixed size
- Different types per element
- Create a tuple by writing a comma-separated list of values inside parentheses

::right::

<v-click>

```rust
fn main() {
    let tup = (1, 2.0, 'Z');
    let (a, b, c) = tup;
    println!("({}, {}, {})", a, b, c);

    let another_tuple = (true, 42);
    println!("{}", another_tuple.1);
}
```

- Tuples can be destructured to get to their individual values
- You can also access individual elements using the period operator followed by
  a zero based index

</v-click>

<!--
- Note how the tuple type and the tuple value are constructed similarly, but
the type contains individual element types
- We will see more powerful variants of this destructuring later
- Note that after destructuring the original value is no longer accessible
-->

---

# Arrays

```rust
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    println!("{}", arr[0]);
    let [a, b, c] = arr;
    println!("[{}, {}, {}]", a, b, c);
}
```

- Also a collection of multiple values, but this time all of the same type
- Always a fixed length at compile time (similar to tuples)
- Use square brackets to access an individual value
- Destructuring as with tuples
- Rust always checks array bounds when accessing a value in an array

<!--
- Create an array by writing a comma-separated list of values inside brackets
- Note how unlike C/C++ arrays must always have a length defined at compile
time and cannot be constructed dynamically
- You can also construct an array using [value; repetitions] instead of having
to write out each value if you have a repeating value.
- For the type declaration the element type and count are separated by a semicolon and
written between brackets
-->

---

# Control flow

```rust {all|3-10|4-9|8|13-16|18-20|all}
fn main() {
    let mut x = 0;
    loop {
        if x < 5 {
            println!("x: {}", x);
            x += 1;
        } else {
            break;
        }
    }

    let mut y = 5;
    while y > 0 {
        y -= 1;
        println!("y: {}", x);
    }

    for i in [1, 2, 3, 4, 5] {
        println!("i: {}", i);
    }
}
```

<!--
- A loop or if condition must always evaluate to a boolean type, so no `if 1`
- Use break to break out of a loop, also works with for and while, continue
to skip to the next iteration
-->

---

# Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn returns_nothing() -> () {
    println!("Nothing to report");
}

fn also_returns_nothing() {
    println!("Nothing to report");
}
```

- The function boundary must always be explicitly annotated with types
- Within the function body type inference may be used
- A function that returns nothing has the return type *unit* (`()`)
- The function body contains a series of statements optionally ending with an
expression

<!--
- Rust always uses snake case for variables and functions
- We must annotate each function parameter with a type, using a colon
- We must annotate the function return type using an arrow (`->`) followed by
the return type
- Unit may be omitted, note the syntax looks like an empty tuple: a tuple with
no value members has no instances, just as with unit.
- In Rust you must always specify your type signatures for function boundaries
-->

---

# Statements
- Statements are instructions that perform some action and do not return a value
- A definition of any kind (function definition etc.)
- The `let var = expr;` statement
- Almost everything else is an expression

## Example statements
```rust
fn my_fun() {
    println!("{}", 5);
}
```

```rust
let x = 10;
```

<v-click>

```rust
let x = (let y = 10); // invalid
```

</v-click>

<!--
- Note how `let` within a `let` is not allowed because of `let` being a statement,
thus you may not declare multiple variables at the same time with the same
value
- `let` is a statement because ownership makes multiple assignments behave
differently than many would expect, it is almost never what you want in
Rust
- It also makes sense if you think of all other declarations also being
statements
-->

---

# Expressions

- Expressions evaluate to a resulting value
- Expressions make up most of the Rust code you write
- Includes all control flow such as `if` and `while`
- Includes scoping braces (`{` and `}`)
- An expression can be turned into a statement by adding a semicolon (`;`)

```rust {all|2-5}
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y); // 4
}
```

---

# Expressions - control flow

- Control flow expressions as a statement do not need to end with a semicolon
if they return *unit* (`()`)
- Remember: A block/function can end with an expression, but it needs to have
the correct type

```rust {all|3-8|10-15}
fn main() {
    let y = 11;
    // if as an expression
    let x = if y < 10 {
        42
    } else {
        24
    };

    // if as a statement
    if x == 42 {
        println!("Foo");
    } else {
        println!("Bar");
    }
}
```

---

# Scope

- We just mentioned the scope braces (`{` and `}`)
- Variable scopes are actually very important for how Rust works

```rust
fn main() {
    println!("Hello, {}", name);  // invalid: name is not yet defined
    let name = "world";  // from this point name is in scope
    println!("Hello, {}", name);
} // name goes out of scope
```

---

# Scope

As soon as a scope ends, all variables for that scope can be removed from the
stack

```rust
fn main() { // nothing in scope here
    let i = 10; // i is now in scope
    if i > 5 {
        let j = 20; // j is now also in scope
        println!("i = {}, j = {}", i, j);
    } // j is no longer in scope, i still remains
    println!("i = {}", i);
} // i is no longer in scope
```

<!--
- Note that this is the same with C and C++
-->

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
layout: section
---
# Rust's ownership model

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
        <div class="relative top-12 left-6">← Stack pointer</div>
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
        <div class="relative top-19 left-6">← Stack pointer</div>
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
        <div class="relative top-12 left-6">← Stack pointer</div>
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

---
layout: default
---
# Summary

* Loads of syntax
* Values are owned by variables
* Values may be moved to new owners or copied
* Some types may be explicitly `Clone`d

---
layout: default
---
# Practicalities

- Follow installation instructions: https://101-rs.tweede.golf
- Exercises A1 during tutorial
- Help each other out!

---
layout: end
---
