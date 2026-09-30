#[allow(deprecated)]
use common::primes::{is_prime, is_prime_eratosthenes};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn compare_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("is prime comparison");
    for i in [101, 10_001, 1_000_001] {
        group.bench_with_input(BenchmarkId::new("trial division", i), &i, |b, i| {
            b.iter(|| is_prime(black_box(*i)))
        });
        #[allow(deprecated)]
        group.bench_with_input(BenchmarkId::new("eratosthenes", i), &i, |b, i| {
            b.iter(|| is_prime_eratosthenes(black_box(*i)))
        });
    }
    group.finish();
}

criterion_group!(benches, compare_functions);
criterion_main!(benches);
