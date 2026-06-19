---

### 2. `DESIGN.md`

```markdown
# DESIGN.md: v-dap-engine

## 1. Objective
`v-dap-engine` provides an experimental high-performance mempool architecture capable of sustaining post-quantum cryptographic (PQC) signature validation.

## 2. Architectural Choices
* **Concurrency:** `crossbeam-deque` for lock-free queuing. In benchmark conditions, the queue layer achieved approximately 10M operations/s.
* **Parallelism:** `rayon` work-stealing scheduler for dynamic load balancing across CPU cores.
* **PQC Integration:** `pqcrypto-dilithium`. Dilithium-family algorithms were selected because they form the basis of NIST post-quantum digital signature standardization.

## 3. Data Flow
Transaction 
    | 
    v 
Ingress Queue 
    | 
    v 
Validation Workers 
    | 
    v 
PQC Verification 
    | 
    v 
Mempool Storage

## 4. Current Limitations
- Single-node benchmark.
- No network transport included.
- No persistent storage layer.
- Benchmark focused on CPU and concurrency behavior.

## 5. Performance Goals
* Keep infrastructure synchronization overhead below 10% of the total latency budget.
* Achieve near-linear multi-threaded scalability up to the hardware's physical limit, as validated by the speedup curve.
