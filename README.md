# fast-ntt [![Rust](https://github.com/0xWOLAND/fast-ntt/actions/workflows/rust.yml/badge.svg)](https://github.com/0xWOLAND/fast-ntt/actions/workflows/rust.yml)

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

Computer Stats:

```
CPU(s):                          16
Thread(s) per core:              2
Core(s) per socket:              8
Socket(s):                       1
```

### Number-Theoretic Transform

| Polynomial Degree | NTT       |
| ----------------- | --------- |
| 64                | 201.11 µs |
| 128               | 355.35 µs |
| 256               | 845.66 µs |
| 512               | 1.3013 ms |
| 1024              | 2.1763 ms |
| 2048              | 4.1270 ms |
| 4096              | 7.9391 ms |
| 8192              | 16.674 ms |
| 16384             | 34.160 ms |
| 32768             | 79.303 ms |

### Polynomial Multiplication

| Polynomial Degree | NTT-Based | Brute-Force |
| ----------------- | --------- | ----------- |
| 64                | 1.2677 ms | 50.389 µs   |
| 128               | 2.3206 ms | 196.92 µs   |
| 256               | 3.6952 ms | 777.25 µs   |
| 512               | 6.9324 ms | 3.2495 ms   |
| 1024              | 13.158 ms | 12.777 ms   |
| 2048              | 26.159 ms | 51.868 ms   |
| 4096              | 55.093 ms | 205.77 ms   |
| 8192              | 115.62 ms | 812.68 ms   |
| 16384             | 241.09 ms | 3.2130 s    |
| 32768             | 502.79 ms | 12.959 s    |
