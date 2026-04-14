# Axis A/B PR Validation Report

- Generated At: 1776135251s-since-epoch
- Run ID: ab-pr-validation-1776135166
- Base Seed: 20260408-1776135166
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
| P95 CU Gate | YES | samples_ok=true (4 scenarios >=30 comparable runs), candidate(g3m) p95_total_cu=38612.00 vs baseline(pfda3) 36077.00 (limit <= +10%) |
| P50 Latency Gate | YES | success baseline/candidate = 100.00% / 100.00%, p50 slots baseline/candidate = 11.00 / 1.00, limit <= +20% |
| Quality Gate | NO | p50 slippage baseline/candidate = 41.47 / 65.38 bps; compensation_via_cu=NO |
| Reliability Gate | YES | candidate success=100.00% (>=99%), candidate critical invariant violations=0 |
| Significance Gate | YES | N=200 comparable, sample_rule=true (4 / 4 scenarios >=30 comparable runs) | total_cu p=0 ci=Some([7684.320000000004, 9214.412000000004]) | slippage p=0 ci=Some([23.811855947941105, 37.1508663455039]) |

### Scenario: scenario-01

- Description: reserve=100000000 | swap_ratio=50bps | drift_ratio=800bps | fee=100bps | sampled_tokens=5
- Scenario seed: 20260408-1776135166-scenario-01
- Token sample: ["USDT", "bSOL", "mSOL", "USDC", "JUP"]
- Comparison tokens: ["USDT", "bSOL", "mSOL"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 27076.00 / 36076.55 | 34853.00 / 38606.75 |
| Slippage bps p50/p95 | 100.00 / 100.01 | 148.72 / 150.14 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 8199.8200 | [6730.0800, 9640.5645] | 0.000000 |
| slippage_bps | 48.8570 | [48.6371, 49.0803] | 0.000000 |

### Scenario: scenario-02

- Description: reserve=1000000000 | swap_ratio=25bps | drift_ratio=500bps | fee=50bps | sampled_tokens=3
- Scenario seed: 20260408-1776135166-scenario-02
- Token sample: ["mSOL", "JTO", "USDT"]
- Comparison tokens: ["mSOL", "JTO", "USDT"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25577.00 / 36077.00 | 35587.00 / 38597.55 |
| Slippage bps p50/p95 | 50.00 / 50.00 | 74.81 / 75.37 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 8431.1800 | [6963.2925, 9872.0210] | 0.000000 |
| slippage_bps | 24.8198 | [24.7126, 24.9198] | 0.000000 |

### Scenario: scenario-03

- Description: reserve=100000000 | swap_ratio=25bps | drift_ratio=800bps | fee=30bps | sampled_tokens=5
- Scenario seed: 20260408-1776135166-scenario-03
- Token sample: ["USDT", "JUP", "wSOL", "JTO", "mSOL"]
- Comparison tokens: ["USDT", "JUP", "wSOL"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 27075.50 / 37576.00 | 34114.50 / 38614.20 |
| Slippage bps p50/p95 | 30.02 / 30.04 | 54.76 / 55.38 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 7959.0800 | [6158.6515, 9518.5010] | 0.000000 |
| slippage_bps | 24.7165 | [24.6032, 24.8324] | 0.000000 |

### Scenario: scenario-04

- Description: reserve=1000000 | swap_ratio=25bps | drift_ratio=1000bps | fee=30bps | sampled_tokens=5
- Scenario seed: 20260408-1776135166-scenario-04
- Token sample: ["mSOL", "bSOL", "JTO", "JUP", "USDT"]
- Comparison tokens: ["mSOL", "bSOL", "JTO"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25574.00 / 32399.00 | 35586.00 / 39422.80 |
| Slippage bps p50/p95 | 32.08 / 32.91 | 55.10 / 56.64 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 9328.4000 | [8007.9965, 10647.7360] | 0.000000 |
| slippage_bps | 22.9737 | [22.6104, 23.3306] | 0.000000 |

## Environment: local-validator

- Status: not_run
- Note: Run local-validator transaction-behavior benchmark separately and publish as an isolated layer.
- Note: Do not mix this layer with LiteSVM conclusions.

## Environment: devnet/mainnet-fork

- Status: not_run
- Note: Run real routing / fork validation separately and publish as an isolated layer.
- Note: Do not mix this layer with LiteSVM or local-validator conclusions.

