# Axis A/B PR Validation Report

- Generated At: 1776659675s-since-epoch
- Run ID: ab-pr-validation-1776659573
- Base Seed: 20260408-1776659573
- Repeats/Scenario: 50

## Fairness Rules

- Token universe candidates: ["wSOL", "USDC", "USDT", "JUP", "JTO", "mSOL", "bSOL"]
- Initial liquidity: ETF A/B use equal initial reserve value per active token under each scenario.
- Fee rule: ETF A/B use the same fee_bps sampled per scenario.
- Swap rule: ETF A/B use the same swap ratio and swap amount per run.
- Note: Cold-start CU is separated from steady-state CU.
- Note: Gate evaluation is environment-local and never mixed across layers.
- Note: Sampler auto-runs additional attempts (up to AB_MAX_ATTEMPT_MULT=4x) to hit target comparable N per scenario.

## Environment: LiteSVM

- Status: completed
- Note: Fast iteration environment; conclusions stay within LiteSVM layer.
- Note: A/B gate uses grouped comparison: PFDA-3 executes 3-token batch path while G3M executes 2-token path on the same active swap pair.

### Multi-Metric Gate

- Baseline: PFDA-3
- Candidate: G3M
- Gate Result: **FAIL**

| Gate | Pass | Detail |
|---|---|---|
| P95 CU Gate | YES | samples_ok=true (4 scenarios >=30 comparable runs), candidate(g3m) p95_total_cu=38596.40 vs baseline(pfda3) 37575.00 (limit <= +10%) |
| P50 Latency Gate | YES | success baseline/candidate = 88.50% / 100.00%, p50 slots baseline/candidate = 11.00 / 1.00, limit <= +20% |
| Quality Gate | NO | p50 slippage baseline/candidate = 50.07 / 86.43 bps; compensation_via_cu=NO |
| Reliability Gate | YES | candidate success=100.00% (>=99%), candidate critical invariant violations=0 |
| Significance Gate | YES | N=200 comparable, sample_rule=true (4 / 4 scenarios >=30 comparable runs) | total_cu p=0 ci=Some([7601.246375000001, 9190.532374999997]) | slippage p=0 ci=Some([44.63051644398353, 52.871239375453506]) |

### Scenario: scenario-01

- Description: reserve=1000000 | swap_ratio=50bps | drift_ratio=1200bps | fee=50bps | sampled_tokens=3
- Scenario seed: 20260408-1776659573-scenario-01
- Token sample: ["USDC", "JTO", "JUP"]
- Comparison tokens: ["USDC", "JTO", "JUP"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 27074.00 / 35399.00 | 34107.50 / 37106.70 |
| Slippage bps p50/p95 | 51.22 / 51.92 | 99.36 / 100.93 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 8135.3000 | [6695.1765, 9545.7215] | 0.000000 |
| slippage_bps | 48.2477 | [47.9189, 48.5604] | 0.000000 |

### Scenario: scenario-02

- Description: reserve=100000000 | swap_ratio=25bps | drift_ratio=1200bps | fee=50bps | sampled_tokens=5
- Scenario seed: 20260408-1776659573-scenario-02
- Token sample: ["wSOL", "bSOL", "JTO", "USDT", "JUP"]
- Comparison tokens: ["wSOL", "bSOL", "JTO"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25576.00 / 37575.55 | 35595.50 / 38606.20 |
| Slippage bps p50/p95 | 50.02 / 50.04 | 74.84 / 75.41 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 9186.8600 | [7684.7515, 10507.3020] | 0.000000 |
| slippage_bps | 24.7925 | [24.6820, 24.8984] | 0.000000 |

### Scenario: scenario-03

- Description: reserve=10000000 | swap_ratio=25bps | drift_ratio=800bps | fee=50bps | sampled_tokens=5
- Scenario seed: 20260408-1776659573-scenario-03
- Token sample: ["mSOL", "bSOL", "JTO", "USDC", "wSOL"]
- Comparison tokens: ["mSOL", "bSOL", "JTO"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25575.00 / 36900.00 | 34103.00 / 38594.55 |
| Slippage bps p50/p95 | 50.23 / 50.37 | 74.75 / 75.48 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 8363.4400 | [6413.8220, 10133.2785] | 0.000000 |
| slippage_bps | 24.5584 | [24.4176, 24.6891] | 0.000000 |

### Scenario: scenario-04

- Description: reserve=10000000 | swap_ratio=100bps | drift_ratio=1200bps | fee=50bps | sampled_tokens=3
- Scenario seed: 20260408-1776659573-scenario-04
- Token sample: ["JUP", "wSOL", "bSOL"]
- Comparison tokens: ["JUP", "wSOL", "bSOL"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 76
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25575.00 / 40050.00 | 34108.50 / 38601.50 |
| Slippage bps p50/p95 | 50.04 / 50.08 | 146.76 / 148.05 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 65.79% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 7867.1800 | [6275.5940, 9489.2740] | 0.000000 |
| slippage_bps | 96.7447 | [96.5008, 96.9909] | 0.000000 |

## Environment: local-validator

- Status: not_run
- Note: Run local-validator transaction-behavior benchmark separately and publish as an isolated layer.
- Note: Do not mix this layer with LiteSVM conclusions.

## Environment: devnet/mainnet-fork

- Status: not_run
- Note: Run real routing / fork validation separately and publish as an isolated layer.
- Note: Do not mix this layer with LiteSVM or local-validator conclusions.

