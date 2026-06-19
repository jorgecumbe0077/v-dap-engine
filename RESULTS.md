# RESULTS.md: Benchmarking Protocol

This document defines the standard procedure for reproducing the performance benchmarks of the `v-dap-engine`.

## 1. Environment Requirements
To achieve comparable results, ensure your test environment matches these specifications:
- **OS:** Linux (Ubuntu 22.04+ recommended)
- **Rust Toolchain:** v1.70 or newer (`rustc --version`)
- **CPU:** Physical cores (ensure frequency scaling governors are set to `performance` mode)

## 2. Reproduction Protocol
Follow these steps to generate consistent metrics:

1. **Clean:** Ensure a clean state to avoid cache interference:
   ```bash
   cargo clean
