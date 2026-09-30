#[allow(deprecated)]
use common::primes::{get_primes, get_primes_slow};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn compare_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("count primes comparison");
    for i in [100, 10_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::new("eratosthenes", i), &i, |b, i| {
            b.iter(|| get_primes(black_box(*i)))
        });
        #[allow(deprecated)]
        group.bench_with_input(BenchmarkId::new("trial division", i), &i, |b, i| {
            b.iter(|| get_primes_slow(black_box(*i)))
        });
    }
    group.finish();
}

criterion_group!(benches, compare_functions);
criterion_main!(benches);
