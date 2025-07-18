/// Very naive implementation of FizzBuzz
pub fn fizz_buzz(i: u32) -> String {
    if i % 3 == 0 {
        if i % 5 == 0 {
            "FizzBuzz".to_owned()
        } else {
            "Fizz".to_owned()
        }
    } else if i % 5 == 0 {
        "Buzz".to_owned()
    } else {
        format!("{i}")
    }
}

// TODO Write a unit test, using the contents of `fizzbuzz.out` file
// to compare.
// You can use the `include_str!()` macro to include file
// contents as `&str` in your artifact.
