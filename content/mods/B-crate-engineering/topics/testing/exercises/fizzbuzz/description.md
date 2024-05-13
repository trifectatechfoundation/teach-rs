In this exercise, you will practise writing a unit test, and use Rusts benchmarking functionality to help you optimize a [FizzBuzz](https://en.wikipedia.org/wiki/Fizz_buzz) app. You will need [`cargo-criterion`](https://bheisler.github.io/criterion.rs/book/cargo_criterion/cargo_criterion.html), a tool that runs benchmarks and creates nice reports. You can install it by running

```bash
cargo install cargo-criterion --version=1.1.0
```

### #[modmod:exercise_ref].A Testing Fizz Buzz ⭐
Open `#[modmod:exercise_dir]/src/lib.rs`. Create a unit test that verifies the correctness of the `fizz_buzz` function. You can use the [`include_str`](https://doc.rust-lang.org/std/macro.include_str.html) macro to include `#[modmod:exercise_dir]/fizzbuzz.out` as a `&str` into your binary. Each line of `fizzbuzz.out` contains the expected output of the `fizz_buzz` function given the line number as input. You can run the test with

```bash
cargo test
```

By default, Rusts test harness captures all output and discards it, If you like to debug your test code using print statements, you can run

```bash
cargo test -- --nocapture
```

to prevent the harness from capturing output.


### #[modmod:exercise_ref].B Benchmarking Fizz Buzz ⭐⭐
You'll probably have noticed the `fizz_buzz` implementation is not very optimized. We will use `criterion` to help us benchmark `fizz_buzz`. To run a benchmark, run the following command when in the `#[modmod:exercise_dir]/` directory:

```bash
cargo criterion
```

This command will run the benchmarks, and report some statistics to your terminal. It also generates HTML reports including graphs that you can find under `target/criterion/reports`. For instance, `target/criterion/reports/index.html` is a summary of all benchmark. Open it with your browser and have a look.

Your job is to do some optimization of the `fizz_buzz` function, and use `cargo-criterion` to measure the impact of your changes. Don't be afraid to change the signature of `fizz_buzz`, if, for instance, you want to minimize the number of allocations done by this function. However, make sure that the function is able to correctly produce the output. How fast can you FizzBuzz?