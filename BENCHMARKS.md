# Benchmarks

## Table of Contents

- [Overview](#overview)
- [Benchmark Results](#benchmark-results)
    - [Number-Theoretic Transform Benchmarks](#number-theoretic-transform-benchmarks)
    - [Polynomial Multiplication Benchmarks](#polynomial-multiplication-benchmarks)

## Overview

This benchmark comparison report shows the difference in performance between parallel, NTT-based and serial, brute-force 
polynomial multiplication algorithms. Each row entry in the first table is an n-degree forward NTT and each row entry in the second table represents an n-degree polynomial multiplication.

Computer Stats:

```
CPU(s):                          16
Thread(s) per core:              2
Core(s) per socket:              8
Socket(s):                       1
```

## Benchmark Results

### Number-Theoretic Transform Benchmarks

|             | `NTT`                      |
|:------------|:-------------------------- |
| **`64`**    | `187.17 us` (✅ **1.00x**)  |
| **`128`**   | `231.50 us` (✅ **1.00x**)  |
| **`256`**   | `333.26 us` (✅ **1.00x**)  |
| **`512`**   | `623.88 us` (✅ **1.00x**)  |
| **`1024`**  | `951.62 us` (✅ **1.00x**)  |
| **`2048`**  | `1.48 ms` (✅ **1.00x**)    |
| **`4096`**  | `2.78 ms` (✅ **1.00x**)    |
| **`8192`**  | `5.48 ms` (✅ **1.00x**)    |
| **`16384`** | `11.09 ms` (✅ **1.00x**)   |
| **`32768`** | `23.08 ms` (✅ **1.00x**)   |

### Polynomial Multiplication Benchmarks

|             | `NTT-Based`               | `Brute-Force`                      |
|:------------|:--------------------------|:---------------------------------- |
| **`64`**    | `818.69 us` (✅ **1.00x**) | `494.52 us` (✅ **1.66x faster**)   |
| **`128`**   | `1.12 ms` (✅ **1.00x**)   | `1.93 ms` (❌ *1.72x slower*)       |
| **`256`**   | `1.74 ms` (✅ **1.00x**)   | `7.78 ms` (❌ *4.48x slower*)       |
| **`512`**   | `2.69 ms` (✅ **1.00x**)   | `30.35 ms` (❌ *11.30x slower*)     |
| **`1024`**  | `4.33 ms` (✅ **1.00x**)   | `121.49 ms` (❌ *28.05x slower*)    |
| **`2048`**  | `7.47 ms` (✅ **1.00x**)   | `493.59 ms` (❌ *66.07x slower*)    |
| **`4096`**  | `14.23 ms` (✅ **1.00x**)  | `1.98 s` (❌ *139.11x slower*)      |
| **`8192`**  | `31.60 ms` (✅ **1.00x**)  | `7.88 s` (❌ *249.28x slower*)      |
| **`16384`** | `65.51 ms` (✅ **1.00x**)  | `31.46 s` (❌ *480.32x slower*)     |
| **`32768`** | `141.24 ms` (✅ **1.00x**) | `126.02 s` (❌ *892.30x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

