use anyhow::Result;

/// 1スロット観測結果（最小）
#[derive(Debug, Clone)]
pub struct SlotObservation {
    pub slot: usize,
    pub ext_price: f64,
    pub pre_pool_price: f64,
    pub post_pool_price: f64,
    pub pre_drift_abs_sum: f64,
    pub post_drift_abs_sum: f64,
    pub arb_fired: bool,
    pub arb_gross_usd: f64,
    pub arb_net_usd: f64,
    pub edge_bps_pre: f64,
    pub threshold_bps_used: f64,
}

/// 集計結果
#[derive(Debug, Clone)]
pub struct SimSummary {
    pub label: String,
    pub slots: usize,
    pub arb_count: usize,
    pub arb_rate: f64,
    pub avg_slots_between_arb: f64,
    pub median_slots_between_arb: f64,
    pub mean_drift_pre: f64,
    pub mean_drift_post: f64,
    pub total_arb_gross_usd: f64,
    pub total_arb_net_usd: f64,
    pub avg_extraction_per_arb: f64,
    pub avg_net_per_arb: f64,
    pub mean_edge_bps_when_arb: f64,
    pub mean_threshold_bps_used: f64,
}

/// 閾値モード
#[derive(Debug, Clone)]
pub enum ThresholdMode {
    /// 固定閾値（例: 2.5bps）
    Fixed(f64),
    /// 分位点ミックス（20/60/20）
    QuantileMixture {
        low_bps: f64,
        base_bps: f64,
        high_bps: f64,
        w_low: f64,
        w_base: f64,
        w_high: f64,
    },
}

impl ThresholdMode {
    pub fn describe(&self) -> String {
        match self {
            ThresholdMode::Fixed(v) => format!("Fixed({:.2}bps)", v),
            ThresholdMode::QuantileMixture {
                low_bps,
                base_bps,
                high_bps,
                w_low,
                w_base,
                w_high,
            } => format!(
                "Mixture[{:.0}%:{:.1}bps, {:.0}%:{:.1}bps, {:.0}%:{:.1}bps]",
                w_low * 100.0,
                low_bps,
                w_base * 100.0,
                base_bps,
                w_high * 100.0,
                high_bps
            ),
        }
    }

    pub fn sample_bps(&self, rng_state: &mut u64) -> f64 {
        match self {
            ThresholdMode::Fixed(v) => *v,
            ThresholdMode::QuantileMixture {
                low_bps,
                base_bps,
                high_bps,
                w_low,
                w_base,
                w_high,
            } => {
                let total = (*w_low + *w_base + *w_high).max(1e-12);
                let u = next_rand_unit(rng_state) * total;

                if u < *w_low {
                    *low_bps
                } else if u < *w_low + *w_base {
                    *base_bps
                } else {
                    *high_bps
                }
            }
        }
    }
}

/// sim設定（Phase 3.8.2 用）
#[derive(Debug, Clone)]
pub struct SimConfig {
    pub slots: usize,
    pub arb_cost_usd_per_trade: f64,
    pub threshold_mode: ThresholdMode,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            slots: 1200,
            arb_cost_usd_per_trade: 0.10, // 仮値（後でJito proxyで改善）
            threshold_mode: ThresholdMode::Fixed(2.5),
        }
    }
}

/// Phase 3.8.2: edgeスケールを実測レンジに寄せたslot観測モデル + threshold sampling
pub fn run_simulation_with_config(
    cfg: &SimConfig,
    label: impl Into<String>,
) -> Result<(SimSummary, Vec<SlotObservation>)> {
    let label = label.into();

    let slots = cfg.slots;
    let arb_cost = cfg.arb_cost_usd_per_trade;

    // 仮想価格・プール価格（表示用）
    let mut ext_price = 1.0_f64;
    let mut pool_price = 1.0_f64;

    // 擬似ランダム
    let mut rng_state: u64 = 0x1234_5678_9abc_def0;

    let mut obs = Vec::<SlotObservation>::with_capacity(slots);

    for slot in 0..slots {
        // ----- 外部価格を少し動かす（見た目用） -----
        let r = next_rand_unit(&mut rng_state);
        let centered = (r - 0.5) * 2.0;
        let drift_trend = 0.000002 * ((slot as f64) / 180.0).sin();
        let ret = centered * 0.00012 + drift_trend;
        ext_price *= 1.0 + ret;

        // ===== 実測っぽい pre-edge を生成 =====
        let edge_bps_pre = sample_observed_like_edge_bps(&mut rng_state);

        // 今回のポイント：thresholdを毎slotサンプル
        let threshold_bps_used = cfg.threshold_mode.sample_bps(&mut rng_state);

        // edge から pre_drift を作る（proxy）
        let pre_drift_abs_sum = edge_bps_pre / 10_000.0 * 0.6;
        let post_drift_abs_sum_if_arb = pre_drift_abs_sum * 0.5;
        let post_drift_abs_sum_no_arb = pre_drift_abs_sum * 0.95;

        // pool価格の差分を edge_bps に合わせて構成（表示用）
        let sign = if next_rand_unit(&mut rng_state) < 0.5 { -1.0 } else { 1.0 };
        let pre_pool_price = ext_price * (1.0 + sign * edge_bps_pre / 10_000.0);

        let arb_fired = edge_bps_pre >= threshold_bps_used;

        let (post_pool_price, post_drift_abs_sum, arb_gross_usd, arb_net_usd) = if arb_fired {
            let diff = pre_pool_price - ext_price;
            let post_price = ext_price + diff * 0.5;

            // 3.7.1の onchain notional に近いレンジ
            let notional_usd = 80.0 + 920.0 * next_rand_unit(&mut rng_state); // 80~1000
            let gross = (edge_bps_pre / 10_000.0) * notional_usd * 0.35;
            let net = (gross - arb_cost).max(0.0);

            (post_price, post_drift_abs_sum_if_arb, gross, net)
        } else {
            let post_price = pre_pool_price + (ext_price - pre_pool_price) * 0.05;
            (post_price, post_drift_abs_sum_no_arb, 0.0, 0.0)
        };

        pool_price = post_pool_price;

        obs.push(SlotObservation {
            slot,
            ext_price,
            pre_pool_price,
            post_pool_price: pool_price,
            pre_drift_abs_sum,
            post_drift_abs_sum,
            arb_fired,
            arb_gross_usd,
            arb_net_usd,
            edge_bps_pre,
            threshold_bps_used,
        });
    }

    let summary = summarize(&obs, label);
    Ok((summary, obs))
}

