use anyhow::Result;
use std::collections::BTreeMap;

pub fn run_sim() -> Result<()> {
    println!("=== Phase 3.8.2-A: Sim threshold calibration (fixed + mixture) ===");
    println!("Observed proxy source: Coinbase-filtered edge distribution (Phase 3.7.1)");
    println!("Fixed thresholds:");
    println!("  Low  = 1.0 bps (≈ p10)");
    println!("  Base = 2.5 bps (≈ p50)");
    println!("  High = 4.5 bps (≈ p90)");
    println!("Mixture threshold:");
    println!("  20% @ 1.0bps, 60% @ 2.5bps, 20% @ 4.5bps\n");

    let mut summaries = tfmm_sim::run_threshold_calibration_scenarios()?;
    let mixture = tfmm_sim::run_threshold_mixture_scenario()?;
    summaries.push(mixture);

    println!("--- Scenario summaries ---");
    for s in &summaries {
        tfmm_sim::print_summary(s);
        println!();
    }

    println!("--- Comparison (key metrics) ---");
    println!(
        "{:<20} {:>8} {:>10} {:>12} {:>12} {:>12} {:>12}",
        "scenario", "arb_rate%", "avg_gap", "avg_gross", "avg_net", "mean_edge", "mean_th"
    );
    for s in &summaries {
        println!(
            "{:<20} {:>8.2} {:>10.2} {:>12.4} {:>12.4} {:>12.4} {:>12.4}",
            truncate_label(&s.label, 20),
            s.arb_rate * 100.0,
            s.avg_slots_between_arb,
            s.avg_extraction_per_arb,
            s.avg_net_per_arb,
            s.mean_edge_bps_when_arb,
            s.mean_threshold_bps_used,
        );
    }

    println!("\nInterpretation hint:");
    println!("- Mixture scenario should usually land between Base and Low/High on most metrics.");
    println!("- It can be a better 'default crowd model' than a single fixed threshold.");
    println!("- Keep fixed scenarios for sensitivity analysis.");

    Ok(())
}

// ===== real / live (Phase 3.7.1 Coinbase版のまま) =====

