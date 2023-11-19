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
| **`64`**    | `202.26 us` (✅ **1.00x**)  |
| **`128`**   | `354.08 us` (✅ **1.00x**)  |
| **`256`**   | `665.54 us` (✅ **1.00x**)  |
| **`512`**   | `1.12 ms` (✅ **1.00x**)    |
| **`1024`**  | `2.00 ms` (✅ **1.00x**)    |
| **`2048`**  | `3.94 ms` (✅ **1.00x**)    |
| **`4096`**  | `7.69 ms` (✅ **1.00x**)    |
| **`8192`**  | `16.13 ms` (✅ **1.00x**)   |
| **`16384`** | `34.01 ms` (✅ **1.00x**)   |
| **`32768`** | `74.65 ms` (✅ **1.00x**)   |

### Polynomial Multiplication Benchmarks

|             | `NTT-Based`               | `Brute-Force`                      |
|:------------|:--------------------------|:---------------------------------- |
| **`64`**    | `1.18 ms` (✅ **1.00x**)   | `48.62 us` (🚀 **24.21x faster**)   |
| **`128`**   | `2.30 ms` (✅ **1.00x**)   | `198.30 us` (🚀 **11.59x faster**)  |
| **`256`**   | `3.54 ms` (✅ **1.00x**)   | `766.71 us` (🚀 **4.62x faster**)   |
| **`512`**   | `6.50 ms` (✅ **1.00x**)   | `3.11 ms` (🚀 **2.09x faster**)     |
| **`1024`**  | `12.43 ms` (✅ **1.00x**)  | `12.34 ms` (✅ **1.01x faster**)    |
| **`2048`**  | `24.68 ms` (✅ **1.00x**)  | `49.90 ms` (❌ *2.02x slower*)      |
| **`4096`**  | `51.36 ms` (✅ **1.00x**)  | `200.91 ms` (❌ *3.91x slower*)     |
| **`8192`**  | `106.21 ms` (✅ **1.00x**) | `803.87 ms` (❌ *7.57x slower*)     |
| **`16384`** | `226.19 ms` (✅ **1.00x**) | `3.24 s` (❌ *14.31x slower*)       |
| **`32768`** | `467.75 ms` (✅ **1.00x**) | `12.75 s` (❌ *27.25x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

