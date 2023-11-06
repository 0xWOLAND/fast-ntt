# fast-ntt

fast-ntt is a Rust package to compute polynomial multiplication in `O(nlog(n))` time.

## Usage

```rust
// Polynomial Addition
    let a = Polynomial::new(vec![1, 2, 3, 4].iter().map(|&x| BigInt::from(x)).collect());
    let b = Polynomial::new(vec![1, 2].iter().map(|&x| BigInt::from(x)).collect());
    println!("{}", a + b);

// Polynomial Multiplication
    let a = Polynomial::new(vec![1, 2, 3].iter().map(|&x| BigInt::from(x)).collect());
    let b = Polynomial::new(
        vec![1, 2, 3, 4]
            .iter()
            .map(|&x| BigInt::from(x))
            .collect(),
    );
    println!("{}", a * b);

// Polynomial Differentiation
    let a = Polynomial::new(vec![3, 2, 1].iter().map(|&x| BigInt::from(x)).collect());
    let da = a.diff();
```

## Benchmarks
```
| Polynomial Degree | FFT-Based | Brute-Force |
|-------------------|-----------|-------------|
| 64                | 6.3443 ms | 49.039 µs   |
| 128               | 14.303 ms | 198.03 µs   |
| 256               | 31.494 ms | 776.95 µs   |
| 512               | 67.613 ms | 3.1632 ms   |
| 1024              | 148.00 ms | 13.152 ms   |
| 2048              | 322.06 ms | 51.614 ms   |
| 4096              | 691.63 ms | 201.23 ms   |
| 8192              | 1.4871 s  | 805.62 ms   |
| 16384             | 3.1461 s  | 3.2964 s    |
| 32768             | 6.7333 s  | 13.054 s    |
```