pub fn run_real(pool: &str) -> Result<()> {
    println!("=== Phase 3.7.1: Helius + Coinbase (edge_bps_at_trigger with match-dt filter) ===");
    println!("pool address: {pool}");

    const MAX_MATCH_DT_MS: i64 = 2_000;

    let helius = tfmm_ingest::HeliusClient::from_env()?;

    let txs = match helius.get_address_transactions(pool, 100, Some("SWAP"), None) {
        Ok(v) => {
            println!("fetched {} txs (type=SWAP)", v.len());
            v
        }
        Err(e) => {
            println!("SWAP filter fetch failed: {e}");
            println!("fallback: fetching without type filter...");
            let v = helius.get_address_transactions(pool, 100, None, None)?;
            println!("fetched {} txs (no type filter)", v.len());
            v
        }
    };

    if txs.is_empty() {
        println!("No transactions returned.");
        return Ok(());
    }

    let trades = tfmm_ingest::extract_swap_trade_previews(&txs);
    let (norm_trades, norm_report) = tfmm_ingest::normalize_sol_usdc_trades(&trades);

    println!("\n--- Normalization report (SOL/USDC only) ---");
    println!("total_trades             : {}", norm_report.total_trades);
    println!("normalized_sol_usdc      : {}", norm_report.normalized_sol_usdc);
    println!("dropped_not_sol_usdc_pair: {}", norm_report.dropped_not_sol_usdc_pair);
    println!("dropped_invalid_amount   : {}", norm_report.dropped_invalid_amount);

    if norm_trades.is_empty() {
        println!("No normalized SOL/USDC trades found.");
        return Ok(());
    }

    let min_ts_sec = norm_trades.iter().filter_map(|t| t.timestamp).min();
    let max_ts_sec = norm_trades.iter().filter_map(|t| t.timestamp).max();

    let (min_ts_sec, max_ts_sec) = match (min_ts_sec, max_ts_sec) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            println!("No timestamps on normalized trades; cannot align external price.");
            return Ok(());
        }
    };

    let start_ms = (min_ts_sec - 5).max(0) * 1000;
    let end_ms = (max_ts_sec + 5).max(0) * 1000;

    println!("\nHelius normalized trade window:");
    println!("  min_ts_sec: {}", min_ts_sec);
    println!("  max_ts_sec: {}", max_ts_sec);
    println!("  query Coinbase trades range: {} .. {} (ms)", start_ms, end_ms);

    let coinbase = tfmm_ingest::coinbase::CoinbaseClient::new()?;
    let cb_trades =
        coinbase.get_trades_covering_range("SOL-USD", start_ms, end_ms, 1000, 10)?;

    println!("\nCoinbase trades fetched (in range): {}", cb_trades.len());

    if cb_trades.is_empty() {
        println!("No Coinbase trades returned in time window.");
        println!("(Try wider window / more pages / rerun shortly)");
        return Ok(());
    }

    #[derive(Debug, Clone)]
    struct MatchedTradeEdge {
        slot: u64,
        tx_ts_sec: i64,
        exec_price_usdc_per_sol: f64,
        ext_price_usd_per_sol: f64,
        edge_bps: f64,
        match_time_diff_ms: i64,
        notional_usdc: f64,
        fee_lamports: Option<u64>,
    }

    let mut matched = Vec::<MatchedTradeEdge>::new();
    let mut unmatched_count = 0usize;
    let mut dropped_by_match_dt = 0usize;

    for t in &norm_trades {
        let ts_sec = match t.timestamp {
            Some(v) => v,
            None => {
                unmatched_count += 1;
                continue;
            }
        };
        let ts_ms = ts_sec * 1000;

        if let Some(m) = tfmm_ingest::coinbase::nearest_price_match(&cb_trades, ts_ms) {
            if m.abs_time_diff_ms > MAX_MATCH_DT_MS {
                dropped_by_match_dt += 1;
                continue;
            }

            if let Some(edge) =
                tfmm_ingest::coinbase::edge_bps(t.exec_price_usdc_per_sol, m.price_usd_per_sol)
            {
                matched.push(MatchedTradeEdge {
                    slot: t.slot,
                    tx_ts_sec: ts_sec,
                    exec_price_usdc_per_sol: t.exec_price_usdc_per_sol,
                    ext_price_usd_per_sol: m.price_usd_per_sol,
                    edge_bps: edge,
                    match_time_diff_ms: m.abs_time_diff_ms,
                    notional_usdc: t.notional_usdc,
                    fee_lamports: t.fee,
                });
            } else {
                unmatched_count += 1;
            }
        } else {
            unmatched_count += 1;
        }
    }

    println!("\nMatch filter config:");
    println!("  max_match_dt_ms         : {}", MAX_MATCH_DT_MS);

    println!("\nMatched trade edges: {}", matched.len());
    println!("Unmatched / skipped : {}", unmatched_count);
    println!("Dropped by match dt : {}", dropped_by_match_dt);

    if matched.is_empty() {
        println!("No matched trades with Coinbase external price after dt filter.");
        println!("Try relaxing MAX_MATCH_DT_MS to 5000 or 10000.");
        return Ok(());
    }

    println!("\n--- First 5 matched trade edges ---");
    for (i, m) in matched.iter().take(5).enumerate() {
        println!(
            "[{}] slot={} ts={} exec={:.6} ext={:.6} edge_bps={:.3} dt_ms={} notional_usdc={:.3} fee={:?}",
            i,
            m.slot,
            m.tx_ts_sec,
            m.exec_price_usdc_per_sol,
            m.ext_price_usd_per_sol,
            m.edge_bps,
            m.match_time_diff_ms,
            m.notional_usdc,
            m.fee_lamports
        );
    }

    let mut edges: Vec<f64> = matched.iter().map(|m| m.edge_bps).collect();
    edges.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let edge_min = *edges.first().unwrap_or(&0.0);
    let edge_p10 = percentile_sorted(&edges, 0.10).unwrap_or(0.0);
    let edge_p25 = percentile_sorted(&edges, 0.25).unwrap_or(0.0);
    let edge_p50 = percentile_sorted(&edges, 0.50).unwrap_or(0.0);
    let edge_p75 = percentile_sorted(&edges, 0.75).unwrap_or(0.0);
    let edge_p90 = percentile_sorted(&edges, 0.90).unwrap_or(0.0);
    let edge_max = *edges.last().unwrap_or(&0.0);
    let edge_mean = edges.iter().sum::<f64>() / edges.len() as f64;

    let mean_match_dt_ms =
        matched.iter().map(|m| m.match_time_diff_ms as f64).sum::<f64>() / matched.len() as f64;
    let max_match_dt_observed = matched
        .iter()
        .map(|m| m.match_time_diff_ms)
        .max()
        .unwrap_or(0);

    #[derive(Debug, Clone)]
    struct SlotEdgeAgg {
        slot: u64,
        trade_count: usize,
        mean_edge_bps: f64,
        min_edge_bps: f64,
        max_edge_bps: f64,
        total_notional_usdc: f64,
        mean_match_dt_ms: f64,
    }

    let mut slot_map: BTreeMap<u64, Vec<&MatchedTradeEdge>> = BTreeMap::new();
    for m in &matched {
        slot_map.entry(m.slot).or_default().push(m);
    }

    let mut slot_aggs = Vec::<SlotEdgeAgg>::new();
    for (slot, xs) in slot_map {
        let trade_count = xs.len();
        let edge_vals: Vec<f64> = xs.iter().map(|x| x.edge_bps).collect();

        let mean_edge = edge_vals.iter().sum::<f64>() / edge_vals.len() as f64;
        let min_edge = edge_vals.iter().copied().fold(f64::INFINITY, f64::min);
        let max_edge = edge_vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        let total_notional_usdc = xs.iter().map(|x| x.notional_usdc).sum::<f64>();
        let mean_dt_ms = xs
            .iter()
            .map(|x| x.match_time_diff_ms as f64)
            .sum::<f64>()
            / xs.len() as f64;

        slot_aggs.push(SlotEdgeAgg {
            slot,
            trade_count,
            mean_edge_bps: mean_edge,
            min_edge_bps: min_edge,
            max_edge_bps: max_edge,
            total_notional_usdc,
            mean_match_dt_ms: mean_dt_ms,
        });
    }

    slot_aggs.sort_by_key(|s| s.slot);

    println!("\n--- Slot-level edge aggregation (first 10) ---");
    for s in slot_aggs.iter().take(10) {
        println!(
            "slot={} trades={} mean_edge_bps={:.3} min={:.3} max={:.3} total_notional_usdc={:.3} mean_match_dt_ms={:.1}",
            s.slot,
            s.trade_count,
            s.mean_edge_bps,
            s.min_edge_bps,
            s.max_edge_bps,
            s.total_notional_usdc,
            s.mean_match_dt_ms
        );
    }

    let total_notional: f64 = matched.iter().map(|m| m.notional_usdc).sum();
    let slot_count = slot_aggs.len();

    println!("\nsummary (edge_bps_at_trigger proxy, Coinbase SOL-USD as external):");
    println!("  raw txs                      : {}", txs.len());
    println!("  parsed trades (all)          : {}", trades.len());
    println!("  normalized SOL/USDC trades   : {}", norm_trades.len());
    println!("  matched trades w/ Coinbase   : {}", matched.len());
    println!("  unmatched/skipped            : {}", unmatched_count);
    println!("  dropped by match dt filter   : {}", dropped_by_match_dt);
    println!("  unique slots (matched)       : {}", slot_count);
    println!("  total notional usdc (matched): {:.3}", total_notional);
    println!("  mean Coinbase match dt       : {:.1} ms", mean_match_dt_ms);
    println!("  max Coinbase match dt        : {} ms", max_match_dt_observed);

    println!("\ntrade-level edge_bps distribution:");
    println!("  min  : {:.3}", edge_min);
    println!("  p10  : {:.3}", edge_p10);
    println!("  p25  : {:.3}", edge_p25);
    println!("  p50  : {:.3}", edge_p50);
    println!("  p75  : {:.3}", edge_p75);
    println!("  p90  : {:.3}", edge_p90);
    println!("  max  : {:.3}", edge_max);
    println!("  mean : {:.3}", edge_mean);

    println!("\nNext (Phase 3.8): calibrate sim threshold model from filtered Coinbase-based edge distribution");

    Ok(())
}

pub fn run_live(pool: &str) -> Result<()> {
    println!("live command reached (placeholder) - pool={pool}");
    println!("Next: wire this to realtime Helius + Coinbase streams");
    Ok(())
}

fn truncate_label(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

fn percentile_sorted(xs: &[f64], p: f64) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    if !(0.0..=1.0).contains(&p) {
        return None;
    }

    let n = xs.len();
    if n == 1 {
        return Some(xs[0]);
    }

    let pos = p * (n as f64 - 1.0);
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;

    if lo == hi {
        Some(xs[lo])
    } else {
        let w = pos - lo as f64;
        Some(xs[lo] * (1.0 - w) + xs[hi] * w)
    }
}