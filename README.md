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
    let c: Constants<BigInt> = working_modulus(N, M);
    println!("{}", fast_mul(a, b, c));

// Polynomial Differentiation
    let a = Polynomial::new(vec![3, 2, 1].iter().map(|&x| BigInt::from(x)).collect());
    let da = diff(a);
```

## Benchmarks

Generate benchmarks using:

```bash
# If you don't have it already
cargo install cargo-criterion criterion-table -- --cfg

cargo criterion --message-format=json --features parallel | criterion-table > BENCHMARKS.md
```

Benchmarks are also available [here](./BENCHMARKS.md)
