# Benchmarks

## Table of Contents

- [Overview](#overview)
- [Benchmark Results](#benchmark-results)
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

### Polynomial Multiplication Benchmarks

|             | `NTT-Based`               | `Concrete-NTT`                       |
|:------------|:--------------------------|:------------------------------------ |
| **`64`**    | `982.22 us` (✅ **1.00x**) | `84.31 ns` (🚀 **11650.05x faster**)  |
| **`128`**   | `1.18 ms` (✅ **1.00x**)   | `149.42 ns` (🚀 **7901.64x faster**)  |
| **`256`**   | `2.03 ms` (✅ **1.00x**)   | `286.35 ns` (🚀 **7091.84x faster**)  |
| **`512`**   | `2.75 ms` (✅ **1.00x**)   | `600.13 ns` (🚀 **4580.51x faster**)  |
| **`1024`**  | `4.99 ms` (✅ **1.00x**)   | `1.32 us` (🚀 **3779.10x faster**)    |
| **`2048`**  | `9.42 ms` (✅ **1.00x**)   | `2.74 us` (🚀 **3434.03x faster**)    |
| **`4096`**  | `18.04 ms` (✅ **1.00x**)  | `5.84 us` (🚀 **3089.55x faster**)    |
| **`8192`**  | `35.30 ms` (✅ **1.00x**)  | `12.27 us` (🚀 **2877.03x faster**)   |
| **`16384`** | `72.15 ms` (✅ **1.00x**)  | `25.50 us` (🚀 **2829.18x faster**)   |
| **`32768`** | `155.51 ms` (✅ **1.00x**) | `53.88 us` (🚀 **2886.10x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

