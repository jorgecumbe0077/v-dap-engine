# Benchmark Methodology

## Test Environment

### Hardware

Processor: Intel Core i5

Architecture: x86_64

Memory: 8 GB RAM

Environment: WSL/Linux

### Software

Rust: rustc --version

Profiler: Linux perf

Build Mode:

cargo build --release

---

## Metrics

### Throughput

Million packets processed per second (Mpps).

### Tail Latency

Measured using:

* P50
* P95
* P99
* P99.9

### End-to-End Latency

Time from packet creation until packet consumption.

---

## Example Profiling Commands

perf stat -d ./target/release/performance_benchmark

perf stat -e instructions,cycles,cache-misses,branch-misses ./target/release/performance_benchmark

---

## Scalability Results

### 4 Producers

Throughput: 3.12 Mpps

P99 High Priority: 168 ms

P99 Low Priority: 224 ms

### 16 Producers

Throughput: 2.88 Mpps

P99 High Priority: 784 ms

P99 Low Priority: 910 ms

---

## Conclusions

The benchmark demonstrates that:

* The architecture scales efficiently up to moderate contention.
* The single consumer becomes the primary bottleneck under heavy load.
* Tail latency increases significantly once queue saturation occurs.
