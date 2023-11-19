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
| 64                | 7.4158 ms | 153.32 µs   |
| 128               | 13.675 ms | 617.36 µs   |
| 256               | 25.056 ms | 2.4731 ms   |
| 512               | 54.627 ms | 9.9661 ms   |
| 1024              | 91.212 ms | 39.673 ms   |
| 2048              | 189.84 ms | 157.35 ms   |
| 4096              | 407.08 ms | 631.73 ms   |
| 8192              | 858.76 s  | 2.5609 s    |
| 16384             | 1.7593 s  | 10.222 s    |
| 32768             | 3.7149 s  | 41.119 s    |
```
