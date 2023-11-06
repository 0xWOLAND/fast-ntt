use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fast_ntt::{
    ntt::{forward, working_modulus, Constants},
    numbers::BigInt,
    polynomial::Polynomial,
};
use itertools::Itertools;

const deg: usize = 16;

fn bench_mul(x: usize, y: usize, c: &Constants) {
    let ONE = BigInt::from(1);
    let a = Polynomial::new(vec![0; x].iter().map(|_| ONE).collect_vec());
    let b = Polynomial::new(vec![0; y].iter().map(|_| ONE).collect_vec());
    let _ = a.mul(b, c);
}

fn bench_mul_brute(x: usize, y: usize) {
    let ONE = BigInt::from(1);
    let a = Polynomial::new(vec![0; x].iter().map(|_| ONE).collect_vec());
    let b = Polynomial::new(vec![0; y].iter().map(|_| ONE).collect_vec());
    let _ = a.mul_brute(b);
}

fn bench_forward(n: usize, c: &Constants) {
    let ONE = BigInt::from(1);
    let a = Polynomial::new(vec![0; n].iter().map(|_| ONE).collect_vec());
    let _ = forward(a.coef, c);
}

fn criterion_forward(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_forward");
    (6..deg).for_each(|x| {
        group.bench_function(BenchmarkId::from_parameter(x), |b| {
            let c = working_modulus(BigInt::from(x), BigInt::from(2 * x + 1));
            b.iter(|| bench_forward(black_box(1 << x), black_box(&c)))
        });
    });
}

fn criterion_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_mul");
    (6..deg).for_each(|x| {
        group.bench_function(BenchmarkId::from_parameter(x), |b| {
            let N = BigInt::from((2 * x as usize).next_power_of_two());
            let M = N << 1 + 1;
            let c = working_modulus(N, M);
            b.iter(|| bench_mul(black_box(1 << x), black_box(1 << x), black_box(&c)))
        });
    });
    group.finish();
}

fn criterion_brute_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_brute_mul");
    (6..deg).for_each(|x| {
        group.bench_function(BenchmarkId::from_parameter(x), |b| {
            b.iter(|| bench_mul_brute(black_box(1 << x), black_box(1 << x)))
        });
    });
    group.finish();
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = criterion_forward, criterion_mul, criterion_brute_mul
}
criterion_main!(benches);
