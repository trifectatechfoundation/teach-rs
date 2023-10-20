
---
layout: section
---

# Design patterns in Rust

---
layout: default
---

# Why learn design patterns?

- Common problems call for common, tried an tested solutions
- Make crate architecture more clear
- Speed up development
- Rust does some patterns ever-so-slightly differently

*Learning common Rust patterns makes understanding new code easier*

---
layout: default
---

# What we'll do

```rust
const PATTERNS: &[Pattern] = &[
    Pattern::new("Newtype"),
    Pattern::new("RAII with guards"),
    Pattern::new("Typestate"),
    Pattern::new("Strategy"),
];
fn main() {
    for pattern in PATTERNS {
        pattern.introduce();
        pattern.show_example();
        pattern.when_to_use();
    }
}
```

---
layout: statement
---

# 1. The Newtype pattern
a small but useful pattern

---
layout: default
---

# Newtype: introduction
&nbsp;

### Wrap an external type in a new local type

```rust
pub struct Imei(String)
```

That's it!

---
layout: default
---

# Newtype: example

```rust
pub enum ValidateImeiError { /* - snip - */}

pub struct Imei(String);

impl Imei {
    fn validate(imei: &str) -> Result<(), ValidateImeiError> {
        todo!();
    }
}

impl TryFrom<String> for Imei {
    type Error = ValidateImeiError;

    fn try_from(imei: String) -> Result<Self, Self::Error> {
        Self::validate(&imei)?;
        Ok(Self(imei))
    }
}

fn register_phone(imei: Imei, label: String) {
    // We can certain `imei` is valid here
}
```

---
layout: default
---

# Newtype: when to use

Newtype solves some problems:
- Orphan rule: no `impl`s for external `trait`s on external types
- Allow for semantic typing (`url` example from mod B)
- Enforce input validation

---
layout: statement
---

# 2. The RAII guard pattern
More robust resource handling

---
layout: default
---

# RAII Guards: introduction

- Resource Acquisition Is Initialization (?)
- Link acquiring/releasing a resource to the lifetime of a variable
- Guard constructor initializes resource, destructor frees it
- Access resource through the guard

*Do you know of an example?*

---
layout: two-cols
---

# RAII Guards: example

```rust
pub struct Transaction<'c> {
    connection: &'c mut Connection,
    did_commit: bool,
    id: usize,
}

impl<'c> Transaction<'c> {
    pub fn begin(connection: &'c mut Connection)
     -> Self {
        let id = 
            connection.start_transaction();
        Self {
            did_commit: false,
            id,
            connection,
        }
    }

    pub fn query(&self sql: &str) { /* - snip - */}

    pub fn commit(self) {
        self.did_commit = true;
    }
}
```
::right::
<div style="padding-left:10px; padding-top: 50px;">

```rust
impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if self.did_commit {
            self
                .connection
                .commit_transaction(self.id);
            
        } else {
            self
                .connection
                .rollback_transaction(self.id);
        }
    }
}
```
</div>

---
layout: default
---

# RAII Guards: when to use

- Ensure a resource is freed at some point
- Ensure invariants hold while guard lives

---
layout: statement
---

# 3. The Typestate pattern
Encode state in the type

---
layout: default
---

# Typestate: introduction

- Define uninitializable types for each state of your object
```rust
pub enum Ready {} // No variants, cannot be initialized
```
<v-click>

- Make your type generic over its state using `std::marker::PhantomData`
- Implement methods only for relevant states
- Methods that update state take owned `self` and return instance with new state

*👻 `PhantomData<T>` makes types act like they own a `T`, and takes no space*
</v-click>
---
layout: two-cols
---

# Typestate: example

```rust
pub enum Idle {} // Nothing to do
pub enum ItemSelected {} // Item was selected
pub enum MoneyInserted {} // Money was inserted

pub struct CoffeeMachine<S> {
    _state: PhantomData<S>,
}

impl<CS> CoffeeMachine<CS> {
    /// Just update the state
    fn into_state<NS>(self) -> CoffeeMachine<NS> {
        CoffeeMachine {
            _state: PhantomData,
        }
    }
}

impl CoffeeMachine<Idle> {
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }
}
```

