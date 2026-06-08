# DESIGN.md

# v-dap-engine Architecture Report

## 1. Overview

The v-dap-engine is a high-performance transaction processing engine written in Rust and designed to explore low-latency infrastructure patterns commonly found in blockchain mempools, financial systems, and network packet processing pipelines.

The project focuses on:

* Lock-free concurrency
* Deterministic latency
* Cache-aware design
* Priority-based scheduling
* Hardware-level performance analysis

The primary objective is to investigate how architectural decisions impact throughput, latency, and scalability under concurrent workloads.

---

## 2. Architectural Goals

The engine was designed around four core goals:

### High Throughput

Maximize the number of transactions processed per second while maintaining predictable performance characteristics.

### Low Tail Latency

Reduce P95, P99, and P99.9 latency, as these metrics typically determine user-perceived responsiveness and infrastructure quality.

### Reproducibility

All performance claims are backed by benchmark results and profiling data available in the repository.

### Simplicity

Avoid unnecessary abstractions and focus on observable performance behavior.

---

## 3. System Architecture

### Producer Layer

Multiple producer threads generate transactions and submit them into the processing pipeline.

Responsibilities:

* Transaction generation
* Traffic simulation
* Priority assignment

### Queue Layer

The queue layer is implemented using `crossbeam-channel`.

Reasons for this choice:

* Lock-free message passing
* Low synchronization overhead
* Mature and widely adopted implementation
* Suitable for producer-consumer workloads

### Scheduling Layer

Transactions are classified into priority classes.

A Weighted Round Robin (WRR) scheduler determines processing order.

Benefits:

* Reduced latency for critical traffic
* Fair resource allocation
* Prevention of low-priority starvation

### Consumer Layer

A dedicated consumer thread processes incoming transactions.

Responsibilities:

* Queue draining
* Scheduling execution
* Latency measurement
* Statistics collection

---

## 4. Cache-Aware Design

Modern CPUs are heavily dependent on cache locality.

To reduce cache inefficiencies, critical structures are aligned to cache-line boundaries:

```rust
#[repr(align(64))]
```

This design aims to:

* Reduce false sharing
* Improve memory locality
* Minimize cache-line bouncing between cores

The optimization becomes increasingly important under multi-threaded workloads.

---

## 5. Lock-Free Concurrency

Traditional synchronization primitives introduce contention when multiple threads attempt to access shared state.

The engine adopts a message-passing architecture instead of shared mutable state whenever possible.

Expected benefits:

* Reduced lock contention
* Improved scalability
* More predictable latency behavior

Benchmark comparisons against lock-based approaches are documented in BENCHMARKS.md.

---

## 6. Memory Management Strategy

The critical path minimizes dynamic memory allocation.

Design principles:

* Fixed-size structures
* Stack-oriented data handling
* Avoid unnecessary heap allocations

The goal is to reduce latency variance introduced by allocator activity.

---

## 7. Performance Metrics

The project tracks:

### Throughput

Measured in:

* Packets per second
* Transactions per second
* Millions of packets per second (Mpps)

### Latency

Measured using percentile analysis:

* P50
* P95
* P99
* P99.9

Tail latency is prioritized over average latency because infrastructure systems are often evaluated based on worst-case behavior.

---

## 8. Observed Bottlenecks

Testing revealed a clear scalability limit.

### Single Consumer Saturation

At moderate producer counts, throughput scales efficiently.

At higher producer counts:

* Queue residence time increases
* Tail latency grows significantly
* Consumer throughput becomes the bottleneck

This behavior is documented in BENCHMARKS.md.

---

## 9. Future Work

### Multi-Consumer Worker Pool

Replace the single consumer architecture with a worker pool.

Expected benefits:

* Higher throughput
* Better scalability under heavy contention

### Bounded Queues

Introduce backpressure mechanisms to prevent unbounded memory growth.

### Transaction Validation

Integrate realistic validation stages such as:

* Signature verification
* Hash verification
* State lookup

### PQC Experiments

Evaluate the impact of post-quantum signature verification workloads on latency and throughput.

---

## 10. Conclusion

The v-dap-engine is an experimental infrastructure project focused on understanding the interaction between concurrency, cache locality, scheduling policies, and latency behavior.

The project demonstrates measurable performance characteristics through benchmarking, profiling, and architectural experimentation rather than theoretical claims.
