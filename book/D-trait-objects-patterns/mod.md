# Module D - Trait objects and Rust patterns

[Slides](/slides/D/) (or [pdf](/slides/D-trait-objects-patterns.pdf))

## D.1 BSN
The BSN (Burgerservicennummer) is a Dutch personal identification number that somewhat resembles the US Social Security Number in its use.
The BSN is a number that adheres to some rules.
In this exercise, we will create a Rust type that guarantees that it represents a valid BSN.


## D.1.A Newtype ⭐⭐
In this part we will implement the BSN number validation, as well as a fallible constructor.

A BSN is valid if and only if it matches the following criteria:

- It consists of 8 or 9 digits
- It passes a variant of the 11 check ([elfproef (Dutch)](https://nl.wikipedia.org/wiki/Elfproef)):

For 8-digit BSNs, we concatenate a `0` to the end. The digits of the number are labeled as  `ABCDEFGHI`.
For example: for BSN `123456789`, `A = 1`, `B = 2`, `C = 3`, and so forth until `I`.

Then, `(9 × A) + (8 × B) + (7 × C) + (6 × D) + (5 × E) + (4 × F) + (3 × G) + (2 × H) + (-1 × I)` must be a multiple of 11

Open `exercises/D/1-bsn` in your editor. You'll find the scaffolding code there, along with two files:
- `valid_bsns.in` containing a list of valid BSNs
- `invalid_bsns.in` containing a list of invalid BSNs.

In `src/lib.rs`, implement `Bsn::validate` to make the `test_validation` test case pass.
Implement `Bsn::try_from_string` as well.
To try just the `test_validation` test case, run:
```
cargo test -- test_validation
```

## D.1.A Visitor with Serde ⭐⭐⭐
Next up is implementing the `serde::Serialize` and `serde::Deserialize` traits, to support serialization and deserialization of `Bsn`s.
In this case, simply deriving those traits won't suffice, as we want to represent the `BSN` as a string after serialization.
We also want to deserialize strings directly into `Bsn`s, while still upholding the guarantee that an instantiated `Bsn` represents a valid BSN.
Therefore, you have to incorporate `Bsn::validate` into the implementation of the deserialization visitor.

More information on implementing the traits:
- `serde::Serialize`: https://serde.rs/impl-serialize.html
- `serde::Deserialize`: https://serde.rs/impl-deserialize.html

If everything works out, all tests should pass.

## D.2 Typestate 3D Printer ⭐⭐
An imaginary 3D printer uses filament to create all kinds of things.
Its states can be represented with the following state diagram:

```
                   ┌─────────────────┐
                   │                 │
                   │                 │   Reset
                   │      Idle       │◄────────────────────────────┐
         ┌────────►│                 │                             │
         │         │                 │                             │
         │         │                 │                             │
         │         └────────┬────────┘                             │
         │                  │                                      │
         │                  │                                      │
         │                  │ Start                                │
         │                  │                                      │
         │                  ▼                                      │
         │         ┌─────────────────┐                    ┌────────┴────────┐
         │         │                 │                    │                 │
         │         │                 │   Out of filament  │                 │
Product  │         │    Printing     ├──────────────────► │      Error      │
retrieved│         │                 │                    │                 │
         │         │                 │                    │                 │
         │         │                 │                    │                 │
         │         └────────┬────────┘                    └─────────────────┘
         │                  │
         │                  │ Product ready
         │                  │
         │                  ▼
         │         ┌─────────────────┐
         │         │                 │
         │         │                 │
         │         │  Product Ready  │
         └─────────┤                 │
                   │                 │
                   │                 │
                   └─────────────────┘
```

The printer boots in Idle state. Once a job is started, the printer enters the Printing state.
In printing state, it keeps on printing the product until either it is ready or the printer is out of filament.
If the printer is out of filament, the printer goes into Error state, which it can only come out of upon device reset.
If the product is ready, the printer goes to Product Ready state, and once the user retrieves the product, the printer goes back to Idle.

The printer can be represented in Rust using the typestate pattern as described during the lecture. This allows you to write a simple 3D printer driver. In `exercises/D/2-3d-printer`, a `Printer3D` struct is instantiated. Add methods corresponding to each of the traits, that simulate the state transitions by printing the state. A method simulating checking if the printer is out of filament is provided.

Of course, to make the printer more realistic, you can add more states and transitions.

# D.3 Dynamic deserialization ⭐⭐
In this exercise, you'll work with dynamic dispatch to deserialize with `serde_json` or `serde_yaml`, depending on the file extension. The starter code is in `exercises/D/3-config-reader`. Fix the todo's in there.

To run the program, you'll need to pass the file to deserialize to the binary, not to Cargo. To do this, run
```bash
cargo run -- <FILE_PATH>
```

Deserializing both `config.json` and `config.yml` should result in the `Config` being printed correctly.
