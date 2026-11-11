use common::primes::Primes;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_prime_factorisation(c: &mut Criterion) {
    let mut group = c.benchmark_group("prime factorisation");
    for i in [1000u64, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::new("vec", i), &i, |b, i| {
            b.iter(|| black_box(i).get_prime_factors())
        });
        group.bench_with_input(BenchmarkId::new("hash set", i), &i, |b, i| {
            b.iter(|| get_prime_factors_hash(black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("hash map", i), &i, |b, i| {
            b.iter(|| black_box(i).get_distinct_prime_factors())
        });
    }
}

fn get_prime_factors_hash(num: u64) -> std::collections::HashSet<u64> {
    let mut num = num;
    let mut factors = std::collections::HashSet::new();
    while num % 2 == 0 {
        num /= 2;
        factors.insert(2);
    }
    let mut i = 3;
    while i * i <= num {
        while num % i == 0 {
            num /= i;
            factors.insert(i);
        }
        i += 2;
    }
    if num > 2 {
        factors.insert(num);
    }
    factors
}

criterion_group!(benches, bench_prime_factorisation);
criterion_main!(benches);