/// 固定3シナリオ（従来）
pub fn run_threshold_calibration_scenarios() -> Result<Vec<SimSummary>> {
    let scenarios = [
        ("Low (p10≈0.9bps)", ThresholdMode::Fixed(1.0_f64)),
        ("Base (p50≈2.4bps)", ThresholdMode::Fixed(2.5_f64)),
        ("High (p90≈4.6bps)", ThresholdMode::Fixed(4.5_f64)),
    ];

    let mut out = Vec::new();
    for (label, mode) in scenarios {
        let cfg = SimConfig {
            threshold_mode: mode,
            ..SimConfig::default()
        };
        let (summary, _obs) = run_simulation_with_config(&cfg, label)?;
        out.push(summary);
    }
    Ok(out)
}

/// Phase 3.8.2-A: 分位点ミックス閾値シナリオ
pub fn run_threshold_mixture_scenario() -> Result<SimSummary> {
    let cfg = SimConfig {
        threshold_mode: ThresholdMode::QuantileMixture {
            low_bps: 1.0,
            base_bps: 2.5,
            high_bps: 4.5,
            w_low: 0.20,
            w_base: 0.60,
            w_high: 0.20,
        },
        ..SimConfig::default()
    };

    let (summary, _obs) = run_simulation_with_config(&cfg, "Mixture (20/60/20)")?;
    Ok(summary)
}

/// 単発実行（CLI互換）
pub fn run_simulation() -> Result<()> {
    let cfg = SimConfig::default();
    let (summary, obs) =
        run_simulation_with_config(&cfg, format!("Single run ({})", cfg.threshold_mode.describe()))?;

    println!("=== TFMM Simulation (Phase 3.8.2 single run) ===");
    print_summary(&summary);

    println!("\n--- First 5 slots (pre/post preview) ---");
    for x in obs.iter().take(5) {
        println!(
            "slot={:4} ext={:.6} pre(px={:.6},dr={:.6}) post(px={:.6},dr={:.6}) arb={} gross={:.4} net={:.4} edge_bps={:.3} th_bps={:.3}",
            x.slot,
            x.ext_price,
            x.pre_pool_price,
            x.pre_drift_abs_sum,
            x.post_pool_price,
            x.post_drift_abs_sum,
            x.arb_fired,
            x.arb_gross_usd,
            x.arb_net_usd,
            x.edge_bps_pre,
            x.threshold_bps_used,
        );
    }

    Ok(())
}

/// 実測っぽい edge 分布をざっくり再現する簡易サンプラー
/// 目標感（Phase 3.7.1）: p10~0.9, p50~2.4, p90~4.6 bps
fn sample_observed_like_edge_bps(state: &mut u64) -> f64 {
    let u = next_rand_unit(state);

    if u < 0.10 {
        let v = next_rand_unit(state);
        0.01 + v * 0.89 // 0.01..0.90
    } else if u < 0.50 {
        let v = next_rand_unit(state);
        0.90 + v * 1.60 // 0.9..2.5
    } else if u < 0.90 {
        let v = next_rand_unit(state);
        2.50 + v * 2.10 // 2.5..4.6
    } else {
        let v = next_rand_unit(state);
        4.60 + v * 1.90 // 4.6..6.5
    }
}

