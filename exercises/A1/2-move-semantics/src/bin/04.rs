//! Refactor this code so that instead of passing `vec0` into the `fill_vec` function,
//! the Vector gets created in the function itself and passed back to the main
//! function.

fn main() {
    let s0 = Vec::new();

    let mut s1 = create_string(s0);

    println!("{} == `{}`", stringify!(s1), s1);

    s1.push_str(" World!");

    println!("{} == `{}`", stringify!(s1), s1);
}

///`create_string()` no longer takes `s: String` as argument
fn create_string() -> String {
    let s = String::from("Hello");

    s
}
