*This exercise is adapted from the [serde_lifetimes exercise](https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc) by Ferrous Systems*

Open `exercises/B/1-my-serde-app/src/main.rs`. In there, you'll find some Rust code we will do this exercise with.

We used `todo!()` macros to mark places where you should put code to make the program run. Look at the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/#functions) api for help.

<details>
    <summary><b>Hint</b></summary>
Serde comes with two traits: `Serializable` and `Deserializable`. These traits can be `derive` d for your `struct` or `enum` types. Other `serde-*` crates use these traits to convert our data type from and to corresponding representation (`serde-json` to JSON, `serde-yaml` to YAML, etc.).
</details>

> ***How come `main` returns an `anyhow::Result<()>`?***
> By having `main` return a result, we can bubble errors up all the way to runtime. You can find more information about it in [Rust By Example](https://doc.rust-lang.org/rust-by-example/error/result.html#using-result-in-main). The `anyhow::Result` is a more flexible type of `Result`, which allows for easy conversion of error types.

> ***What is that `r#"...` thing?***  
> `r` in front of a string literal means it's a "raw" string. Escape sequences (`\n`, `\"`, etc.) don't work, and thus they are very convenient for things like regular expressions, JSON literals, etc.
>
> Optionally `r` can be followed by one or more symbols (like `#` in our case), and then your string ends when there's a closing double quote followed by the same number of the same symbols. This is great for cases when you want to have double quotes inside your string literal. For our example `r#" ... "#` works great for JSON. In rare cases you'd want to put two or more pound signs. Like, when you store CSS color values in your JSON strings:
```rust
// here `"#` would not terminate the string
r##"
    {
        "color": "#ff00ff"
    }
"##
```
