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

Generate benchmarks using:

```bash
cargo criterion --message-format=json | criterion-table > BENCHMARKS.md
```

Benchmarks are also available [here](./BENCHMARKS.md)
