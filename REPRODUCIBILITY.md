# Reproducibility

The objective is that any engineer can reproduce
the benchmark independently.

# REPRODUCIBILITY.md

# v-dap-engine Reproducibility Report

## Objective

This document describes how to reproduce the performance experiments performed on different hardware architectures.

The goal is to verify that the observed behavior is not dependent on a single development environment.

---

# Environment A

## Desktop Linux

Architecture:

```
x86_64
```

Build:

```bash
cargo build --release
```

Run:

```bash
./target/release/v-dap-engine
```

---

# Environment B

## Android ARM Device

Hardware:

```
Mobile ARM processor
2GB RAM
```

Environment:

```
Termux Linux environment
```

Installation:

```bash
pkg update

pkg install rust git clang
```

Clone:

```bash
git clone https://github.com/jorgecumbe0077/v-dap-engine.git

cd v-dap-engine
```

Build:

```bash
cargo build --release
```

Run:

```bash
./target/release/v-dap-engine
```

---

# Observed Scaling

Android ARM:

| Threads |    Time |
| ------- | ------: |
| 1       | 1943 ms |
| 2       |  946 ms |
| 4       |  491 ms |
| 8       |  492 ms |

---

# Analysis

The workload demonstrated near-linear scaling until the physical execution capacity of the device was reached.

The performance plateau observed at higher thread counts suggests hardware saturation rather than software synchronization overhead.

---

# Conclusion

The benchmark was reproduced on different architectures, showing consistent scaling behavior.

The experiment demonstrates that the v-dap-engine architecture is portable across environments.
