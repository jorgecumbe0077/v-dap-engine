# PERF_ANALYSIS.md

# CPU Performance Analysis

This document analyzes low-level CPU telemetry collected during v-dap-engine benchmarking using Linux perf.

---

## Profiling Command

```bash
perf stat -d ./target/release/performance_benchmark
```

The objective was to understand whether performance was limited by:

* Lock contention
* CPU execution resources
* Cache hierarchy
* Memory subsystem
* Branch prediction

---

## Benchmark Configuration

Workload:

* 4 Producer Threads
* 1 Consumer Thread
* 10,000,000 Operations

Architecture:

* Rust Release Build
* Crossbeam MPSC Lock-Free Queue

---

## Collected Metrics

### Execution Time

Lock-Free MPSC:

```text
580 ms
```

DashMap:

```text
1293 ms
```

Observed speedup:

```text
~2.2x faster
```

---

## IPC Analysis

Metric:

```text
Instructions: 5,969,976,136

Cycles: 8,789,278,302

IPC: 0.7
```

Interpretation:

IPC (Instructions Per Cycle) measures how efficiently the CPU executes instructions.

Modern CPUs may reach:

* 2.0 to 4.0 IPC in highly optimized workloads
* Below 1.0 IPC in memory-bound workloads

Observed:

```text
IPC = 0.7
```

This suggests the processor spends significant time waiting for data rather than executing instructions continuously.

Conclusion:

The benchmark is not compute-bound.

The primary bottleneck is likely memory access latency and synchronization overhead.

---

## Cache Analysis

Metric:

```text
L1-dcache-load-misses: 13.9%
```

Interpretation:

Approximately 14% of memory accesses missed the L1 cache.

For a concurrent workload involving:

* Multiple producers
* Shared communication channels
* Frequent memory movement

This miss rate is acceptable but indicates that cache locality is a major factor.

The benchmark appears increasingly memory-sensitive as producer count increases.

---

## Branch Prediction Analysis

Metric:

```text
Branch Misses: 1.0%
```

Interpretation:

A branch miss rate near 1% is very good.

This indicates:

* Predictable execution flow
* Low branch speculation penalty
* Efficient control flow structure

The scheduler logic is not a major performance bottleneck.

---

## Front-End Stall Analysis

Metric:

```text
Stalled Frontend Cycles: 0.99
```

Interpretation:

The instruction pipeline frequently waits before new instructions can be dispatched.

Potential causes:

* Instruction cache pressure
* Memory fetch latency
* Synchronization traffic
* Cache-line transfers between cores

Given the relatively low branch miss rate, memory latency is the most likely explanation.

---

## Scalability Findings

### 4 Producers

Throughput:

```text
3.12 Mpps
```

P99:

```text
168 ms
```

### 16 Producers

Throughput:

```text
2.88 Mpps
```

P99:

```text
784 ms
```

Observation:

Increasing producer count by 4x did not increase throughput.

Instead:

* Throughput decreased
* Queue residence time increased
* Tail latency increased dramatically

This indicates consumer-side saturation.

---

## Architectural Bottleneck

Current topology:

```text
Multiple Producers
        ↓
Crossbeam MPSC
        ↓
Single Consumer
```

The single consumer eventually becomes a serialization point.

At higher producer counts:

* Queue depth grows
* Waiting time increases
* Tail latency expands

The consumer thread becomes the dominant bottleneck.

---

## Engineering Conclusions

The benchmark demonstrates:

1. Lock-free communication significantly outperforms DashMap for this workload.

2. The workload is primarily memory-bound rather than compute-bound.

3. Branch prediction performance is excellent.

4. Tail latency growth is caused by queue saturation.

5. The current throughput ceiling is imposed by the single-consumer architecture.

---

## Future Investigations

Planned experiments:

* Multi-consumer worker pool
* NUMA-aware scheduling
* Thread affinity
* Bounded channels
* Cache-line ownership analysis (perf c2c)
* Atomic ordering optimization
* Queue sharding

```
```
