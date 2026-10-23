# V-DAP External Pilot — Evidence Report v0.1

## 1. Purpose

This pilot evaluates whether V-DAP temporal instrumentation can distinguish
latency occurring inside a cryptographic operation from latency introduced
outside that operation in an HTTP service.

This is an experimental engineering validation.

It is not a production performance benchmark and does not establish scheduler
causality, security leakage, or production reliability.

## 2. Experimental environment

- Host: Intel i5-3335S, 2.70 GHz
- Logical CPUs: CPU0–CPU3
- Architecture: x86_64
- Linux kernel: 7.0.0-31-generic
- Rust/Cargo: 1.93.1
- Cryptographic primitive: Dilithium2
- HTTP service: Rust standard-library TCP server
- Listener: 127.0.0.1:8080
- Crypto timing:
  - RDTSCP TSC interval (`crypto_latency_ticks`)
  - independent `Instant` interval (`crypto_latency_ns`)
- Request timing:
  - monotonic `Instant`
- Tail threshold:
  - 5,625,546 TSC ticks
- Experimental request budget:
  - 2,500,000 ns

## 3. Measurement model

For each request, the service records:

- request start/end timestamps;
- cryptographic start/end timestamps;
- cryptographic latency in TSC ticks;
- independent cryptographic latency in nanoseconds;
- total request latency;
- derived non-cryptographic latency;
- Tail classification;
- timeout classification;
- TSC AUX values and migration indicator.

The independent `Instant` measurement is used to characterize temporal
duration without converting TSC ticks into nanoseconds.

## 4. V0.1 — sequential diagnostic control

Dataset:

`evidence/http_diagnostic_v01/run_sequential_100.jsonl`

N = 100 requests.

SHA-256:

`f8fce062e525fba62ec633cfcc52b7cec44b7653734318fd9816121d3795eca5`

Observed summary:

| Metric | Result |
|---|---:|
| Request mean | 0.807033 ms |
| Request median | 0.622112 ms |
| Request P95 | 1.809426 ms |
| Request P99 | 2.232825 ms |
| Request maximum | 2.464746 ms |
| Crypto mean | 0.801611 ms |
| Crypto median | 0.616693 ms |
| Crypto P95 | 1.804608 ms |
| Crypto P99 | 2.228163 ms |
| Crypto maximum | 2.460098 ms |
| Non-crypto overhead mean | 5.422 us |
| Non-crypto overhead median | 5.231 us |
| Non-crypto overhead P95 | 5.921 us |
| Non-crypto overhead maximum | 15.367 us |
| Crypto/request Pearson correlation | 0.999995 |
| Tail Events | 2 |
| Timeouts | 0 |
| CPU migrations | 0 |

Interpretation:

In this controlled sequential service, request latency closely tracked the
measured cryptographic interval. The non-cryptographic remainder was small
relative to the crypto interval in this run.

This establishes the baseline decomposition used by the diagnostic experiment.

## 5. V0.2 — controlled non-cryptographic perturbation

The V0.2 experiment retained the cryptographic and timing instrumentation and
introduced a controlled 1 ms sleep after the cryptographic interval and before
the final request-latency measurement.

Experimental source:

`adapter/src/main.rs`

SHA-256:

`57887a5ec8763c3c1f691f0a0bca2caa584b8976d6c79a52b1251420a7ce46ec`

Control source:

`adapter/src/main_sequential_diagnostic_control.rs`

SHA-256:

`da4f41f6704eb2022fc7090fbc676d41287e22db6ddad7f27c176a19e90cdb36`

Dataset:

`evidence/http_diagnostic_v02/run_noncrypto_20.jsonl`

N = 20 requests.

SHA-256:

`9be2764dcdeac499494eaf6284af95f0ac4110d354aa1de44384dc5e63ca71e4`

Observed summary:

| Metric | Result |
|---|---:|
| Request mean | 2.056348 ms |
| Request median | 1.954082 ms |
| Request minimum | 1.542613 ms |
| Request maximum | 2.801485 ms |
| Crypto mean | 0.952347 ms |
| Crypto median | 0.820918 ms |
| Crypto minimum | 0.470701 ms |
| Crypto maximum | 1.728538 ms |
| Non-crypto component mean | 1.104001 ms |
| Non-crypto component median | 1.071600 ms |
| Non-crypto component minimum | 1.064705 ms |
| Non-crypto component maximum | 1.619993 ms |
| Tail Events | 0 |
| Timeouts | 2 |
| CPU migrations | 0 |

The two timeout-classified requests were requests 10 and 16.

Request 10:

