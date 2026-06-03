# v-dap-engine

High-performance packet and transaction processing engine written in Rust.

## Overview

v-dap-engine is a research and engineering project focused on low-latency, high-throughput message processing using lock-free architectures.

The project investigates how queue design, cache locality, scheduling policies, and thread contention affect real-world performance on modern CPUs.

## Goals

* Study lock-free architectures in Rust.
* Measure scalability under increasing producer contention.
* Analyze tail latency (P95/P99/P99.9).
* Investigate cache behavior and synchronization costs.
* Build a foundation for future transaction-processing and distributed validation systems.

## Architecture

Current prototype:

Producer Threads
↓
Crossbeam MPSC Channel
↓
Priority Scheduler (WRR)
↓
Consumer Worker
↓
Telemetry & Statistics

## Implemented Features

* Lock-free message passing using Crossbeam.
* Weighted Round Robin scheduling.
* Priority-aware packet processing.
* Throughput measurement (Mpps).
* Tail latency analysis (P50/P95/P99/P99.9).
* Scalability benchmarking.

## Key Results

### Lock-Based vs Lock-Free

| Structure      | Time   |
| -------------- | ------ |
| DashMap        | ~1.29s |
| Crossbeam MPSC | ~0.58s |

Workload: 10 million operations.

### Scalability

| Producers | Throughput |
| --------- | ---------- |
| 4         | 3.12 Mpps  |
| 16        | 2.88 Mpps  |

The results reveal a saturation point caused by the single-consumer architecture.

## Repository Structure

README.md

DESIGN.md

BENCHMARKS.md

PERF_ANALYSIS.md

src/

benchmarks/

## Future Work

* Multi-consumer worker pools.
* NUMA-aware scheduling.
* Bounded queues and backpressure.
* CPU affinity experiments.
* PQC transaction validation workloads.

