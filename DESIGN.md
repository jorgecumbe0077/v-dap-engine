# Design Report

## Problem Statement

High-throughput systems frequently suffer from synchronization overhead, cache contention, and queue saturation.

The objective of v-dap-engine is to investigate alternative lock-free architectures capable of maintaining predictable latency under load.

---

## Core Design Principles

### Lock-Free Communication

The system uses Crossbeam MPSC channels instead of shared mutable structures protected by locks.

Benefits:

* Reduced contention.
* Better scalability.
* Simpler ownership model.

---

### Priority Scheduling

Traffic is classified into priority classes.

A Weighted Round Robin scheduler ensures:

* Faster service for critical traffic.
* Fairness for lower-priority traffic.
* No complete starvation.

---

### Cache Awareness

The design attempts to minimize:

* False sharing.
* Cache-line bouncing.
* Excessive synchronization.

Where applicable, structures are aligned to cache-line boundaries.

---

## Current Limitation

The architecture currently uses a single consumer.

This simplifies analysis but creates a serialization bottleneck under high producer counts.

Observed effects:

* Throughput plateau.
* Increased queue residence time.
* Higher tail latency.

---

## Planned Evolution

* Multi-consumer worker pool.
* Queue sharding.
* NUMA-aware partitioning.
* Affinity-based scheduling.
