use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fast_ntt::{numbers::BigInt, polynomial::Polynomial};
use itertools::Itertools;

fn bench_mul(x: usize, y: usize, k: BigInt) {
    let a = Polynomial::new(vec![0; x].iter().map(|_| k.clone()).collect_vec());
    let b = Polynomial::new(vec![0; y].iter().map(|_| k.clone()).collect_vec());
    let _ = a * b;
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_mul");
    let deg = 10;
    (1..deg).for_each(|x| {
        group.bench_function(BenchmarkId::from_parameter(x), |b| {
            b.iter(|| bench_mul(x, x, BigInt::from(1)))
        });
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
