use concrete_ntt::prime32::Plan;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fast_ntt::{
    ntt::{forward, working_modulus, Constants},
    numbers::BigInt,
    polynomial::{fast_mul, mul_brute, Polynomial, PolynomialFieldElement, PolynomialTrait},
};
use itertools::Itertools;

const deg: usize = 16;

fn bench_mul<T: PolynomialFieldElement>(x: usize, c: &Constants<T>) {
    let ONE = T::from_i32(1, BigInt::modulus());
    let a = Polynomial::new(vec![0; x].iter().map(|_| ONE).collect_vec());
    let b = Polynomial::new(vec![0; x].iter().map(|_| ONE).collect_vec());
    let _ = fast_mul(a, b, c);
}

fn bench_concrete<T: PolynomialFieldElement>(x: usize, plan: &Plan) {
    let data = vec![1; x];

    let mut transformed_fwd = data;
    plan.fwd(&mut transformed_fwd);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Polynomial Multiplication Benchmarks");

    (6..deg).for_each(|n| {
        let id = BenchmarkId::new("NTT-Based", 1 << n);
        let N = BigInt::from_usize((2 * n).next_power_of_two());
        let M = N << 1 + 1;
        let c = working_modulus(N, M);
        group.bench_with_input(id, &n, |b, n| {
            b.iter(|| bench_mul(black_box(1 << n), black_box(&c)))
        });

        let id = BenchmarkId::new("Concrete-NTT", 1 << n);

        let N = (1 << n);
        let p = 1062862849;
        let plan = Plan::try_new(N, p).unwrap();
        group.bench_with_input(id, &n, |b, n| {
            b.iter(|| bench_concrete::<BigInt>(black_box(1 << n), black_box(&plan)))
        });
    });
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets =  criterion_benchmark
}
criterion_main!(benches);
