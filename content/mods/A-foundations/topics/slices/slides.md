
---
layout: section
---

# Slices

---

# Vectors and arrays
What if we wanted to write a sum function, we could define one for arrays of
a specific size:

```rust
fn sum(data: &[i64; 10]) -> i64 {
  let mut total = 0;
  for val in data {
    total += val;
  }
  total
}
```

---

# Vectors and arrays
Or one for just vectors:

```rust
fn sum(data: &Vec<i64>) -> i64 {
  let mut total = 0;
  for val in data {
    total += val;
  }
  total
}
```

---

# Slices
What if we want something to work on arrays of any size? Or what if we want
to support summing up only parts of a vector?

* A slice is a dynamically sized view into a contiguous sequence
* Contiguous: elements are layed out in memory such that they are evenly spaced
* Dynamically sized: the size of the slice is not stored in the type, but is
  determined at runtime
* View: a slice is never an owned data structure
* Slices are typed as `[T]`, where `T` is the type of the elements in the slice

---

# Slices

```rust
fn sum(data: [i64]) -> i64 {
  let mut total = 0;
  for val in data {
    total += val;
  }
  total
}

fn main() {
  let data = vec![10, 11, 12, 13, 14];
  println!("{}", sum(data));
}
```

<v-click>

```text
   Compiling playground v0.0.1 (/playground)
error[E0277]: the size for values of type `[i64]` cannot be known at compilation time
 --> src/main.rs:1:8
  |
1 | fn sum(data: [i64]) -> i64 {
  |        ^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `[i64]`
help: function arguments must have a statically known size, borrowed types always have a known size
```

</v-click>

<!--
- This cannot compile because [T] cannot exist on its own because it is never
  an owned data structure
- We must always put slices behind a pointer type
-->

---

# Slices

```rust
fn sum(data: &[i64]) -> i64 {
  let mut total = 0;
  for val in data {
    total += val;
  }
  total
}

fn main() {
  let data = vec![10, 11, 12, 13, 14];
  println!("{}", sum(&data));
}
```

<v-click>

```text
   Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.89s
     Running `target/debug/playground`
60
```

</v-click>

---

# Slices

* `[T]` is an incomplete type: we need to know how many `T` there are
* Types that have a known compile time size implement the `Sized` trait, raw
  slices do **not** implement it
* Slices must always be behind a reference type, i.e. `&[T]` and `&mut [T]`
  (but also `Box<[T]>` etc)
* The length of the slice is always stored together with the reference

<div style="margin-top: 50px; margin-left:auto; margin-right:auto; display:block; width:50%;">

<LightOrDark>
    <template #dark>
        <img src="/images/A2-slice-ptr-dark.svg"/>
    </template>
    <template #light>
        <img src="/images/A2-slice-ptr-light.svg"/>
    </template>
</LightOrDark>

</div>

---

# Creating slices
Because we cannot create slices out of thin air, they have to be located
somewhere. There are three possible ways to create slices:

* Using a borrow
  - We can borrow from arrays and vectors to create a slice of their entire
    contents
* Using ranges
  - We can use ranges to create a slice from parts of a vector or array
* Using a literal (for immutable slices only)
  - We can have memory statically available from our compiled binary

---

# Creating slices
Using a borrow

```rust
fn sum(data: &[i32]) -> i32 { /* ... */ }

fn main() {
  let v = vec![1, 2, 3, 4, 5, 6];
  let total = sum(&v);
  println!("{}", total);
}
```

---

# Creating slices
Using ranges

```rust
fn sum(data: &[i32]) -> i32 { /* ... */ }

fn main() {
  let v = vec![0, 1, 2, 3, 4, 5, 6];
  let all = sum(&v[..]);
  let except_first = sum(&v[1..]);
  let except_last = sum(&v[..5]);
  let except_ends = sum(&v[1..5]);
}
```

* The range `start..end` contains all values `x` with `start <= x < end`.

<v-click>

* Note: you can also use ranges on their own, for example in a for loop:

```rust
fn main() {
  for i in 0..10 {
    println!("{}", i);
  }
}
```

</v-click>

---

# Creating slices
From a literal

```rust {3-5,12|7-9,13|all}
fn sum(data: &[i32]) -> i32 { todo!("Sum all items in `data`") }

fn get_v_arr() -> &'static [i32] {
    &[0, 1, 2, 3, 4, 5, 6]
}

fn main() {
  let all = sum(get_v_arr());
}
```

<v-click>

* Interestingly `get_v_arr` works, even though the literal looks like it would
  only exist temporarily
* Literals actually exist during the entire lifetime of the program
* `&'static` here is used to indicate that this slice will exist the entire
  lifetime of the program

</v-click>
---

# Strings
We have already seen the `String` type being used before, but let's dive a
little deeper

* Strings are used to represent text
* In Rust they are always valid UTF-8
* Their data is stored on the heap
* A String is almost the same as `Vec<u8>` with extra checks to prevent
  creating invalid text
<!--
- We store data on the heap so we can easily have strings of variable sizes
  and grow and shrink them as needed when they are modified.
- In general we really don't care about the exact length of the string
-->

---

# Strings
Let's take a look at some strings

```rust
fn main() {
  let s = String::from("Hello world\nSee you!");
  println!("{:?}", s.split_once(" "));
  println!("{}", s.len());
  println!("{:?}", s.starts_with("Hello"));
  println!("{}", s.to_uppercase());
  for line in s.lines() {
    println!("{}", line);
  }
}
```

---

# String literals
We have already seen string literals being used while constructing a string.
The string literal is what arrays are to vectors

```rust
fn main() {
  let s1 = "Hello world";
  let s2 = String::from("Hello world");
}
```

---

# String literals
We have already seen string literals being used while constructing a string.
The string literal is what arrays are to vectors

```rust
fn main() {
  let s1: &'static str = "Hello world";
  let s2: String = String::from("Hello world");
}
```

* `s1` is actually a slice, a string slice

---

# String literals
We have already seen string literals being used while constructing a string.
The string literal is what arrays are to vectors

```rust
fn main() {
  let s1: &str = "Hello world";
  let s2: String = String::from("Hello world");
}
```

* `s1` is actually a slice, a string slice

---

# `str` - the string slice
It should be possible to have a reference to part of a string. But what is it?

* Not `[u8]`: not every sequence of bytes is valid UTF-8
* Not `[char]`: we could not create a slice from a string since it is stored as
  UTF-8 encoded bytes
* We introduce a new special kind of slice: `str`
* For string slices we do not use brackets!

---

# `str`, `String`, `[T; N]`, `Vec`

| Static   | Dynamic  | Borrowed |
|----------|----------|----------|
| `[T; N]` | `Vec<T>` | `&[T]`   |
| -        | `String` | `&str`   |

* There is no static variant of str
* This would only be useful if we wanted strings of an exact length
* But just like we had the static slice literals, we can use `&'static str`
  literals for that instead!

---

# `String` or `str`
When do we use `String` and when do we use `str`?

```rust
fn string_len(data: &String) -> usize {
  data.len()
}
```

---

# `String` or `str`
When do we use String and when do we use str?

```rust
fn string_len(data: &str) -> usize {
  data.len()
}
```

* Prefer `&str` over `String` whenever possible
* If you need to mutate a string you might try `&mut str`, but you cannot change a slice's length
* Use `String` or `&mut String` if you need to fully mutate the string
