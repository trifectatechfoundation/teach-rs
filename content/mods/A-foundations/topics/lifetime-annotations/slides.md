---
layout: section
---

# Lifetime annotations

---
layout: default
---

# What lifetime?

- References refer to variable
- Variable has a lifetime:
  - Start at declaration
  - End at drop


*Question: Will this compile?*
```rust
/// Return reference to longest of `&str`s
fn longer(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

---
layout: default
---
```rust{all|2}
/// Return reference to longest of `&str`s
fn longer(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

```
   Compiling playground v0.0.1 (/playground)
error[E0106]: missing lifetime specifier
 --> src/lib.rs:2:32
  |
2 | fn longer(a: &str, b: &str) -> &str {
  |              ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
2 | fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
  |          ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `playground` due to previous error
```

---
layout: default
---

# Lifetime annotations

```rust{all|1}
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() > right.len() {
        left
    } else {
        right
    }
}
```

English: 

- Given a lifetime called `'a`,
- `longer` takes two references `left` and `right`
- that live for <ins>at least</ins> `'a`
- and returns a reference that lives for `'a`

*Note: Annotations do NOT change the lifetime of variables! Their scopes do!*

They just provide information for the borrow checker

---
layout: default
---

# Validating boundaries

- Lifetime validation is done within function boundaries
- No information of calling context is used

*Question: Why?*


---
layout: default
---

# Lifetime annotations in types

```rust
/// A struct that contains a reference to a T
pub struct ContainsRef<'r, T> {
  reference: &'r T
}
```

---
layout: default
---

# Lifetime elision
&nbsp;

Q: "Why haven't I come across this before?"<br/>
<v-click>
<div>
A: "Because of lifetime elision!"
</div>
</v-click>
<v-click>
<div>
<br/>
<br/>

## Rust compiler has heuristics for eliding lifetime bounds:
- Each elided lifetime in input position becomes a distinct lifetime parameter.
- If there is exactly one input lifetime position (elided or annotated), that lifetime is assigned to all elided output lifetimes.
- If there are multiple input lifetime positions, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all elided output lifetimes.
- Otherwise, annotations are needed to satisfy compiler
</div>
</v-click>
---
layout: default
---
# Lifetime elision examples

```rust{all|1-2|4-5|7-8|10|12|14-15}
fn print(s: &str);                                      // elided
fn print<'a>(s: &'a str);                               // expanded

fn debug(lvl: usize, s: &str);                          // elided
fn debug<'a>(lvl: usize, s: &'a str);                   // expanded

fn substr(s: &str, until: usize) -> &str;               // elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str;     // expanded

fn get_str() -> &str;                                   // ILLEGAL (why?)

fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL (why?)

fn get_mut(&mut self) -> &mut T;                        // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T;              // expanded
```
