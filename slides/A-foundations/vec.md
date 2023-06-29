
---
layout: section
---

# `Vec`

---

# `Vec`: storing more of the same
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

# `Vec`
`Vec` is such a common type that there is an easy way to initialize
it with values that looks similar to arrays

```rust
fn main() {
  let mut nums = vec![1, 2];
  nums.push(3);
  println!("{:?}", nums);
}
```

---

# `Vec`: memory layout
How can a vector grow? Things on the stack need to be of a fixed size

<div style="margin-top: 50px; margin-left:auto; margin-right:auto; display:block; width:50%;">

<LightOrDark>
    <template #dark>
        <div style="padding: 20px; background-color:#1b1b1b; border-radius: var(--slidev-code-radius) !important;">
          <img src="/images/A2-vector-rust-dark.svg"/>
        </div>
    </template>
    <template #light>
        <div style="padding: 20px; background-color:#F8F8F8; border-radius: var(--slidev-code-radius) !important;">
          <img src="/images/A2-vector-rust-light.svg"/>
        </div>
    </template>
</LightOrDark>

</div>

<!--
- A Vec does this by allocating its contents on the heap as opposed to the
  stack-based storage of an array
- Think about what would happen if the capacity is full but we still want to
  add another element
-->

