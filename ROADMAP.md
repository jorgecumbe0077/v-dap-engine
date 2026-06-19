# Roadmap

### v0.1 (Current)
- [x] Lock-free ingestion pipeline using `crossbeam`.
- [x] PQC validation benchmark integrated with `pqcrypto-dilithium`.
- [x] Multithread scalability analysis and bottleneck identification.

### v0.2 (Planned)
- [ ] Network transport layer implementation.
- [ ] Integration of a real-time mempool API.
- [ ] Observability: Metrics endpoint (Prometheus/OpenTelemetry) to track throughput and P99 latency.

### v0.3 (Future)
- [ ] Distributed validation worker nodes.
- [ ] Persistent storage layer for transaction indexing.
- [ ] Full integration with production-grade blockchain runtimes.
