# Axis A/B Test Report

- Generated: 1776135164s-since-epoch
- Environment: LiteSVM (local, multi-scenario)

## Scenario 1: Small pool, tiny swap

Reserve: 1000000, Swap: 10000, Drift trigger: 200000

- Swap amount: 10000
- Initial reserves: [1000000, 1000000]

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|--------|---------------:|------------:|
| Init CU | 0 | 13992 |
| Swap/Request CU | 7740 | 10977 |
| Clear/Rebalance CU | 10301 | 8635 |
| Claim CU | 3033 | N/A |
| **Total CU** | **21074** | **34770** |
| Tokens received | 9970 | 9803 |
| Execution slots | 11 | 1 |

## Scenario 2: Medium pool, 1% swap

Reserve: 100000000, Swap: 1000000, Drift trigger: 20000000

- Swap amount: 1000000
- Initial reserves: [100000000, 100000000]

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|--------|---------------:|------------:|
| Init CU | 0 | 17000 |
| Swap/Request CU | 13740 | 10969 |
| Clear/Rebalance CU | 20803 | 8626 |
| Claim CU | 3033 | N/A |
| **Total CU** | **37576** | **37761** |
| Tokens received | 997000 | 980296 |
| Execution slots | 11 | 1 |

## Scenario 3: Large pool, 0.5% swap

Reserve: 1000000000, Swap: 5000000, Drift trigger: 200000000

- Swap amount: 5000000
- Initial reserves: [1000000000, 1000000000]

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|--------|---------------:|------------:|
| Init CU | 0 | 10998 |
| Swap/Request CU | 12240 | 10964 |
| Clear/Rebalance CU | 14802 | 8628 |
| Claim CU | 3033 | N/A |
| **Total CU** | **30075** | **31756** |
| Tokens received | 4985000 | 4925619 |
| Execution slots | 11 | 1 |

## Scenario 4: Large pool, 1% swap

Reserve: 1000000000, Swap: 10000000, Drift trigger: 200000000

- Swap amount: 10000000
- Initial reserves: [1000000000, 1000000000]

| Metric | ETF A (PFDA-3) | ETF B (G3M) |
|--------|---------------:|------------:|
| Init CU | 0 | 12498 |
| Swap/Request CU | 9240 | 10975 |
| Clear/Rebalance CU | 14804 | 8629 |
| Claim CU | 3033 | N/A |
| **Total CU** | **27077** | **33268** |
| Tokens received | 9970000 | 9802951 |
| Execution slots | 11 | 1 |

## Summary

- Average total CU: ETF A = 28950, ETF B = 34388
- CU efficiency: ETF B uses 119% of ETF A's compute
