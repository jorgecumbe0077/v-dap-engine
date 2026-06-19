<<<<<<< HEAD
# V-DAP Engine (Validated Distributed Architecture Pipeline)

A high-performance Rust library for low-latency transaction processing, utilizing lock-free MPSC channels to minimize synchronization overhead.

## Context
This project explores the limits of concurrent data processing in Rust. Through hardware-level profiling (perf), we identified that traditional lock-based structures (e.g., DashMap) introduce contention bottlenecks under extreme write pressure.

## Methodology
- **Profiling:** Used `perf stat` to monitor IPC (Instructions Per Cycle) and cache miss rates.
- **Optimization:** Implemented a lock-free architecture leveraging `crossbeam-channel` for thread communication.
- **Data Layout:** Optimized memory alignment (`#[repr(align(64))]`) to maximize L1/L2 cache locality.

## Results (10M Transactions)
| Approach | Throughput | Hardware Bottleneck |
| :--- | :--- | :--- |
| Lock-Based | Base | Lock Contention |
| Lock-Free (V-DAP) | ~2.2x Faster | Memory Latency (Memory Bound) |

## Professional Roadmap
- [ ] Transition to a production-ready library (`lib.rs`).
- [ ] Implement realistic transaction validation logic.
- [ ] Publish technical whitepaper on performance engineering findings.

---
*Built with scientific rigor by Jorge in Maputo.*
=======
# v-dap-engine
**Low-Latency Transaction Processing Engine**

## Overview
`v-dap-engine` is a low-latency transaction processing engine built in Rust, focused on the validation of post-quantum cryptographic (PQC) signatures. The project measures the critical trade-off between infrastructure performance and the computational cost of post-quantum security (Dilithium-family).

## Architecture
The engine utilizes a *lock-free* architecture based on `crossbeam` for efficient transaction queuing and a *work-stealing scheduler* via `rayon` to parallelize intensive cryptographic validation.



## Key Benchmarks
Benchmarks demonstrate the clear separation between infrastructure efficiency and the algorithmic cost of PQC validation. The benchmark highlights that cryptographic verification, not queue synchronization, becomes the dominant computational cost.

| Scenario | Throughput (ops/s) |
| :--- | :--- |
| Lock-free Infrastructure Baseline | ~10,131,712 |
| Dilithium2 Validation Pipeline | ~17,088 |

## Scalability Analysis
Multi-threaded scalability analysis demonstrates how the system leverages multiple cores. Scaling plateaus after 4 threads, suggesting saturation of available CPU execution resources on the benchmark environment.

| Threads | Time (ms) | Speedup |
| :--- | :--- | :--- |
| 1 | 155.31 | 1.00x |
| 2 | 82.76 | 1.88x |
| 4 | 46.29 | 3.36x |
| 8 | 46.36 | 3.35x |

## Engineering Observations
* **Bottleneck Migration:** With the introduction of Dilithium2, the system bottleneck shifts from synchronization latency to computational complexity.
* **Hardware Saturation:** The system achieves peak performance at 4 threads, indicating an effective parallel scheduling strategy under the tested workload.

## Getting Started
```bash
git clone [https://github.com/your-username/v-dap-engine](https://github.com/jorgecumbe0077/v-dap-engine)
cd v-dap-engine
cargo test
cargo run --release
>>>>>>> a8e75ce17017dc5284af52da6603fdd72da0232f
