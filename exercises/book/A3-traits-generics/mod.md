# Module A3 - Traits and generics

[Slides](/slides/A3/) (or [pdf](/slides/A3-traits-generics.pdf))

## A3 Local Storage Vec
In this exercise, we'll create a type called `LocalStorageVec`, which is generic list of items that resides either on the stack or the heap, depending on its size. If its size is small enough for items to be put on the stack, the `LocalStorageVec` buffer is backed by an array. `LocalStorageVec` is not only generic over the type  (`T`) of items in the list, but also by the size (`N`) of this stack-located array using a relatively new feature called 'const generics'. Once the `LocalStorageVec` contains more items than fit in the array, a heap based [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) is allocated as space for the items to reside in.

**Within this exercise, the objectives are annotated with a number of stars (⭐), indicating the difficulty. You are likely not to be able to finish all exercises during the turorial session**

**Questions**
1. When is such a data structure more efficient than a standard `Vec`?
2. What are the downsides, compared to just using a `Vec`?

Open the `exercises/A3/2-local-storage-vec` crate. It contains a `src/lib.rs` file, meaning this crate is a library. `lib.rs` contains a number of tests, which can be run by calling `cargo test`. Don't worry if they don't pass or even compile right now: it's your job to fix that in this exercise. Most of the tests are commented out right now, to enable a step-by-step approach. **Before you begin, have a look at the code and the comments in there, they contain various helpful clues.**

### A3.A Defining the type ⭐
Currently, the `LocalStorageVec` `enum` is incomplete. Give it two variants: `Stack` and `Heap`. `Stack` contains two named fields, `buf` and `len`. `buf` will be the array with a capacity to hold `N` items of type `T`; `len` is a field of type `usize` that will denote the amount of items actually stored. The `Heap` variant has an unnamed field containing a `Vec<T>`. If you've defined the `LocalStorageVec` variants correctly, running `cargo test` should output something like

