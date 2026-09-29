use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use problem_7::find_nth_prime_using_ratio;
use std::hint::black_box;

fn bench_ratio(c: &mut Criterion) {
    let mut group = c.benchmark_group("looping Eratosthenes ratios");
    for nth_prime in [1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::new("Eratosthenes_ratio_2", nth_prime),
            &nth_prime,
            |b, nth_prime| {
                b.iter(|| find_nth_prime_using_ratio(black_box(*nth_prime), black_box(2)))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Eratosthenes_ratio_5", nth_prime),
            &nth_prime,
            |b, nth_prime| {
                b.iter(|| find_nth_prime_using_ratio(black_box(*nth_prime), black_box(5)))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Eratosthenes_ratio_7", nth_prime),
            &nth_prime,
            |b, nth_prime| {
                b.iter(|| find_nth_prime_using_ratio(black_box(*nth_prime), black_box(7)))
            },
        );
    }
    group.finish()
}

criterion_group!(benches, bench_ratio);
criterion_main!(benches);