- crypto: 1.728538 ms
- request: 2.801485 ms
- non-crypto: 1.072947 ms

Request 16:

- crypto: 1.659493 ms
- request: 2.790576 ms
- non-crypto: 1.131083 ms

The largest observed non-cryptographic remainder occurred at request 17:

- crypto: 0.714853 ms
- request: 2.334846 ms
- non-crypto: 1.619993 ms

No Tail Event was recorded in V0.2.

## 6. Diagnostic comparison

The two experiments provide complementary controlled cases.

### V0.1

The request was dominated by the cryptographic interval:

`request latency ≈ crypto latency + small remainder`

The mean non-cryptographic remainder was approximately 5.4 microseconds.

### V0.2

The request contained an intentionally introduced non-cryptographic delay:

`request latency ≈ crypto latency + approximately 1 ms non-crypto component`

The mean non-cryptographic component was 1.104001 ms.

The experiment therefore produced a case where request latency increased without
the cryptographic Tail detector classifying the cryptographic interval as a
Tail Event.

## 7. Engineering observation

Within this controlled HTTP service, V-DAP instrumentation made it possible
to distinguish two different temporal situations:

1. latency dominated by the cryptographic operation;
2. latency introduced outside the cryptographic operation.

The relevant engineering capability is therefore not simply measuring an
aggregate request latency. It is decomposing the request into temporal
components and observing which component accounts for the measured delay.

## 8. What this evidence demonstrates

The evidence supports the following bounded statement:

> In a controlled HTTP service, V-DAP temporal instrumentation can distinguish
> degradation occurring within the measured cryptographic interval from
> degradation introduced outside that interval.

The experiment also demonstrates that the same request-level latency increase
can occur without a cryptographic Tail Event when the perturbation is placed
outside the crypto interval.

## 9. What this evidence does not demonstrate

The experiments do not establish:

- scheduler causality;
- the physical cause of observed latency variation;
- that TSC ticks are equivalent to CPU cycles;
- absolute execution determinism;
- production performance;
- production timeout rates;
- production retry behaviour;
- a reduction in latency caused by V-DAP;
- a systematic performance advantage over existing observability tools;
- side-channel leakage or cryptographic vulnerability;
- a specific scheduler mechanism responsible for any observed event.

The 1 ms perturbation in V0.2 was deliberately introduced by the experiment.
Its purpose was diagnostic localization, not simulation of a production incident.

## 10. Evidence status

| Claim | Status |
|---|---|
| Per-request crypto timing is observable | OBSERVED |
| Request latency can be decomposed into crypto and non-crypto components | OBSERVED |
| Controlled non-crypto perturbation appears in the non-crypto component | OBSERVED |
| Crypto Tail and request latency can be compared | OBSERVED |
| Temporal diagnostic localization in the controlled service | DEMONSTRATED |
| Scheduler mechanism identified | NOT DEMONSTRATED |
| Scheduler causality established | NOT DEMONSTRATED |
| Production impact established | NOT DEMONSTRATED |
| Production performance established | NOT DEMONSTRATED |

## 11. Reproducibility artifacts

### Control source

`adapter/src/main_sequential_diagnostic_control.rs`

SHA-256:

`da4f41f6704eb2022fc7090fbc676d41287e22db6ddad7f27c176a19e90cdb36`

### Experimental source

`adapter/src/main.rs`

SHA-256:

`57887a5ec8763c3c1f691f0a0bca2caa584b8976d6c79a52b1251420a7ce46ec`

### V0.1 dataset

`evidence/http_diagnostic_v01/run_sequential_100.jsonl`

SHA-256:

`f8fce062e525fba62ec633cfcc52b7cec44b7653734318fd9816121d3795eca5`

### V0.2 dataset

`evidence/http_diagnostic_v02/run_noncrypto_20.jsonl`

SHA-256:

`9be2764dcdeac499494eaf6284af95f0ac4110d354aa1de44384dc5e63ca71e4`

### Schema

`schema/pilot_v01.csv`

SHA-256:

`9df156c8a40e05b76b665ebaf200d7896d464081be95abc077d8bcdd55c7dc95`

## 12. Pilot conclusion

The controlled pilot does not attempt to prove that V-DAP improves system
performance.

Its result is narrower and operationally testable:

> V-DAP provides temporal observability that can separate cryptographic
> execution time from non-cryptographic request delay and can therefore change
> the localization of an engineering investigation.

The next validation step is not another synthetic perturbation. It is to apply
the same measurement model to a real service workload where an existing
latency, timeout, queueing, or capacity problem is already observable and
where the additional temporal decomposition can be evaluated against the
engineering decision it enables.
