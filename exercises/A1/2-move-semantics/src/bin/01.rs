fn main() {
    let s0 = String::new();

    let mut s1 = append_to_string(s0);

    println!("{} == `{}`", stringify!(s1), s1);

    s1.push_str("!");

    println!("{} == `{}`", stringify!(s1), s1);
}

fn append_to_string(s: String) -> String {
    let mut s = s;

    s.push_str("Hello");
    s.push_str(" ");
    s.push_str("World");

    s
}
