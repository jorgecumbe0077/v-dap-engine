# v-dap-engine

**High-performance packet and transaction processing engine written in Rust.**

## Overview
`v-dap-engine` is an engineering study focused on low-latency, high-throughput message processing. The project explores the intersection of lock-free architectures, priority-based scheduling, and concurrency, investigating how queue design and thread contention affect real-world performance on modern CPUs.

## Design Philosophy
* **Lock-Free Concurrency**: Utilizes `crossbeam` channels to minimize synchronization overhead.
* **Weighted Round Robin (WRR)**: Ensures faster service for critical traffic while preventing starvation.
* **Cache-Awareness**: Structures are aligned to cache-line boundaries (`#[repr(align(64))]`) to minimize false sharing.

## Performance Snapshot
| Metric | Result (4 Producers) |
| :--- | :--- |
| **Throughput** | 3.12 Mpps |
| **P99 (High Priority)** | 168 ms |

## Documentation
* [DESIGN.md](./DESIGN.md): Architectural decisions.
* [BENCHMARKS.md](./BENCHMARKS.md): Scalability analysis.
* [PERF_ANALYSIS.md](./PERF_ANALYSIS.md): Hardware profiling (IPC, cache misses).
* [LESSONS_LEARNED.md](./LESSONS_LEARNED.md): Key insights.
