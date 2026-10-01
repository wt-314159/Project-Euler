use common::triangle_numbers::{find_triangle_number_divisors, find_triangle_number_naive};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn compare_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("triangle number divisors");
    for i in [100, 500] {
        group.bench_with_input(BenchmarkId::new("naive approach", i), &i, |b, i| {
            b.iter(|| find_triangle_number_naive(black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("clever way", i), &i, |b, i| {
            b.iter(|| find_triangle_number_divisors(black_box(*i)))
        });
    }
    group.finish();
}

criterion_group!(benches, compare_functions);
criterion_main!(benches);
