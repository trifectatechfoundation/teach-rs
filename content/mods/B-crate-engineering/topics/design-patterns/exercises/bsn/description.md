The BSN (Burgerservicennummer) is a Dutch personal identification number that somewhat resembles the US Social Security Number in its use.
The BSN is a number that adheres to some rules.
In this exercise, we will create a Rust type that guarantees that it represents a valid BSN.


# #[modmod:exercise_ref].A Newtype ⭐⭐
In this part we will implement the BSN number validation, as well as a fallible constructor.

A BSN is valid if and only if it matches the following criteria:

- It consists of 8 or 9 digits
- It passes a variant of the 11 check ([elfproef (Dutch)](https://nl.wikipedia.org/wiki/Elfproef)):

For 8-digit BSNs, we concatenate a `0` to the end. The digits of the number are labeled as  `ABCDEFGHI`.
For example: for BSN `123456789`, `A = 1`, `B = 2`, `C = 3`, and so forth until `I`.

Then, `(9 × A) + (8 × B) + (7 × C) + (6 × D) + (5 × E) + (4 × F) + (3 × G) + (2 × H) + (-1 × I)` must be a multiple of 11

Open `exercises/B/4-bsn` in your editor. You'll find the scaffolding code there, along with two files:
- `valid_bsns.in` containing a list of valid BSNs
- `invalid_bsns.in` containing a list of invalid BSNs.

In `src/lib.rs`, implement `Bsn::validate` to make the `test_validation` test case pass.
Implement `Bsn::try_from_string` as well.
To try just the `test_validation` test case, run:
```
cargo test -- test_validation
```

# #[modmod:exercise_ref].B Visitor with Serde ⭐⭐⭐
Next up is implementing the `serde::Serialize` and `serde::Deserialize` traits, to support serialization and deserialization of `Bsn`s.
In this case, simply deriving those traits won't suffice, as we want to represent the `BSN` as a string after serialization.
We also want to deserialize strings directly into `Bsn`s, while still upholding the guarantee that an instantiated `Bsn` represents a valid BSN.
Therefore, you have to incorporate `Bsn::validate` into the implementation of the deserialization visitor.

More information on implementing the traits:
- `serde::Serialize`: https://serde.rs/impl-serialize.html
- `serde::Deserialize`: https://serde.rs/impl-deserialize.html

If everything works out, all tests should pass.
