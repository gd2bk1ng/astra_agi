// ============================================================================
//                     ASTRA AGI • EPISODIC MEMORY SAMPLER
//        Pattern Extraction, Theme Detection & Experience Clustering
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Samples recent episodes from Narrative Memory, clusters them by theme,
//       and extracts patterns that feed into consolidation, reflection, and
//       long-term preference shaping.
//
//   Core Functions:
//       • Retrieve recent episodes from memory
//       • Cluster by theme, emotion, and outcome
//       • Produce summaries for consolidation and learning
//
//   File:        /src/cognition/episodic_sampler.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-12
//   Updated:     2026-01-12
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::astra_memory::Episode;

#[derive(Debug, Clone)]
pub struct EpisodeCluster {
    pub theme: String,
    pub count: usize,
    pub success_rate: f32,
}

pub fn sample_recent_episodes(episodes: &[Episode]) -> Vec<EpisodeCluster> {
    let mut clusters = Vec::new();

    let help_related: Vec<_> = episodes
        .iter()
        .filter(|e| e.description.contains("help"))
        .collect();

    if !help_related.is_empty() {
        let success_rate = help_related.iter().filter(|e| e.success).count() as f32
            / help_related.len() as f32;

        clusters.push(EpisodeCluster {
            theme: "helping_behavior".into(),
            count: help_related.len(),
            success_rate,
        });
    }

    clusters
}
