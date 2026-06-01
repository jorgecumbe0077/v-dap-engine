# Benchmarking Methodology

This document describes the hardware environment, software stack, workload characteristics, and profiling methodology used to evaluate the performance of the v-dap-engine.

## Hardware Environment

### CPU

Output collected using:

```bash
lscpu
```

Hardware Environment
CPU: Intel(R) Core(TM) i5-3335S CPU @ 2.70GHz

Architecture: x86_64

Physical Cores: 4

Threads per core: 1

L1d Cache: 128 KiB (4 instances)

L1i Cache: 128 KiB (4 instances)

L2 Cache: 1 MiB (4 instances)

L3 Cache: 6 MiB (1 instance)

Memory: 8 GB
### Memory

* RAM: 8 GB

## Software Environment
Operating System: Windows 11 / WSL2 (Ubuntu)

Compiler: rustc 1.95.0 (59807616e 2026-04-14)
### Operating System

* Windows 11
* WSL2 Ubuntu

### Compiler

Collected using:

```bash
rustc --version
```

Example:

```text
rustc 1.95.0
```

### Profiling Tools

* Linux perf
* cargo bench
* cargo run --release

## Benchmark Methodology

The benchmark compares:

1. DashMap (lock-based synchronization)
2. Crossbeam MPSC (lock-free message passing)

### Workload

* Producers: 4
* Consumers: 1
* Total Operations: 10,000,000

Each producer generates messages concurrently while a dedicated consumer processes incoming events.

## Profiling Commands

Basic execution:

```bash
cargo run --release
```

Hardware counters:

```bash
perf stat -d ./target/release/performance_benchmark
```

Custom counters:

```bash
perf stat \
-e instructions,cycles,cache-misses,branch-misses,page-faults \
./target/release/performance_benchmark
```

## Metrics

### IPC (Instructions Per Cycle)

Measures CPU execution efficiency.

Higher values indicate better utilization of the execution pipeline.

### L1 Cache Miss Rate

Measures how often data is not found in the L1 cache.

Lower values generally indicate better memory locality.

### CPU Migrations

Counts thread movement between CPU cores.

Lower values improve cache stability.

### Context Switches

Measures operating-system scheduling activity.

High values may indicate synchronization overhead.

## Limitations

Benchmarks were executed inside WSL2.

Results should be interpreted primarily as comparative measurements between implementations rather than absolute hardware limits.

## Reproducibility

All benchmarks were executed using release builds:

```bash
cargo build --release
```

Source code and benchmark configurations are available within this repository.
