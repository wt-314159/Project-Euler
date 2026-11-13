#[allow(deprecated)]
use common::primes::Primes;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn compare_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("is prime comparison");
    for i in [101, 10_001, 1_000_001] {
        group.bench_with_input(BenchmarkId::new("trial division", i), &i, |b, i| {
            b.iter(|| black_box(*i).is_prime())
        });
        group.bench_with_input(BenchmarkId::new("cycle division", i), &i, |b, i| {
            b.iter(|| prime(black_box(*i)))
        });
    }
    group.finish();
}

fn prime(number: usize) -> bool {
    for divisor in [2, 3, 5] {
        // Most numbers will be greater than 25.
        if number % divisor == 0 {
            return false;
        }
    }
    let cycle = [4, 2, 4, 2, 4, 6, 2, 6]; // Cycle 8 / 30 that excludes multiples of 2, 3, 5.
    let root = number.isqrt();
    let mut divisor = 7;
    let mut i = 0;
    while divisor <= root {
        if number % divisor == 0 {
            return false;
        }
        divisor += cycle[i];
        i = (i + 1) % 8;
    }
    true
}

criterion_group!(benches, compare_functions);
criterion_main!(benches);
