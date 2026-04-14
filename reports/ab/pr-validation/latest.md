# Axis A/B PR Validation Report

- Generated At: 1776163484s-since-epoch
- Run ID: ab-pr-validation-1776163398
- Base Seed: 20260408-1776163398
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
| P95 CU Gate | YES | samples_ok=true (4 scenarios >=30 comparable runs), candidate(g3m) p95_total_cu=40096.25 vs baseline(pfda3) 37576.10 (limit <= +10%) |
| P50 Latency Gate | YES | success baseline/candidate = 100.00% / 100.00%, p50 slots baseline/candidate = 11.00 / 1.00, limit <= +20% |
| Quality Gate | NO | p50 slippage baseline/candidate = 75.01 / 124.10 bps; compensation_via_cu=NO |
| Reliability Gate | YES | candidate success=100.00% (>=99%), candidate critical invariant violations=0 |
| Significance Gate | YES | N=200 comparable, sample_rule=true (4 / 4 scenarios >=30 comparable runs) | total_cu p=0 ci=Some([7678.152999999998, 9245.468125000001]) | slippage p=0 ci=Some([47.34253846410195, 61.34303744983502]) |

### Scenario: scenario-01

- Description: reserve=1000000 | swap_ratio=50bps | drift_ratio=500bps | fee=30bps | sampled_tokens=4
- Scenario seed: 20260408-1776163398-scenario-01
- Token sample: ["mSOL", "wSOL", "USDT", "bSOL"]
- Comparison tokens: ["mSOL", "wSOL", "USDT"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 27076.00 / 36901.00 | 34108.00 / 40103.75 |
| Slippage bps p50/p95 | 30.76 / 31.86 | 78.64 / 81.64 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 8283.3000 | [6633.6515, 9816.6050] | 0.000000 |
| slippage_bps | 48.2247 | [47.8228, 48.6442] | 0.000000 |

### Scenario: scenario-02

- Description: reserve=100000000 | swap_ratio=50bps | drift_ratio=500bps | fee=100bps | sampled_tokens=3
- Scenario seed: 20260408-1776163398-scenario-02
- Token sample: ["JUP", "USDT", "JTO"]
- Comparison tokens: ["JUP", "USDT", "JTO"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25579.00 / 35403.45 | 34116.50 / 38614.65 |
| Slippage bps p50/p95 | 100.01 / 100.01 | 148.57 / 149.89 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 9158.3400 | [7808.9565, 10596.5185] | 0.000000 |
| slippage_bps | 48.6188 | [48.4260, 48.8055] | 0.000000 |

### Scenario: scenario-03

- Description: reserve=100000000 | swap_ratio=50bps | drift_ratio=1200bps | fee=50bps | sampled_tokens=3
- Scenario seed: 20260408-1776163398-scenario-03
- Token sample: ["mSOL", "JTO", "USDC"]
- Comparison tokens: ["mSOL", "JTO", "USDC"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 27078.50 / 38403.00 | 35604.00 / 38615.05 |
| Slippage bps p50/p95 | 50.01 / 50.02 | 99.10 / 100.51 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 7299.9000 | [5260.1515, 9191.9735] | 0.000000 |
| slippage_bps | 49.1100 | [48.8564, 49.3629] | 0.000000 |

### Scenario: scenario-04

- Description: reserve=1000000000 | swap_ratio=75bps | drift_ratio=1000bps | fee=100bps | sampled_tokens=4
- Scenario seed: 20260408-1776163398-scenario-04
- Token sample: ["bSOL", "USDT", "JTO", "USDC"]
- Comparison tokens: ["bSOL", "USDT", "JTO"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25580.00 / 34578.00 | 34104.50 / 39420.70 |
| Slippage bps p50/p95 | 100.00 / 100.00 | 173.14 / 174.79 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 9209.9600 | [7770.3545, 10652.0325] | 0.000000 |
| slippage_bps | 73.0290 | [72.6485, 73.4003] | 0.000000 |

## Environment: local-validator

- Status: not_run
- Note: Run local-validator transaction-behavior benchmark separately and publish as an isolated layer.
- Note: Do not mix this layer with LiteSVM conclusions.

## Environment: devnet/mainnet-fork

- Status: not_run
- Note: Run real routing / fork validation separately and publish as an isolated layer.
- Note: Do not mix this layer with LiteSVM or local-validator conclusions.

