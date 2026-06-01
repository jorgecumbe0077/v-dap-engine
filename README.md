# v-dap-engine

A high-performance Rust engine focused on low-latency concurrent processing, lock-free communication patterns, and cache-efficient data structures.

## Overview

v-dap-engine is an experimental systems-engineering project exploring the performance characteristics of modern multi-core CPUs under high-concurrency workloads.

The project investigates:

* Lock-free communication patterns
* Cache locality optimization
* Queue-based architectures
* Memory-access behavior
* Throughput and tail-latency analysis
* Performance benchmarking using Linux perf

## Key Results

### Lock-Based vs Lock-Free Benchmark

Workload:

* 4 Producers
* 1 Consumer
* 10 Million Operations

Results:

| Architecture               | Time  |
| -------------------------- | ----- |
| DashMap (Lock-Based)       | 1.29s |
| Crossbeam MPSC (Lock-Free) | 0.58s |

Observed speedup:

* ~2.2x improvement using lock-free message passing

### Hardware Telemetry

Measured using Linux perf:

* IPC: 0.7
* L1 Cache Miss Rate: 13.9%
* Branch Miss Rate: 1.0%
* CPU Migrations: 0
* Context Switches: 0

These measurements are used to study how memory locality and synchronization mechanisms affect throughput.

## Technical Focus

The project currently explores:

* Rust concurrency primitives
* Crossbeam channels
* Lock-free pipelines
* Cache-aware data layouts
* Atomic operations
* Multi-threaded benchmarking

## Roadmap

* Transaction processing workloads
* Cache-line aware structures
* NUMA-aware experiments
* PQC admission pipeline simulations
* Extended hardware profiling with perf c2c

## Disclaimer

This repository is an engineering research project and benchmarking platform. Performance results are workload-dependent and should not be interpreted as universal improvements for all applications.
