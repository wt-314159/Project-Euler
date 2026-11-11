use common::squares::Squares;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_get_squares(c: &mut Criterion) {
    let mut group = c.benchmark_group("get squares comparison");
    for n in [100usize, 10_000, 1_000_000] {
        let count = n.isqrt();
        group.bench_with_input(BenchmarkId::new("get_n_squares", n), &count, |b, count| {
            b.iter(|| usize::get_n_squares(black_box(*count)))
        });
        group.bench_with_input(BenchmarkId::new("get_squares_to", n), &n, |b, n| {
            b.iter(|| usize::get_squares_to(black_box(*n)))
        });
        group.bench_with_input(BenchmarkId::new("get_squares_take_while", n), &n, |b, n| {
            b.iter(|| get_squares_take_while(black_box(*n)))
        });
    }
}

fn get_squares_take_while(max: usize) -> Vec<usize> {
    (1..).map(|n| n * n).take_while(|s| *s <= max).collect()
}

criterion_group!(benches, bench_get_squares);
criterion_main!(benches);
