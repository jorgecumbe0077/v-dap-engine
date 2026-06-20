# v-dap-engine

High Performance Post-Quantum Transaction Processing Pipeline

Rust-based experimental engine for studying the performance
trade-offs between:

- Lock-free transaction processing
- Blockchain mempool architecture
- Post-quantum cryptographic validation


## Motivation

Blockchain infrastructure faces a challenge:

Increasing security through PQC algorithms increases computational cost.

This project investigates:

Can modern concurrent architectures reduce the performance impact?


## Architecture

Transaction Flow:

Network
  |
  v
Ingestion Layer
  |
  v
Lock-Free Queue
  |
  v
PQC Validation Workers
  |
  v
Mempool State


## Technology

Language:
Rust

Concurrency:
crossbeam

Parallel Execution:
rayon

Cryptography:
Dilithium2 PQC


## Results

Baseline Queue:

~10M ops/s


Dilithium2 Validation:

~17K validations/s


Scaling:

4 threads:
3.36x speedup


## Reproducibility

Tested on:

- x86_64 Linux
- Android ARM via Termux


Clone:

git clone ...


Build:

cargo build --release


Run:

cargo run --release


## Documentation

DESIGN.md
BENCHMARKS.md
RESULTS.md
PERF_ANALYSIS.md
