
---
layout: section
---

# Smart pointers

---

# Put it in a `Box`
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
<div style="margin-top: 50px; margin-left:auto; margin-right:auto; display:block;">

<LightOrDark>
    <template #dark>
      <div style="padding: 20px; background-color:#1b1b1b; border-radius: var(--slidev-code-radius) !important;">
          <img src="/images/A2-box-in-memory-dark.svg"/>
      </div>
    </template>
    <template #light>
        <div style="padding: 20px; background-color:#F8F8F8; border-radius: var(--slidev-code-radius) !important;">
          <img src="/images/A2-box-in-memory-light.svg"/>
        </div>
    </template>
</LightOrDark>

</div>

---

# Boxing
There are several reasons to box a variable on the heap

* When something is too large to move around
* We need something that is sized dynamically
* For writing recursive data structures

```rust
struct Node {
  data: Vec<u8>,
  parent: Box<Node>,
}
```

<!--
- Allowing arbitrarily large values on the stack would quickly let our
  function calls exhaust the stack limit
- Especially if a move actually would involve memcopying the bits to another
  location in memory that would take way too long
- Of course the main reason that a vector uses the heap is to be able to be
  sized dynamically, but even so, a vector can be large, whereas an array will
  generally always have a limited size
-->

---

# To Do

Issue: [tweedegolf/teach-rs#68](https://github.com/tweedegolf/teach-rs/issues/68)