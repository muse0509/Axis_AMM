# Axis A/B Test Report

- Generated: 1776659570s-since-epoch
- Environment: LiteSVM (local, multi-scenario)

## Scenario 1: Small pool, tiny swap

Reserve: 1000000, Swap: 10000, Drift trigger: 200000

- Swap amount: 10000
- Initial reserves: [1000000, 1000000]

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|--------|---------------:|------------:|
| Init CU | 0 | 10992 |
| Swap/Request CU | 7739 | 10977 |
| Clear/Rebalance CU | 10302 | 8639 |
| Claim CU | 3033 | N/A |
| **Total CU** | **21074** | **31774** |
| Tokens received | 9970 | 9803 |
| Execution slots | 11 | 1 |

## Scenario 2: Medium pool, 1% swap

Reserve: 100000000, Swap: 1000000, Drift trigger: 20000000

- Swap amount: 1000000
- Initial reserves: [100000000, 100000000]

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|--------|---------------:|------------:|
| Init CU | 0 | 14000 |
| Swap/Request CU | 13739 | 10969 |
| Clear/Rebalance CU | 20804 | 8630 |
| Claim CU | 3033 | N/A |
| **Total CU** | **37576** | **34765** |
| Tokens received | 997000 | 980296 |
| Execution slots | 11 | 1 |

## Scenario 3: Large pool, 0.5% swap

Reserve: 1000000000, Swap: 5000000, Drift trigger: 200000000

- Swap amount: 5000000
- Initial reserves: [1000000000, 1000000000]

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|--------|---------------:|------------:|
| Init CU | 0 | 13998 |
| Swap/Request CU | 9239 | 10964 |
| Clear/Rebalance CU | 14803 | 8632 |
| Claim CU | 3033 | N/A |
| **Total CU** | **27075** | **34760** |
| Tokens received | 4985000 | 4925619 |
| Execution slots | 11 | 1 |

## Scenario 4: Large pool, 1% swap

Reserve: 1000000000, Swap: 10000000, Drift trigger: 200000000

- Swap amount: 10000000
- Initial reserves: [1000000000, 1000000000]

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|--------|---------------:|------------:|
| Init CU | 0 | 12498 |
| Swap/Request CU | 9239 | 10975 |
| Clear/Rebalance CU | 14805 | 8633 |
| Claim CU | 3033 | N/A |
| **Total CU** | **27077** | **33272** |
| Tokens received | 9970000 | 9802951 |
| Execution slots | 11 | 1 |

## Summary

- Average total CU: ETF A = 28200, ETF B = 33642
- CU efficiency: ETF B uses 119% of ETF A's compute
