# Axis A/B PR Validation Report

- Generated At: 1775831636s-since-epoch
- Run ID: ab-pr-validation-1775831551
- Base Seed: 20260408-1775831551
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
| P95 CU Gate | YES | samples_ok=true (4 scenarios >=30 comparable runs), candidate(g3m) p95_total_cu=38594.35 vs baseline(pfda3) 36149.00 (limit <= +10%) |
| P50 Latency Gate | YES | success baseline/candidate = 100.00% / 100.00%, p50 slots baseline/candidate = 11.00 / 1.00, limit <= +20% |
| Quality Gate | NO | p50 slippage baseline/candidate = 75.10 / 123.30 bps; compensation_via_cu=NO |
| Reliability Gate | YES | candidate success=100.00% (>=99%), candidate critical invariant violations=0 |
| Significance Gate | YES | N=200 comparable, sample_rule=true (4 / 4 scenarios >=30 comparable runs) | total_cu p=0 ci=Some([7729.024375000001, 9213.03975]) | slippage p=0 ci=Some([43.70364081700405, 52.51361898163424]) |

### Scenario: scenario-01

- Description: reserve=100000000 | swap_ratio=75bps | drift_ratio=1200bps | fee=30bps | sampled_tokens=3
- Scenario seed: 20260408-1775831551-scenario-01
- Token sample: ["wSOL", "USDT", "bSOL"]
- Comparison tokens: ["wSOL", "USDT", "bSOL"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 27076.50 / 34576.55 | 34119.50 / 37933.65 |
| Slippage bps p50/p95 | 30.01 / 30.01 | 104.43 / 105.96 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 7724.0200 | [6223.8205, 9225.3300] | 0.000000 |
| slippage_bps | 74.1414 | [73.7806, 74.5230] | 0.000000 |

### Scenario: scenario-02

- Description: reserve=1000000 | swap_ratio=25bps | drift_ratio=1200bps | fee=100bps | sampled_tokens=3
- Scenario seed: 20260408-1775831551-scenario-02
- Token sample: ["mSOL", "JTO", "JUP"]
- Comparison tokens: ["mSOL", "JTO", "JUP"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25574.00 / 33899.00 | 34102.50 / 39421.20 |
| Slippage bps p50/p95 | 102.02 / 103.64 | 124.98 / 127.55 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 9390.0400 | [8071.8270, 10649.9440] | 0.000000 |
| slippage_bps | 22.7997 | [22.2312, 23.3380] | 0.000000 |

### Scenario: scenario-03

- Description: reserve=1000000 | swap_ratio=25bps | drift_ratio=500bps | fee=100bps | sampled_tokens=5
- Scenario seed: 20260408-1775831551-scenario-03
- Token sample: ["JUP", "bSOL", "JTO", "wSOL", "USDT"]
- Comparison tokens: ["JUP", "bSOL", "JTO"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 27074.00 / 37574.00 | 35590.50 / 38586.20 |
| Slippage bps p50/p95 | 101.86 / 103.78 | 124.61 / 127.73 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 7828.2400 | [5936.0610, 9507.2860] | 0.000000 |
| slippage_bps | 22.4440 | [21.8473, 23.0354] | 0.000000 |

### Scenario: scenario-04

- Description: reserve=10000000 | swap_ratio=75bps | drift_ratio=500bps | fee=50bps | sampled_tokens=5
- Scenario seed: 20260408-1775831551-scenario-04
- Token sample: ["bSOL", "JTO", "mSOL", "USDC", "wSOL"]
- Comparison tokens: ["bSOL", "JTO", "mSOL"]
- Comparable for gate: true
- Target repeats: 50
- Attempts: 50
- Comparable runs: 50

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|---|---:|---:|
| Total CU p50/p95 | 25575.00 / 35700.00 | 34845.50 / 37106.95 |
| Slippage bps p50/p95 | 50.06 / 50.12 | 123.60 / 125.55 |
| Slots-to-finality p50/p95 | 11.00 / 11.00 | 1.00 / 1.00 |
| Success rate | 100.00% | 100.00% |

Significance checks:

| Metric | Δ mean (candidate - baseline) | 95% bootstrap CI | Mann-Whitney p |
|---|---:|---|---:|
| total_cu | 8973.8800 | [7563.8510, 10263.7030] | 0.000000 |
| slippage_bps | 73.5075 | [73.1320, 73.8981] | 0.000000 |

## Environment: local-validator

- Status: not_run
- Note: Run local-validator transaction-behavior benchmark separately and publish as an isolated layer.
- Note: Do not mix this layer with LiteSVM conclusions.

## Environment: devnet/mainnet-fork

- Status: not_run
- Note: Run real routing / fork validation separately and publish as an isolated layer.
- Note: Do not mix this layer with LiteSVM or local-validator conclusions.