```txt
running 1 test
test test::it_compiles ... ignored, This test is just to validate the definition of `LocalStorageVec`. If it compiles, all is OK

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

This test does (and should) not run, but is just there for checking your variant definition.

<details>
    <summary><b>Hint 1</b></summary>
    You may be able to reverse-engineer the `LocalStorageVec` definition using the code of the `it_compiles` test case.
</details>
<br/>
<details>
    <summary><b>Hint 2 (If you got stuck, but try to resist me for a while)</b></summary>

Below definition works. Read the code comments and make sure you understand what's going on.
```rust
// Define an enum `LocalStorageVec` that is generic over
// type `T` and a constant `N` of type `usize`
pub enum LocalStorageVec<T, const N: usize> {
    // Define a struct-like variant called `Stack` containing two named fields:
    // - `buf` is an array with elements of `T` of size `N`
    // - `len` is a field of type `usize`
    Stack { buf: [T; N], len: usize },
    // Define a tuplle-like variant called `Heap`, containing a single field
    // of type `Vec<T>`, which is a heap-based growable, contiguous list of `T`
    Heap(Vec<T>),
}
```
</details>

### A3.B `impl`-ing `From<Vec<T>` ⭐

Uncomment the test `it_from_vecs`, and add an implementation for `From<Vec<T>>` to `LocalStorageVec<T>`. To do so, copy the following code in your `lib.rs` file and replace the `todo!` macro invocation with your code that creates a heap-based `LocalStorageVec` containing the passed `Vec<T>`.

```rust
impl<T, const N: usize> From<Vec<T>> for LocalStorageVec<T, N> {
    fn from(v: Vec<T>) -> Self {
        todo!("Implement me");
    }
}
```

**Question** 
1. How would you pronounce the first line of the code you just copied in English?*

Run `cargo test` to validate your implementation.

### A3.C `impl LocalStorageVec` ⭐⭐
To make the `LocalStorageVec` more useful, we'll add more methods to it. Create an `impl`-block for `LocalStorageVec`. Don't forget to declare and provide the generic paramereters. For now, to make implementations easier, we will add a bound `T`, requiring that it implements `Copy` and `Default`. First off, uncomment the test called `it_constructs`. Make it compile and pass by creating a associated function called `new` on `LocalStorageVec` that creates a new, empty `LocalStorageVec` instance without heap allocation.

The next methods we'll implement are `len`, `push`, `pop`, `insert`, `remove` and `clear`:
- `len` returns the length of the `LocalStorageVec`
- `push` appends an item to the end of the `LocalStorageVec` and increments its length. Possibly moves the contents to the heap if they no longer fit on the stack.
- `pop` removes an item from the end of the `LocalStorageVec`, optionally returns it and decrements its length. If the length is 0, `pop` returns `None`
- `insert` inserts an item at the given index and increments the length of the `LocalStorageVec`
- `remove` removes an item at the given index and returns it.
- `clear` resets the length of the `LocalStorageVec` to 0.

 Uncomment the corresponding test cases and make them compile and pass. **Be sure to have a look at the methods provided for slices [`[T]`](https://doc.rust-lang.org/std/primitive.slice.html) and [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)** Specifically, `[T]::copy_within` and `Vec::extend_from_slice` can be of use.


### A3.D `Iterator` and `IntoIterator` ⭐⭐
Our `LocalStorageVec` can be used in the real world now, but we still shouldn't be satisfied. There are various traits in the standard library that we can implement for our `LocalStorageVec` that would make users of our crate happy.

First off, we will implement the [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) and [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) traits. Go ahead and uncomment the `it_iters` test case. Let's define a new type:


```rust
pub struct LocalStorageVecIter<T, const N: usize> {
    vec: LocalStorageVec<T, N>,
    counter: usize,
}
```

This is the type we'll implement the `Iterator` trait on. You'll need to specify the item this `Iterator` implementation yields, as well as an implementation for `Iterator::next`, which yields the next item. You'll be able to make this easier by bounding `T` to `Default` when implementing the `Iterator` trait, as then you can use the [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) function to take an item from the `LocalStorageVec` and replace it with the default value for `T`.

Take a look at the list of methods under the ['provided methods' section](https://doc.rust-lang.org/std/iter/trait.Iterator.html). In there, lots of useful methods that come free with the implementation of the `Iterator` trait are defined, and implemented in terms of the `next` method. Knowing in the back of your head what methods there are, greatly helps in improving your efficiency in programming with Rust. Which of the provided methods can you override in order to make the implementation of `LocalStorageVecIter` more efficient, given that we can access the fields and methods of `LocalStorageVec`?

Now to instantiate a `LocalStorageVecIter`, implement the [`IntoIter`] trait for it, in such a way that calling `into_iter` yields a `LocalStorageVecIter`.

### A3.E `AsRef` and `AsMut` ⭐⭐
[`AsRef`](https://doc.rust-lang.org/std/convert/trait.AsRef.html) and [`AsMut`](https://doc.rust-lang.org/std/convert/trait.AsMut.html) are used to implement *cheap* reference-to-reference coercion. For instance, our `LocalStorageVec<T, N>` is somewhat similar to a slice `&[T]`, as both represent a contiguous series of `T` values. This is true whether the `LocalStorageVec` buffer resides on the stack or on the heap. 

Uncomment the `it_as_refs` test case and implement `AsRef<[T]>` and `AsMut<[T]>`.

<details>
    <summary><b>Hint</b></summary>
    Make sure to take into account the value of `len` for the `Stack` variant of `LocalStorageVec` when creating a slice.
</details>

### A3.F `Index` ⭐⭐
To allow users of the `LocalStorageVec` to read items or slices from its buffer, we can implement the [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html) trait. This trait is generic over the type of the item used for indexing. In order to make our `LocalStorageVec` versatile, we should implement: 

- `Index<usize>`, allowing us to get a single item by calling `vec[1]`;
- `Index<RangeTo<usize>>`, allowing us to get the first `n` items (excluding item `n`) by calling `vec[..n]`;
- `Index<RangeFrom<usize>>`, allowing us to get the last `n` items by calling `vec[n..]`;
- `Index<Range<usize>>`, allowing us to get the items between `n` and `m` items (excluding item `m`) by calling `vec[n..m]`;

Each of these implementations can be implemented in terms of the `as_ref` implementation, as slices `[T]` all support indexing by the previous types. That is, `[T]` also implements `Index` for those types. Uncomment the `it_indexes` test case and run `cargo test` in order to validate your implementation.

### A3.G Removing bounds ⭐⭐
When we implemented the borrowing `Iterator`, we saw that it's possible to define methods in separate `impl` blocks with different type bounds. Some of the functionality you wrote used the assumption that `T` is both `Copy` and `Default`. However, this means that each of those methods are only defined for `LocalStorageVec`s containing items of type `T` that in fact do implement `Copy` and `Default`, which is not ideal. How many methods can you rewrite having one or both of these bounds removed?


#### A3.H Borrowing `Iterator` ⭐⭐⭐
We've already got an iterator for `LocalStorageVec`, though it has the limitation that in order to construct it, the `LocalStorageVec` needs to be consumed. What if we only want to iterate over the items, and not consume them? We will need another iterator type, one that contains an immutable reference to the `LocalStorageVec` and that will thus need a lifetime annotation. Add a method called `iter` to `LocalStorageVec` that takes a shared `&self` reference, and instantiates the borrowing iterator. Implement the `Iterator` trait with the appropriate `Item` reference type for your borrowing iterator. To validate your code, uncomment and run the `it_borrowing_iters` test case.

Note that this time, the test won't compile if you require the items of `LocalStorageVec` be `Copy`! That means you'll have to define `LocalStorageVec::iter` in a new `impl` block that does not put this bound on `T`:

```rust
impl<T: Default + Copy, const N: usize> LocalStorageVec<T, N> {
    // Methods you've implemented so far
}

