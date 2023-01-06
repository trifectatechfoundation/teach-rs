//! Make me compile without changing line 9 or moving line 6!

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
