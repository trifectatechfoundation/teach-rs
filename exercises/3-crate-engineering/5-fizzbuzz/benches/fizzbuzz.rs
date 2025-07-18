use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use exercise_b_3::fizz_buzz;

/// Benchmark [fizz_buzz] with several inputs
/// 1 and 2, 16 and 113 are not divisible by 3 nor 5
/// 3 and 42 are divisible by 3
/// 5 and 40 are divisible by 5
/// 15 and 45 are divisible by both 3 and 5
/// Adapted from <https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_with_inputs.html>
fn bench_fizz_buzz(c: &mut Criterion) {
    let mut group = c.benchmark_group("fizzbuzz");
    for input in [1, 2, 3, 5, 15, 16, 40, 42, 45, 113].into_iter() {
        group.bench_with_input(BenchmarkId::from_parameter(input), &input, |b, input| {
            b.iter(|| fizz_buzz(*input))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fizz_buzz);
criterion_main!(benches);
