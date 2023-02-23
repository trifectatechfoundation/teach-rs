//! Make me compile without changing line 11, but by moving line 8!
//! If you're curious: the `stringify!` macro transforms tokens to
//! a string. For instance, `stringify!(s1)` yields string `"s1"`.

fn main() {
    let s0 = String::from("Hello");

    let mut s1 = append_to_string(s0);

    // Don't change the following line!
    println!("{} == `{}`", stringify!(s0), s0);

    s1.push('!');

    println!("{} == `{}`", stringify!(s1), s1);
}

fn append_to_string(s: String) -> String {
    let mut s = s;

    s.push_str(" World");

    s
}
