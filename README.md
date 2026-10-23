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

## Build check

The selected source snapshot passed:

```bash
cargo check --locked
```

This checks the source without establishing that a release binary is reproducible or identical to any previously measured binary. Two compiler warnings were reported.

To build locally:

```bash
cargo build --release --locked
```

The build command has not been represented here as an independently validated reproduction of the original experimental binary.

## Version note

The package metadata currently identifies the Rust crate as `0.2.0`, while the principal evidence report refers to the experimental engine as V-DAP v0.3. These labels are preserved transparently; they should not be interpreted as proof of a formally reproducible v0.3 release.

## Evidence

Start with:

- `evidence/V-DAP_V0.3_FINAL_EVIDENCE_REPORT.txt`
- `evidence/V-DAP_EVIDENCE_MANIFEST_PILOT_v1.txt`
- `evidence/V-DAP_EXTERNAL_PILOT_EVIDENCE_REPORT_v0.1.md`
- `evidence/V-DAP_EXTERNAL_PILOT_EVIDENCE_MANIFEST_v0.1.txt`

## Scope

V-DAP is being evaluated as a possible diagnostic instrument. Its practical value depends on whether it reveals actionable information beyond conventional metrics and tracing in a realistic, independently validated workload.

No production, security, or economic outcome is guaranteed.

## License

No license has been specified in this repository snapshot. All rights remain with their respective owners unless otherwise stated.
