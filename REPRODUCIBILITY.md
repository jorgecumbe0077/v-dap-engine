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

## Reproducing on Desktop (Windows, Linux, macOS)
To reproduce the benchmark on a desktop environment, you need the Rust toolchain.

1. Prerequisites
Install the Rust toolchain by following the official guide at rustup.rs.
On Linux/macOS: Run```Bash curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh ```

On Windows: Download and run ``` rustup-init.exe``` from the website

2. Obtain the Source Code

```Bash
git clone https://github.com/jorgecumbe0077/v-dap-engine.git
cd v-dap-engine
```

3. Compile the Engine

```Bash
cargo build --release
```
4. Execute the Benchmark
   


# On Linux/macOS:
```Bash
./target/release/v-dap-engine
```

# On Windows:
```Bash
.\target\release\v-dap-engine.exe
```

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
