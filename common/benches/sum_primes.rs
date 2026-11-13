use std::hint::black_box;

use common::primes::Primes;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn bench_sum_primes(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum primes comparison");
    for n in [1_000_000] {
        group.bench_with_input(BenchmarkId::new("eratosthenes", n), &n, |b, n| {
            b.iter(|| {
                let temp: u32 = *n / 21;
                let prime_max = temp + temp.ilog2() * 11;
                let primes = u32::get_primes(black_box(prime_max));
                let mut sums = Vec::new();
                let mut curr_sum = 0;
                for p in primes {
                    curr_sum += p;
                    if curr_sum > *n {
                        break;
                    }
                    sums.push(black_box(curr_sum));
                }
            })
        });
        group.bench_with_input(BenchmarkId::new("eratosthenes in place", n), &n, |b, n| {
            b.iter(|| eratosthenes_sum_in_place(black_box(*n)))
        });
        group.bench_with_input(BenchmarkId::new("is prime", n), &n, |b, n| {
            b.iter(|| sum_primes(black_box(*n)))
        });
    }
    group.finish();
}

fn sum_primes(max: u32) -> Vec<u32> {
    let cycle = [4, 2, 4, 2, 4, 6, 2, 6]; // Cycle 8 / 30 that excludes multiples of 2, 3, 5.
    let mut sequence = vec![2, 2 + 3, 2 + 3 + 5, 2 + 3 + 5 + 7];
    let mut sums = 2 + 3 + 5 + 7;
    let mut number = 11; // We already have 2, 3, 5, 7
    let mut i = 1; // 11 is the 2nd element of the cycle.
    loop {
        if number.is_prime() {
            sums += number;
            if sums > max {
                break;
            }
            sequence.push(sums);
        }
        number += cycle[i];
        i = (i + 1) % 8;
    }
    sequence
}

fn eratosthenes_sum_in_place(max: u32) -> Vec<u32> {
    let temp: u32 = max / 21;
    let prime_max = temp + temp.ilog2() * 11;
    let num_elements = (prime_max - 1) / 2;
    let num_elements: usize = num_elements.try_into().expect("Failed to parse to usize");
    let mut is_prime: Vec<bool> = vec![true; num_elements];
    let limit = (prime_max.isqrt() / 2) as usize; // if above cast to usize didn't panic, this won't
    for i in 0..limit {
        if is_prime[i] {
            // number `a` stored in index `i` is `a = 2i + 3`
            // we need `a^2`, which is `4i^2 + 12i + 9`
            // The index for this should be 2x + 3 = 4i^2 + 12i + 9
            // So index is `x = 2i^2 + 6i + 3`
            let start = 2 * i * i + 6 * i + 3;
            // We want to increment by steps of a, which = 2i + 3
            for j in (start..num_elements).step_by(2 * i + 3) {
                is_prime[j] = false;
            }
        }
    }

    let mut prime_sum: Vec<u32> = vec![2];
    let mut curr_sum = 2;
    for (i, is_prime) in is_prime.iter().enumerate() {
        if *is_prime {
            // 2i + 3 = a
            let num = 2 * i + 3;
            curr_sum += num;
            if curr_sum > max as usize {
                break;
            }
            prime_sum.push(curr_sum as u32);
        }
    }
    prime_sum
}

criterion_group!(benches, bench_sum_primes);
criterion_main!(benches);