fn summarize(obs: &[SlotObservation], label: String) -> SimSummary {
    let slots = obs.len();
    let arb_indices: Vec<usize> = obs
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.arb_fired.then_some(i))
        .collect();
    let arb_count = arb_indices.len();

    let arb_rate = if slots > 0 {
        arb_count as f64 / slots as f64
    } else {
        0.0
    };

    let mut gaps = Vec::<usize>::new();
    for w in arb_indices.windows(2) {
        gaps.push(w[1] - w[0]);
    }
    let avg_slots_between_arb = if gaps.is_empty() {
        slots as f64
    } else {
        gaps.iter().sum::<usize>() as f64 / gaps.len() as f64
    };
    let median_slots_between_arb = median_usize(&gaps).unwrap_or(slots as f64);

    let mean_drift_pre = mean(obs.iter().map(|x| x.pre_drift_abs_sum));
    let mean_drift_post = mean(obs.iter().map(|x| x.post_drift_abs_sum));

    let total_arb_gross_usd: f64 = obs.iter().map(|x| x.arb_gross_usd).sum();
    let total_arb_net_usd: f64 = obs.iter().map(|x| x.arb_net_usd).sum();

    let avg_extraction_per_arb = if arb_count > 0 {
        total_arb_gross_usd / arb_count as f64
    } else {
        0.0
    };
    let avg_net_per_arb = if arb_count > 0 {
        total_arb_net_usd / arb_count as f64
    } else {
        0.0
    };

    let mean_edge_bps_when_arb = {
        let xs: Vec<f64> = obs
            .iter()
            .filter(|x| x.arb_fired)
            .map(|x| x.edge_bps_pre)
            .collect();
        mean(xs.into_iter())
    };

    let mean_threshold_bps_used = mean(obs.iter().map(|x| x.threshold_bps_used));

    SimSummary {
        label,
        slots,
        arb_count,
        arb_rate,
        avg_slots_between_arb,
        median_slots_between_arb,
        mean_drift_pre,
        mean_drift_post,
        total_arb_gross_usd,
        total_arb_net_usd,
        avg_extraction_per_arb,
        avg_net_per_arb,
        mean_edge_bps_when_arb,
        mean_threshold_bps_used,
    }
}

pub fn print_summary(s: &SimSummary) {
    println!("label                 : {}", s.label);
    println!("slots                 : {}", s.slots);
    println!("arb_count             : {}", s.arb_count);
    println!("arb_rate              : {:.4} ({:.2}%)", s.arb_rate, s.arb_rate * 100.0);
    println!("avg_slots_between_arb : {:.2}", s.avg_slots_between_arb);
    println!("median_slots_between_arb: {:.2}", s.median_slots_between_arb);
    println!("mean_drift_pre        : {:.6}", s.mean_drift_pre);
    println!("mean_drift_post       : {:.6}", s.mean_drift_post);
    println!("total_arb_gross_usd   : {:.4}", s.total_arb_gross_usd);
    println!("total_arb_net_usd     : {:.4}", s.total_arb_net_usd);
    println!("avg_extraction_per_arb: {:.4}", s.avg_extraction_per_arb);
    println!("avg_net_per_arb       : {:.4}", s.avg_net_per_arb);
    println!("mean_edge_bps_when_arb: {:.4}", s.mean_edge_bps_when_arb);
    println!("mean_threshold_bps_used: {:.4}", s.mean_threshold_bps_used);
}

fn mean<I>(iter: I) -> f64
where
    I: IntoIterator<Item = f64>,
{
    let mut sum = 0.0;
    let mut n = 0usize;
    for x in iter {
        sum += x;
        n += 1;
    }
    if n == 0 { 0.0 } else { sum / n as f64 }
}

fn median_usize(xs: &[usize]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let mut v = xs.to_vec();
    v.sort_unstable();
    let n = v.len();
    if n % 2 == 1 {
        Some(v[n / 2] as f64)
    } else {
        Some((v[n / 2 - 1] as f64 + v[n / 2] as f64) / 2.0)
    }
}

fn next_rand_unit(state: &mut u64) -> f64 {
    // xorshift64*
    let mut x = *state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *state = x;
    let z = x.wrapping_mul(0x2545F4914F6CDD1D);
    ((z >> 11) as f64) / ((1u64 << 53) as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_scenarios_run() {
        let xs = run_threshold_calibration_scenarios().unwrap();
        assert_eq!(xs.len(), 3);
    }

    #[test]
    fn mixture_scenario_runs() {
        let s = run_threshold_mixture_scenario().unwrap();
        assert_eq!(s.slots, 1200);
    }

    #[test]
    fn threshold_changes_arb_count_monotonically_for_fixed() {
        let (low, _) = run_simulation_with_config(
            &SimConfig {
                threshold_mode: ThresholdMode::Fixed(1.0),
                ..Default::default()
            },
            "low",
        )
        .unwrap();
        let (base, _) = run_simulation_with_config(
            &SimConfig {
                threshold_mode: ThresholdMode::Fixed(2.5),
                ..Default::default()
            },
            "base",
        )
        .unwrap();
        let (high, _) = run_simulation_with_config(
            &SimConfig {
                threshold_mode: ThresholdMode::Fixed(4.5),
                ..Default::default()
            },
            "high",
        )
        .unwrap();

        assert!(low.arb_count >= base.arb_count);
        assert!(base.arb_count >= high.arb_count);
    }
}