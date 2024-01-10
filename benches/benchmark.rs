use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fast_ntt::{
    ntt::{forward, working_modulus, Constants},
    numbers::BigInt,
    polynomial::{fast_mul, mul_brute, Polynomial, PolynomialFieldElement, PolynomialTrait},
};
use itertools::Itertools;

const deg: usize = 16;

fn bench_mul<T: PolynomialFieldElement>(x: usize, y: usize, c: &Constants<T>) {
    let ONE = T::from_i32(1, BigInt::modulus());
    let a = Polynomial::new(vec![0; x].iter().map(|_| ONE).collect_vec());
    let b = Polynomial::new(vec![0; y].iter().map(|_| ONE).collect_vec());
    let _ = fast_mul(a, b, c);
}

fn bench_mul_brute<T: PolynomialFieldElement>(x: usize, y: usize) {
    let ONE = T::from_i32(1, BigInt::modulus());
    let a = Polynomial::new(vec![0; x].iter().map(|_| ONE).collect_vec());
    let b = Polynomial::new(vec![0; y].iter().map(|_| ONE).collect_vec());
    let _ = mul_brute(a, b);
}

fn bench_forward<T: PolynomialFieldElement>(n: usize, c: &Constants<T>) {
    let ONE = T::from_i32(1, BigInt::modulus());
    let a = Polynomial::new(vec![0; n].iter().map(|_| ONE).collect_vec());
    let _ = forward(a.coef, c);
}

fn criterion_forward(c: &mut Criterion) {
    let mut group = c.benchmark_group("Number-Theoretic Transform Benchmarks");
    (6..deg).for_each(|n| {
        let id = BenchmarkId::new("NTT", 1 << n);
        let c = working_modulus(BigInt::from_usize(n), BigInt::from_usize(2 * n + 1));
        group.bench_with_input(id, &n, |b, n| {
            b.iter(|| bench_forward(black_box(1 << n), black_box(&c)))
        });
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Polynomial Multiplication Benchmarks");

    (6..deg).for_each(|n| {
        let id = BenchmarkId::new("NTT-Based", 1 << n);
        let N = BigInt::from_usize((2 * n).next_power_of_two());
        let M = N << 1 + 1;
        let c = working_modulus(N, M);
        group.bench_with_input(id, &n, |b, n| {
            b.iter(|| bench_mul(black_box(1 << n), black_box(1 << n), black_box(&c)))
        });

        let id = BenchmarkId::new("Brute-Force", 1 << n);
        group.bench_with_input(id, &n, |b, n| {
            b.iter(|| bench_mul_brute::<BigInt>(black_box(1 << n), black_box(1 << n)))
        });
    });
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = criterion_forward, criterion_benchmark
}
criterion_main!(benches);
