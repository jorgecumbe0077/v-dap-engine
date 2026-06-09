# BENCHMARKS.md: Scalability Analysis

## Results Table
| Producers | Throughput (Mpps) | P99 High Priority (ms) |
| :--- | :--- | :--- |
| 4 | 3.12 | 168 |
| 16 | 2.88 | 784 |

## Analysis
The system demonstrates high efficiency up to 4 producers. Beyond this, the single-consumer bottleneck causes throughput degradation and significant tail-latency (P99) expansion, indicating queue saturation.
