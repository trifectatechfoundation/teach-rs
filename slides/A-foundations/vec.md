---

# Vec: storing more of the same
The vector is an array that can grow

* Compare this to the array we previously saw, which has a fixed size

```rust
fn main() {
  let arr = [1, 2];
  println!("{:?}", arr);

  let mut nums = Vec::new();
  nums.push(1);
  nums.push(2);
  println!("{:?}", nums);
}
```

---

# Vec
Vec is such a common type that there is an easy way to initialize
it with values that looks similar to arrays

```rust
fn main() {
  let mut nums = vec![1, 2];
  nums.push(3);
  println!("{:?}", nums);
}
```

---

# Vec: memory layout
How can a vector grow? Things on the stack need to be of a fixed size

<div style="margin-top: 50px; margin-left:auto; margin-right:auto; display:block; width:50%;">

<LightOrDark>
    <template #dark>
        <img src="/images/A2-vector-rust-dark.svg"/>
    </template>
    <template #light>
        <img src="/images/A2-vector-rust-light.svg"/>
    </template>
</LightOrDark>

</div>

<!--
- A Vec does this by allocating its contents on the heap as opposed to the
  stack-based storage of an array
- Think about what would happen if the capacity is full but we still want to
  add another element
-->

---

# Put it in a box
That pointer from the stack to the heap, how do we create such a thing?

* Boxing something is the way to store a value on the heap
* A `Box` uniquely owns that value, there is no one else that also owns that same
  value
* Even if the type inside the box is `Copy`, the box itself is not, move
  semantics apply to a box.

```rust
fn main() {
  // put an integer on the heap
  let boxed_int = Box::new(10);
}
```
<div style="margin-top: 50px; margin-left:auto; margin-right:auto; display:block; width:50%;">

<LightOrDark>
    <template #dark>
        <img src="/images/A2-box-in-memory-dark.svg"/>
    </template>
    <template #light>
        <img src="/images/A2-box-in-memory-light.svg"/>
    </template>
</LightOrDark>

</div>