impl<T: const N: usize> LocalStorageVec<T, N> {
    pub fn iter(&self) -> /* TODO */
}
```

Defining methods in separate `impl` blocks means some methods are not available for certain instances of the generic type. In our case, the `new` method is only available for `LocalStorageVec`s containing items of type `T` that implement both `Copy` and `Default`, but `iter` is available for all `LocalStorageVec`s.

### A3.I Generic `Index` ⭐⭐⭐⭐
You've probably duplicated a lot of code in the last exercise. We can reduce the boilerplate by defining an empty trait:

```rust
trait LocalStorageVecIndex {}
```

First, implement this trait for `usize`, `RangeTo<usize>`, `RangeFrom<usize>`, and `Range<usize>`.

Next, replace the implementations from the previous exercise with a blanket implementation of `Index`. In English:

*"For each type `T`, `I` and constant `N` of type `usize`,*
*implement `Index<I>` for `LocalStorageVec<T, N>`, 
*where `I` implements `LocalStorageVecIndex`*
*and `[T]` implements `Index<I>`"*

If you've done this correctly, `it_indexes` should again compile and pass.

### A3.J `Deref` and `DerefMut` ⭐⭐⭐⭐
The next trait that makes our `LocalStorageVec` more flexible in use are [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) and [`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) that utilize the 'deref coercion' feature of Rust to allow types to be treated as if they were some type they look like. That would allow us to use any [method that is defined on `[T]`](https://doc.rust-lang.org/std/primitive.slice.html) by calling them on a `LocalStorageVec`. Before continueing, read the section ['Treating a Type Like a Reference by Implementing the Deref Trait'](https://doc.rust-lang.org/book/ch15-02-deref.html#treating-a-type-like-a-reference-by-implementing-the-deref-trait) from The Rust Programming Language (TRPL). **Don't confuse deref coercion with any kind of inheritance! Using `Deref` and `DerefMut` for inheritance is frowned upon in Rust.**

Below, an implementation of `Deref` and `DerefMut` is provided in terms of the `AsRef` and `AsMut` implementations. Notice the specific way in which `as_ref` and `as_mut` are called.

```rust
impl<T, const N: usize> Deref for LocalStorageVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        <Self as AsRef<[T]>>::as_ref(self)
    }
}

impl<T, const N: usize> DerefMut for LocalStorageVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        <Self as AsMut<[T]>>::as_mut(self)
    }
}
```

**Question**
- Replacing the implementation of `deref` with `self.as_ref()` results in a stack overflow when running an unoptimized version. Why? (Hint: deref coercion)
