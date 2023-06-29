---

# Boxing
There are several reasons to box a variable on the heap

* When something is too large to move around
* We need something that is sized dynamically
* For writing recursive data structures

```rust
struct Node {
  data: Vec<u8>,
  parent: Node,
}
```

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