::right::
<div style="padding-left:10px; padding-top: 0;">

```rust
impl CoffeeMachine<Idle> {
    fn select_item(self, item: usize) -> CoffeeMachine<ItemSelected> {
        println!("Selected item {item}");
        self.into_state()
    }
}

impl CoffeeMachine<ItemSelected> {
    fn insert_money(self) -> CoffeeMachine<MoneyInserted> {
        println!("Money inserted!");
        self.into_state()
    }
}

impl CoffeeMachine<MoneyInserted> {
    fn make_beverage(self) -> CoffeeMachine<Idle> {
        println!("There you go!");
        self.into_state()
    }
}
```
</div>

---
layout: default
---

# Typestate: when to use

- If your problem is like a state machine
- Ensure *at compile time* that no invalid operation is done

---
layout: statement
---

# 4. The Strategy pattern
Select behavior dynamically

---
layout: default
---

# Strategy: introduction

- Turn set of behaviors into objects
- Make them interchangeble inside context
- Execute strategy depending on input

*Trait objects work well here!*

---
layout: two-cols
---

# Strategy: example

```rust

trait PaymentStrategy {
    fn pay(&self);
}

struct CashPayment;
impl PaymentStrategy for CashPayment {
    fn pay(&self) {
        println!("🪙💸");
    }
}

struct CardPayment;
impl PaymentStrategy for CardPayment {
    fn pay(&self) {
        println!("💳");
    }
}
```
::right::

<div style="padding-left:10px; padding-top: 50px;">

```rust

fn main() {
    let method: &str 
        = todo!("Read input");
    let strategy: &dyn PaymentStrategy 
        = match method {
        "card" => &CardPayment,
        "cash" => &CashPayment,
        _ => panic!("Oh no!"),
    };
    strategy.pay();
}
```

</div> 

---
layout: default
---

# Strategy: when to use

- Switch algorithms based on some run-time parameter (input, config, ...)

---
layout: section
---

# Anti-patterns
What *not* to do

---
layout: section
---

# The Deref polymorphism anti-pattern

A common pitfall you'll want to avoid

---
layout: two-cols
---

# Deref polymorphism: Example

```rust
use std::ops::Deref;

struct Animal {
    name: String,
}

impl Animal {
    fn walk(&self) {
        println!("Tippy tap")
    }
    fn eat(&self) {
        println!("Om nom")
    }
    fn say_name(&self) {
        // Animals generally can't speak
        println!("...")
    }
}
```
::right::
```rust
struct Dog {
    animal: Animal
}
impl Dog {
    fn eat(&self) {
        println!("Munch munch");
    }
    fn bark(&self) {
        println!("Woof woof!");
    }
}

impl Deref for Dog {
    type Target = Animal;

    fn deref(&self) -> &Self::Target {
        &self.animal
    }
}

fn main (){ 
    let dog: Dog = todo!("Instantiate Dog");
    dog.bark();
    dog.walk();
    dog.eat();
    dog.say_name();
}
```

---
layout: default
---

# The output

```txt
Woof woof!
Tippy tap
Munch munch
...
```

*Even overloading works!*

---
layout: default
---

# Why is it bad?

- This is no 'real' inheritance: `Dog` is no subtype of `Animal`
- Traits implemented on `Animal` are not implemented on `Dog` automatically
- `Deref` and `DerefMut` are intended 'pointer-to-`T`' to `T` conversions
- Deref coercion by `.` 'converts' `self` from `Dog` to `Animal`
- Rust favours explicit conversions for easier reasoning about code

*It will only add confusion: for OOP programmers it's incomplete, for Rust programmers it is unidiomatic*

## ⚠️ Don't do OOP in Rust!

---
layout: default
---

# What to do instead?

- *Move away from OOP constructs*
- Compose your structs
- Use facade methods
- Use `AsRef` and `AsMut` for explicit conversion

---
layout: default
---

# More anti-patterns

- Forcing dynamic dispatch in libraries
- `clone()` _to satisfy the borrow checker_
- `unwrap()` or `expect()` _to handle conditions that are recoverable or not impossible_

