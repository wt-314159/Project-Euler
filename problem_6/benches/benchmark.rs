use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use problem_6::{brute_force_sum_of_squares, calc_sum_of_squares};
use std::hint::black_box;

fn bench_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum of squares");
    for i in [100, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::new("brute force", i), &i, |b, i| {
            b.iter(|| brute_force_sum_of_squares(black_box(*i)))
        });
        group.bench_with_input(BenchmarkId::new("calculated", i), &i, |b, i| {
            b.iter(|| calc_sum_of_squares(black_box(*i)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_functions);
criterion_main!(benches);
