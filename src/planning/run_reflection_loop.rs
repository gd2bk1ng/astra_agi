// ============================================================================
//               ASTRA AGI • SELF-REFLECTION & META-LEARNING LOOP
//        Continuous Review of Decisions, Heuristic Tuning & Strategy Adaptation
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Implements Astra’s background self-reflection process, periodically
//       reviewing recent plans, actions, and outcomes to refine internal
//       heuristics and planning strategies. This loop acts as a meta-cognitive
//       layer that learns *how to think better over time*, rather than only
//       learning about the external world.
//
//   Core Functions:
//       • Periodically sample recent episodes from memory and logs
//       • Evaluate plan quality, execution success, and decision efficiency
//       • Adjust planning heuristics and strategy selection policies
//       • Emit updated meta-parameters back into the Planning and Learning subsystems
//
//   File:        /src/planning/run_reflection_loop.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use tokio::time::{sleep, Duration};
use log::{info, warn};

/// How often Astra reflects on her own decisions and strategies.
const REFLECTION_INTERVAL: Duration = Duration::from_secs(120);

/// Parameters controlling how aggressively heuristics are updated.
#[derive(Debug, Clone)]
pub struct ReflectionConfig {
    /// Weight given to recent episodes vs. older history.
    pub recency_bias: f32,
    /// Learning rate for heuristic updates.
    pub meta_learning_rate: f32,
    /// Minimum number of episodes required before adjusting heuristics.
    pub min_episodes_for_update: usize,
}

impl Default for ReflectionConfig {
    fn default() -> Self {
        Self {
            recency_bias: 0.7,
            meta_learning_rate: 0.1,
            min_episodes_for_update: 5,
        }
    }
}

/// Abstract view of an episode Astra can reflect on.
#[derive(Debug, Clone)]
pub struct DecisionEpisode {
    pub goal_id: String,
    pub strategy_used: String, // e.g., "HTN", "GOAP", "Reactive"
    pub success: bool,
    pub total_cost: f32,
    pub duration_ms: u64,
}

/// Aggregated reflection result used to adjust heuristics.
#[derive(Debug, Clone)]
pub struct ReflectionSummary {
    pub strategy_scores: std::collections::HashMap<String, f32>,
}

/// Runs the self-reflection loop indefinitely.
/// In a full system, this would pull from real memory/logs and update real planners.
pub async fn run_reflection_loop() {
    let config = ReflectionConfig::default();

    loop {
        info!("[Reflection Loop] Reviewing recent decisions and strategies...");
        if let Err(e) = run_single_reflection_cycle(&config).await {
            warn!("[Reflection Loop] Error during reflection cycle: {}", e);
        }
        sleep(REFLECTION_INTERVAL).await;
    }
}

/// Runs a single reflection cycle: gather episodes, analyze, and adjust heuristics.
async fn run_single_reflection_cycle(config: &ReflectionConfig) -> anyhow::Result<()> {
    // 1. Retrieve recent decision episodes from memory/logs.
    let episodes = fetch_recent_decision_episodes().await?;

    if episodes.len() < config.min_episodes_for_update {
        info!(
            "[Reflection Loop] Not enough episodes for update ({} / {}). Skipping.",
            episodes.len(),
            config.min_episodes_for_update
        );
        return Ok(());
    }

    // 2. Analyze episodes and compute reflection summary.
    let summary = analyze_episodes(&episodes, config);

    // 3. Apply heuristic updates to planning subsystem.
    apply_planning_heuristic_updates(&summary, config).await?;

    // 4. Optionally feed back into learning subsystem for meta-learning.
    apply_meta_learning_updates(&episodes, &summary, config).await?;

    Ok(())
}

/// Placeholder: fetch recent decision episodes from memory/logging.
/// In a full implementation, this would query the Narrative Memory System
/// and/or structured planning logs.
async fn fetch_recent_decision_episodes() -> anyhow::Result<Vec<DecisionEpisode>> {
    // TODO: Integrate with /src/memory and planning logs.
    Ok(vec![
        DecisionEpisode {
            goal_id: "light_on".into(),
            strategy_used: "GOAP".into(),
            success: true,
            total_cost: 3.0,
            duration_ms: 120,
        },
        DecisionEpisode {
            goal_id: "light_on".into(),
            strategy_used: "Reactive".into(),
            success: false,
            total_cost: 1.0,
            duration_ms: 30,
        },
    ])
}

/// Analyzes decision episodes and assigns scores to strategies based on success,
/// efficiency, and cost. Higher score means better performance.
fn analyze_episodes(
    episodes: &[DecisionEpisode],
    config: &ReflectionConfig,
) -> ReflectionSummary {
    use std::collections::HashMap;

    let mut scores: HashMap<String, (f32, f32)> = HashMap::new(); // (score_sum, weight_sum)

    for (i, ep) in episodes.iter().enumerate() {
        // Recency weighting: newer episodes have more influence.
        let recency_weight = ((i + 1) as f32 / episodes.len() as f32).powf(config.recency_bias);

        let base = if ep.success { 1.0 } else { -1.0 };
        let cost_penalty = (ep.total_cost / 10.0).min(1.0);
        let time_penalty = (ep.duration_ms as f32 / 1000.0).min(1.0);

        let score = base - 0.5 * cost_penalty - 0.3 * time_penalty;

        let entry = scores.entry(ep.strategy_used.clone()).or_insert((0.0, 0.0));
        entry.0 += score * recency_weight;
        entry.1 += recency_weight;
    }

    let strategy_scores = scores
        .into_iter()
        .map(|(k, (sum, w))| (k, if w > 0.0 { sum / w } else { 0.0 }))
        .collect();

    ReflectionSummary { strategy_scores }
}

/// Applies heuristic updates to the planning subsystem based on reflection.
/// In a full system, this might tune:
//  • Strategy selection thresholds (when to use HTN vs GOAP vs Reactive)
//  • Cost weighting for time vs resource usage
///  • Exploration vs exploitation parameters
async fn apply_planning_heuristic_updates(
    summary: &ReflectionSummary,
    config: &ReflectionConfig,
) -> anyhow::Result<()> {
    // TODO: Wire into actual planner configuration (e.g., via a shared state, config service, or
    //       direct mutation of Planner behavior).
    for (strategy, score) in &summary.strategy_scores {
        info!(
            "[Reflection Loop] Strategy '{}' scored {:.3} (lr = {}).",
            strategy, score, config.meta_learning_rate
        );
    }

    // Example placeholder:
    // if summary.strategy_scores["GOAP"] > summary.strategy_scores["Reactive"] {
    //     planner.set_preference(PlanningStrategy::Goap, ...);
    // }

    Ok(())
}

/// Applies meta-learning updates, potentially using the Learning subsystem to
/// refine internal models that predict which strategies work best in which contexts.
async fn apply_meta_learning_updates(
    _episodes: &[DecisionEpisode],
    _summary: &ReflectionSummary,
    _config: &ReflectionConfig,
) -> anyhow::Result<()> {
    // TODO: Integrate with /src/learning:
    //  • Train a model that maps (goal, context) -> optimal strategy
    //  • Update value estimates for certain plan patterns
    //  • Store meta-experiences in Narrative Memory for long-term reflection
    Ok(())
}
