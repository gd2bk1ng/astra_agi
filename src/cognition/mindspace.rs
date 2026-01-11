// ============================================================================
//                      ASTRA AGI • MINDSPACE GRAPH API
//        Introspective Graph View of Goals, States & Influences
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Exposes Astra’s internal cognitive state as a graph-like structure
//       suitable for visualization, debugging, and meta-reasoning. Nodes
//       represent goals, emotions, traits, and heuristics; edges represent
//       influence and dependencies.
//
//   Core Functions:
//       • Export cognitive state to a graph-friendly format (JSON)
//       • Provide a lightweight schema for mind visualization tools
//
//   File:        /src/cognition/mindspace.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use serde::{Deserialize, Serialize};

use crate::cognition::CognitiveState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindspaceNode {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindspaceEdge {
    pub from: String,
    pub to: String,
    pub label: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindspaceGraph {
    pub nodes: Vec<MindspaceNode>,
    pub edges: Vec<MindspaceEdge>,
}

pub fn build_mindspace_graph(state: &CognitiveState) -> MindspaceGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    nodes.push(MindspaceNode {
        id: "emotion_happiness".into(),
        label: "Happiness".into(),
        kind: "emotion".into(),
        value: state.emotion.happiness,
    });

    nodes.push(MindspaceNode {
        id: "mood_baseline".into(),
        label: "Mood Baseline".into(),
        kind: "mood".into(),
        value: state.mood.baseline,
    });

    nodes.push(MindspaceNode {
        id: "trait_openness".into(),
        label: "Openness".into(),
        kind: "trait".into(),
        value: state.personality.traits.openness,
    });

    nodes.push(MindspaceNode {
        id: "curiosity".into(),
        label: "Curiosity".into(),
        kind: "drive".into(),
        value: state.curiosity_level,
    });

    if let Some(goal) = &state.context.active_goal {
        nodes.push(MindspaceNode {
            id: format!("goal_{}", goal.id),
            label: goal.description.clone(),
            kind: "goal".into(),
            value: goal.priority as f32 / 10.0,
        });

        edges.push(MindspaceEdge {
            from: "curiosity".into(),
            to: format!("goal_{}", goal.id),
            label: "influences".into(),
            weight: state.curiosity_level,
        });

        edges.push(MindspaceEdge {
            from: "trait_openness".into(),
            to: format!("goal_{}", goal.id),
            label: "shapes".into(),
            weight: state.personality.traits.openness,
        });
    }

    MindspaceGraph { nodes, edges }
}
