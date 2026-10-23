# V-DAP Engine

**Experimental cryptographic execution and timing-observation engine.**

V-DAP explores temporal variability during cryptographic operations and investigates whether that variability can be meaningfully related to operating-system scheduling and application-level behaviour.

## Project status

Experimental research and validation. Not production-ready.

The included reports document observed timing distributions, scheduler-related observations, and an exploratory payment-gateway lab. They also describe the limitations of the evidence.

The current evidence does **not** establish:
- that V-DAP causes or prevents timeouts;
- guaranteed latency reduction or improved service-level objectives;
- deterministic execution under all system conditions;
- physical CPU isolation or scheduler improvement;
- commercial superiority over existing observability tools.

The payment-gateway timeout results are laboratory observations, not production measurements. Association does not establish causality.

## Repository contents

- `src/` — selected Rust source files.
- `Cargo.toml` and `Cargo.lock` — package metadata and locked dependencies.
- `evidence/` — experimental reports and evidence manifests.

## Reproduction from a clean checkout

These steps let a reviewer build and run the published source without
using a previously compiled V-DAP binary or local experiment files.

### Prerequisites

- Git
- Rust toolchain (`rustc` and `cargo`)
- A native C build toolchain supported by the Rust dependencies

Record the operating system and toolchain versions when reporting results.

### 1. Clone and identify the source

```bash
git clone --branch main https://github.com/jorgecumbe0077/v-dap-engine.git
cd v-dap-engine

git rev-parse HEAD
git log -1 --oneline
rustc --version
cargo --version
uname -a
```

Include the commit hash and environment details in any reproduction report.

### 2. Run tests and build

```bash
cargo test --locked
cargo build --release --locked
```

A successful result means the tests passed and the release build completed
in that environment. It does not establish equivalence to a previously
measured binary.

### 3. Check the CLI

```bash
./target/release/v-dap-engine --help
```

The documented CLI currently exposes `--operations` and `--help`.

### 4. Run one operation outside the source tree

```bash
set -eu

BIN="$PWD/target/release/v-dap-engine"
OUT_DIR="$(mktemp -d)"

(
  cd "$OUT_DIR"
  "$BIN" --operations 1
  test -f v03_latencies_raw.txt
  test "$(wc -l < v03_latencies_raw.txt)" -eq 1
  awk 'NF != 1 || $1 !~ /^[0-9]+$/ { exit 1 } END {
    if (NR != 1) exit 1
  }' v03_latencies_raw.txt
  echo "Raw output check passed"
)
```

The one-operation run should create a raw latency file containing one
numeric record. The measured value, tail classification, and throughput
can vary between runs and machines.

### Interpretation and limitations

- Passing tests validates the tested code paths, not every runtime condition.
- A successful build and one-operation run demonstrate basic build and
  execution, not statistical reliability or independent replication.
- TSC values are reported as ticks; they must not be assumed to be
  nanoseconds without a justified calibration.
- A tail classification does not by itself identify its cause.
- These steps do not demonstrate timeout prevention, causality, production
  suitability, or diagnostic superiority over conventional observability.
- Report the exact commit, environment, commands, outputs, failures, and
  deviations. Do not omit failed runs.
## Version note

The package metadata currently identifies the Rust crate as `0.2.0`, while the principal evidence report refers to the experimental engine as V-DAP v0.3. These labels are preserved transparently; they should not be interpreted as proof of a formally reproducible v0.3 release.

## Evidence and review guide

For an initial technical review, read the documents in this order:

1. **Experimental results and limitations:**
   `evidence/V-DAP_V0.3_FINAL_EVIDENCE_REPORT.txt`

2. **Pilot evidence manifest:**
   `evidence/V-DAP_EVIDENCE_MANIFEST_PILOT_v1.txt`

3. **External pilot report:**
   `evidence/V-DAP_EXTERNAL_PILOT_EVIDENCE_REPORT_v0.1.md`

4. **External pilot evidence manifest:**
   `evidence/V-DAP_EXTERNAL_PILOT_EVIDENCE_MANIFEST_v0.1.txt`

The reports describe experimental work; the manifests help identify the evidence associated with that work. Their presence in this repository does not, by itself, establish independent replication or production validation.

Reviewers should distinguish recorded measurements from interpretation and hypotheses. The current project status and limitations described above remain applicable.

## Scope

V-DAP is being evaluated as a possible diagnostic instrument. Its practical value depends on whether it reveals actionable information beyond conventional metrics and tracing in a realistic, independently validated workload.

No production, security, or economic outcome is guaranteed.

## License

No license has been specified in this repository snapshot. All rights remain with their respective owners unless otherwise stated.
