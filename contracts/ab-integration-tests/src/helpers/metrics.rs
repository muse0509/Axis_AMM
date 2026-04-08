use serde::Serialize;

/// Metrics collected from a G3M (ETF B) test run.
#[derive(Debug, Default, Clone, Serialize)]
pub struct G3mMetrics {
    pub init_cu: u64,
    pub swap_cu: u64,
    pub check_drift_cu: u64,
    pub rebalance_cu: u64,
    pub pre_k: u128,
    pub post_k: u128,
    pub pre_reserves: Vec<u64>,
    pub post_reserves: Vec<u64>,
    pub total_slots: u64,
}

/// Metrics collected from a PFDA-3 (ETF A) test run.
#[derive(Debug, Default, Clone, Serialize)]
pub struct Pfda3Metrics {
    pub init_cu: u64,
    pub add_liq_cu: u64,
    pub swap_request_cu: u64,
    pub clear_batch_cu: u64,
    pub claim_cu: u64,
    pub clearing_prices: [u64; 3],
    pub total_value_in: u64,
    pub tokens_received: u64,
    pub batch_window_slots: u64,
    pub total_slots: u64,
}

/// Full A/B comparison report.
#[derive(Debug, Serialize)]
pub struct ABReport {
    pub generated_at: String,
    pub environment: String,
    pub scenarios: Vec<ABScenario>,
}

/// A single A/B test scenario (e.g. "balanced pool", "imbalanced pool", "large swap").
#[derive(Debug, Serialize)]
pub struct ABScenario {
    pub name: String,
    pub description: String,
    pub swap_amount: u64,
    pub initial_reserves: Vec<u64>,
    pub g3m: G3mMetrics,
    pub pfda3: Pfda3Metrics,
}

impl ABReport {
    pub fn new(environment: &str) -> Self {
        ABReport {
            generated_at: chrono_lite_now(),
            environment: environment.to_string(),
            scenarios: Vec::new(),
        }
    }

    pub fn add_scenario(&mut self, s: ABScenario) {
        self.scenarios.push(s);
    }

    /// Export as JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Export as Markdown string.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Axis A/B Test Report\n\n");
        md.push_str(&format!("- Generated: {}\n", self.generated_at));
        md.push_str(&format!("- Environment: {}\n\n", self.environment));

        for (i, s) in self.scenarios.iter().enumerate() {
            md.push_str(&format!("## Scenario {}: {}\n\n", i + 1, s.name));
            md.push_str(&format!("{}\n\n", s.description));
            md.push_str(&format!("- Swap amount: {}\n", s.swap_amount));
            md.push_str(&format!("- Initial reserves: {:?}\n\n", s.initial_reserves));

            let g = &s.g3m;
            let p = &s.pfda3;
            let g_total = g.init_cu + g.swap_cu + g.check_drift_cu + g.rebalance_cu;
            let p_total = p.init_cu + p.add_liq_cu + p.swap_request_cu + p.clear_batch_cu + p.claim_cu;

            md.push_str("| Metric | ETF A (PFDA-3) | ETF B (G3M) |\n");
            md.push_str("|--------|---------------:|------------:|\n");
            md.push_str(&format!("| Init CU | {} | {} |\n", p.init_cu, g.init_cu));
            md.push_str(&format!("| Swap/Request CU | {} | {} |\n", p.swap_request_cu, g.swap_cu));
            md.push_str(&format!("| Clear/Rebalance CU | {} | {} |\n", p.clear_batch_cu, g.rebalance_cu));
            md.push_str(&format!("| Claim CU | {} | N/A |\n", p.claim_cu));
            md.push_str(&format!("| **Total CU** | **{}** | **{}** |\n", p_total, g_total));
            md.push_str(&format!("| Tokens received | {} | — |\n", p.tokens_received));
            md.push_str(&format!("| Execution slots | {} | {} |\n", p.total_slots, g.total_slots));

            if g.pre_k > 0 {
                let delta = ((g.post_k as i128 - g.pre_k as i128) * 10_000 / g.pre_k as i128) as i64;
                md.push_str(&format!("| Invariant delta (bps) | — | {} |\n", delta));
            }
            md.push_str("\n");
        }

        // Summary
        if self.scenarios.len() > 1 {
            md.push_str("## Summary\n\n");
            let avg_g: u64 = self.scenarios.iter()
                .map(|s| s.g3m.init_cu + s.g3m.swap_cu + s.g3m.check_drift_cu + s.g3m.rebalance_cu)
                .sum::<u64>() / self.scenarios.len() as u64;
            let avg_p: u64 = self.scenarios.iter()
                .map(|s| s.pfda3.init_cu + s.pfda3.add_liq_cu + s.pfda3.swap_request_cu + s.pfda3.clear_batch_cu + s.pfda3.claim_cu)
                .sum::<u64>() / self.scenarios.len() as u64;
            md.push_str(&format!("- Average total CU: ETF A = {}, ETF B = {}\n", avg_p, avg_g));
            md.push_str(&format!("- CU efficiency: ETF B uses {:.0}% of ETF A's compute\n",
                avg_g as f64 / avg_p.max(1) as f64 * 100.0));
        }

        md
    }

    /// Print table to stdout.
    pub fn print_table(&self) {
        for s in &self.scenarios {
            let g = &s.g3m;
            let p = &s.pfda3;

            println!();
            println!("━━━ {} ━━━", s.name);
            println!("╔════════════════════════╤══════════════════╤══════════════════╗");
            println!("║  Metric                │  ETF A (PFDA-3)  │  ETF B (G3M)     ║");
            println!("╠════════════════════════╪══════════════════╪══════════════════╣");
            println!("║  Init CU              │  {:>14}  │  {:>14}  ║", p.init_cu, g.init_cu);
            println!("║  Swap/SwapRequest CU  │  {:>14}  │  {:>14}  ║", p.swap_request_cu, g.swap_cu);
            println!("║  Clear/Rebalance CU   │  {:>14}  │  {:>14}  ║", p.clear_batch_cu, g.rebalance_cu);
            println!("║  Claim CU             │  {:>14}  │  {:>14}  ║", p.claim_cu, "N/A");
            println!("║  Total CU             │  {:>14}  │  {:>14}  ║",
                p.init_cu + p.add_liq_cu + p.swap_request_cu + p.clear_batch_cu + p.claim_cu,
                g.init_cu + g.swap_cu + g.check_drift_cu + g.rebalance_cu);
            println!("╠════════════════════════╪══════════════════╪══════════════════╣");
            println!("║  Tokens received      │  {:>14}  │  {:>14}  ║", p.tokens_received, "—");
            println!("║  Execution slots      │  {:>14}  │  {:>14}  ║", p.total_slots, g.total_slots);
            println!("╚════════════════════════╧══════════════════╧══════════════════╝");
        }
    }
}

/// Lightweight timestamp (no chrono dependency).
fn chrono_lite_now() -> String {
    // Use a fixed format for reproducibility in CI.
    // In real usage this would use std::time::SystemTime.
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}s-since-epoch", d.as_secs())
}

// Legacy compat
pub type ABComparison = ABReport;
