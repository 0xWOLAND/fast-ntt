# fast-ntt

fast-ntt is a Rust package to compute polynomial multiplication in O(nlog(n)) time.

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
![image](https://github.com/0xWOLAND/fast-ntt/assets/41707552/ca52d622-e3d3-43e9-ad1b-3db782fcde18)
| Polynomial Degree | Multiplication Time |
|-------------------|---------------------|
| 0 x 0             | 132.95 µs           |
| 1 x 1             | 312.11 µs           |
| 2 x 2             | 540.28 µs           |
| 3 x 3             | 541.64 µs           |
| 4 x 4             | 1.6425 ms           |
| 5 x 5             | 1.7374              |
| 6 x 6             | 1.7231              |
| 7 x 7             | 1.6463              |
| 8 x 8             | 2.3938              |
