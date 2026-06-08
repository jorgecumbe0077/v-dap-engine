v-dap-engine

A lock-free transaction processing engine written in Rust for low-latency blockchain mempools and distributed systems.

The project explores deterministic latency, cache-aware data structures, concurrent message passing, and queue scheduling under high load.

Current Focus

Building production-oriented infrastructure software and generating reproducible performance engineering results through benchmarking and hardware profiling.

Engineering Philosophy

The objective is not to promote theoretical claims, but to produce measurable and reproducible performance results supported by transparent methodology and empirical analysis.

Technical Highlights
Lock-Free Concurrency

Uses crossbeam-channel to reduce synchronization overhead and minimize thread contention.

Weighted Round Robin (WRR)

Implements priority-aware scheduling that reduces tail latency for high-priority traffic while preventing starvation of lower-priority flows.

Cache-Aware Design

Data structures are aligned to cache-line boundaries (#[repr(align(64))]) to reduce false sharing and improve memory locality.

Performance Instrumentation

Benchmarks include throughput analysis, latency percentiles (P50/P95/P99/P99.9), and hardware telemetry collected using Linux perf.

Performance Snapshot
Metric	Value
Throughput	3.12 Mpps
P99 High Priority	168 ms
P99.9 High Priority	168 ms
Producers	4
Documentation
DESIGN.md — Architecture, scheduling model, and engineering trade-offs.
BENCHMARKS.md — Reproducible benchmark methodology and scalability results.
PERF_ANALYSIS.md — CPU profiling, cache behavior, IPC analysis, and bottleneck investigation.
LESSONS_LEARNED.md — Engineering findings and optimization journey.
Areas of Exploration
Blockchain mempools
Post-Quantum Cryptography infrastructure
Low-latency transaction processing
Concurrent systems programming
Cache-efficient software design
High-throughput network services
Long-Term Goal

To contribute to high-performance infrastructure, distributed systems, blockchain, or security engineering teams in the UK, Europe, and the United States.
