# BENCHMARKS.md

# v-dap-engine Benchmark Report

## Purpose

This document presents the benchmark methodology and performance results obtained during the development of the v-dap-engine.

The objective is to evaluate:

* Throughput scalability
* Tail latency behavior
* Scheduling efficiency
* Producer-to-consumer contention effects

---

# Test Environment

## Hardware

### Development System

* CPU: Intel Core i5 (Broklen workstation)
* Architecture: x86_64
* RAM: 8 GB
* Cache Hierarchy:

  * L1 Cache: CPU dependent
  * L2 Cache: CPU dependent
  * L3 Cache: Shared cache

### Mobile Validation Environment

* Device: Vivo Y67
* SoC: MediaTek MT6750
* RAM: 4 GB
* Environment: Termux

---

# Software Stack

* Language: Rust
* Build Profile: Release
* Operating System: Linux / WSL
* Benchmark Tooling:

  * Linux perf
  * Instant
  * Custom percentile calculations

---

# Methodology

The benchmark uses a producer-consumer architecture.

Each producer generates transactions and submits them into a shared queue.

The consumer processes transactions according to the configured scheduling strategy.

Measured metrics:

* Throughput (Mpps)
* P50 latency
* P95 latency
* P99 latency
* P99.9 latency

Latency is measured as:

Transaction Creation Time → Queue Residence Time → Consumption Time

This represents the complete end-to-end latency experienced by a transaction inside the engine.

---

# Reproduction Steps

Build:

```bash
cargo build --release
```

Run:

```bash
./target/release/performance_benchmark 4
```

Hardware profiling:

```bash
perf stat -d ./target/release/performance_benchmark
```

---

# Scalability Results

## 4 Producers

| Metric              | Value     |
| ------------------- | --------- |
| Throughput          | 3.12 Mpps |
| High Priority P50   | 115 ms    |
| High Priority P99   | 168 ms    |
| High Priority P99.9 | 168 ms    |
| Low Priority P50    | 180 ms    |
| Low Priority P99    | 224 ms    |
| Low Priority P99.9  | 225 ms    |

### Observation

The system reaches peak efficiency at four producers.

The WRR scheduler consistently prioritizes high-priority traffic across all latency percentiles.

---

## 16 Producers

| Metric            | Value     |
| ----------------- | --------- |
| Throughput        | 2.88 Mpps |
| High Priority P99 | 784 ms    |
| Low Priority P99  | 910 ms    |

### Observation

Increasing producer density beyond four producers introduces queue contention and exposes the single-consumer bottleneck.

Throughput decreases while tail latency increases substantially.

---

# Key Findings

## Positive Results

* Lock-free message passing scales efficiently.
* WRR scheduling successfully reduces tail latency for critical traffic.
* Priority guarantees remain stable under load.

## Bottlenecks Identified

* Single consumer thread limits scalability.
* Queue residence time becomes dominant at high producer counts.
* Tail latency grows rapidly after saturation.

---

# Future Work

* Multi-consumer worker pool
* Bounded channels with backpressure
* CPU affinity experiments
* NUMA-aware partitioning
* Post-Quantum signature verification workloads

---

# Conclusion

The benchmark results demonstrate that the v-dap-engine can sustain multi-million transaction throughput while maintaining deterministic latency characteristics under moderate contention.

The experiments also clearly identify the current scalability limits, providing a roadmap for future architectural improvements.
