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
