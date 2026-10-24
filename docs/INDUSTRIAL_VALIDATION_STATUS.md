# V-DAP Industrial Validation Status

**Status:** Experimental validation — industrial value not yet established
**Scope:** Cryptographic timing and request-level diagnostic localization

## 1. Purpose

This document records the current evidence boundary and defines a bounded
validation plan for evaluating whether V-DAP provides actionable diagnostic
information beyond conventional application metrics and tracing.

The immediate candidate use case is latency diagnosis in an HTTP service,
with payment-gateway workloads as a possible later validation environment.

This is a research and engineering evaluation, not a production-readiness
claim.

## 2. Current evidence

### 2.1 Published engine experiments

The repository includes experimental timing measurements, a tail detector,
and evidence manifests. These artifacts document the methods and results
available in this repository snapshot.

A tail classification means that a measured value crossed the configured
experimental threshold. It does not identify the physical or software cause
of the observed latency.

### 2.2 Controlled HTTP diagnostic pilot

The existing report
[`V-DAP_EXTERNAL_PILOT_EVIDENCE_REPORT_v0.1.md`](../evidence/V-DAP_EXTERNAL_PILOT_EVIDENCE_REPORT_v0.1.md)
describes two controlled experiments.

**V0.1 — sequential control**

- 100 requests.
- Reported Pearson correlation between crypto latency and request latency:
  0.999995.
- Reported mean non-cryptographic remainder: approximately 5.422 microseconds.
- No timeout classifications in this run.

**V0.2 — controlled non-cryptographic perturbation**

- 20 requests.
- A deliberate 1 ms sleep was introduced after the cryptographic interval.
- Reported mean non-cryptographic component: 1.104001 ms.
- Two requests were classified as timeouts under the experiment's 2.5 ms
  request budget.
- No cryptographic tail events were recorded in this run.

These results support temporal decomposition in the particular controlled
service and conditions described in the existing report. The V0.2 delay was
intentionally introduced by the experiment; it was not a production incident.

### 2.3 Payment-gateway laboratory

The existing pilot manifest reports 400 requests across four runs:

- 17 tail events;
- 17 laboratory timeout classifications;
- 6 requests with both labels;
- reported P(timeout | tail) of 35.29%;
- reported P(timeout | normal) of 2.87%.

The manifest also states that the tail/timeout association was heterogeneous
across runs and was not consistently reproduced. The laboratory timeout rule
is not a production gateway timeout, and the reported association does not
establish causality.

## 3. Evidence boundary

### Supported by the current published reports

- Cryptographic and request-level timing can be recorded in the documented
  experimental service.
- A deliberately introduced delay outside the cryptographic interval can
  appear in the non-cryptographic remainder.
- Tail classifications and request-level outcomes can be compared.
- The laboratory payment-gateway results show an association that varies
  across experimental runs.

### Not established

- That V-DAP identifies scheduler causality or a specific physical cause.
- That V-DAP outperforms conventional metrics, logs, or distributed tracing.
- That V-DAP prevents timeouts or reduces production latency.
- That laboratory timeout labels represent real payment failures.
- That the results generalize to production traffic or other machines.
- That the instrument's operational benefit exceeds its measurement overhead.

## 4. Industrial hypothesis

**Hypothesis:** For at least one defined service-diagnosis task, adding V-DAP
temporal measurements to a conventional observability baseline helps an
engineer localize a latency problem more accurately or with less investigation
effort.

This hypothesis is not yet validated.

The test must permit a negative result: if the conventional baseline provides
the same diagnostic information, or if V-DAP's additional measurements are
unreliable or too costly, the claimed incremental value is not demonstrated.

## 5. Bounded validation protocol

### Step 1 — choose one real operational question

Select a service workload with a measurable latency problem, such as
requests whose total latency exceeds a pre-defined budget.

Define the request, workload, latency budget, and operational outcome before
collecting evaluation data. Do not label a laboratory threshold as a
production service-level objective.

### Step 2 — define the baseline

Record the information available from conventional observability, such as:

- total request latency and latency percentiles;
- request outcome and error classification;
- available application spans and timing boundaries;
- queueing or resource metrics, where actually collected.

Document unavailable measurements rather than assuming they exist.

### Step 3 — define the V-DAP addition

Record the additional timing boundaries and fields used by the experiment,
including crypto duration, request duration, tail classification, and any
scheduler counters that have been validated.

Document units, clock sources, threshold definitions, instrumentation
overhead, missing data, and measurement-window alignment.

### Step 4 — compare diagnostic outcomes

Use the same workload and pre-defined evaluation cases for both approaches.
Where practical, have reviewers assess the baseline and augmented evidence
without seeing the other condition first.

Measure:

- whether the correct injected or independently verified delay location
  is identified;
- false localization and inconclusive results;
- time or steps required to reach the diagnosis;
- instrumentation overhead and missing or invalid measurements.

Injected delays can validate measurement localization, but they do not by
themselves validate performance on real incidents. A production claim
requires independently grounded incident or workload evidence.

### Step 5 — apply a pre-defined decision rule

**Evidence of incremental value:** the augmented approach improves a
pre-specified diagnostic outcome relative to the baseline, with repeatable
results and acceptable measurement overhead.

**Hypothesis not supported:** the baseline performs equally well, the
improvement does not repeat, false localization is unacceptable, or the
additional measurements cannot be interpreted reliably.

**Inconclusive:** sample size, data quality, or incident ground truth is
insufficient to distinguish the two approaches.

Set numerical acceptance thresholds before the comparative evaluation.
Do not choose them retrospectively to fit the observed results.

## 6. Immediate engineering gate

Before using the scheduler diagnostic adapter as evidence:

1. Preserve the original V-DAP binary and verify its recorded hash.
2. Keep diagnostic adapter changes separate from the published engine source.
3. Validate the scheduler-counter measurement window against the cryptographic
   timing window.
4. Verify JSON field definitions, units, and timeout semantics.
5. Preserve raw outputs, commands, environment versions, and hashes.
6. Re-run the documented checks and report failures as well as successful runs.

A successful adapter request validates instrumentation behaviour only; it
does not establish diagnostic value or scheduler causality.

## 7. Publication rule

Update this status document only when new evidence has been collected and
reviewed. Each result should identify its source artifacts, environment,
sample size, method, and limitations.

Do not replace previous evidence silently. Keep the historical reports and
their manifests available so reviewers can distinguish changes in code,
method, and conclusions.

## 8. Current decision

**Decision: continue with one bounded diagnostic-value evaluation; do not
claim production benefit yet.**

The next useful milestone is a reproducible comparison against conventional
observability on one clearly defined service-diagnosis task. Additional
low-level benchmarks are justified only if they resolve a specific
measurement uncertainty that blocks that comparison.
