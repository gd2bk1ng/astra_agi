// ============================================================================
//                         ASTRA AGI • COGNITIVE LOOP
//        Perception → Goal Formation → Planning → Action → Learning
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Implements Astra’s main cognitive cycle, orchestrating perception,
//       goal formation, planning, execution, memory writing, reflection, and
//       meta-learning. This loop is the heartbeat of Astra’s mind.
//
//   Core Functions:
//       • Integrate stimuli into the cognitive state
//       • Form and select goals, generate plans, and execute them
//       • Record episodes and thought traces for reflection and learning
//
//   File:        /src/cognition/cognitive_loop.rs
//   Author:      Alex Roussinov
//   Created:     2026-01-11
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use anyhow::Result;
use log::{info, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::cognition::{
    build_self_summary, generate_goals_from_stimulus, select_primary_goal,
    update_curiosity, CognitiveState, ThoughtTrace,
};
use crate::cognition::episodes::record_episode;
use crate::cognition::learning_adapter::LearningAdapter;

use crate::planning::executor::{ActionExecutor, PlanExecutor, ExecutionStatus};
use crate::planning::planner::{Planner, WorldState};
use crate::cognition::motivation::{evaluate_goal_motivation, update_energy_after_outcome};
use crate::cognition::goal_formation::Stimulus;

/// Represents an interface that can provide world state from the environment.
pub trait WorldStateProvider {
    fn current_world_state(&self) -> WorldState;
}

/// High-level cognitive loop driver.
pub struct CognitiveLoop<E: ActionExecutor, W: WorldStateProvider, L: LearningAdapter> {
    pub state: Arc<Mutex<CognitiveState>>,
    planner: Planner,
    env_executor: E,
    world_provider: W,
    learner: L,
}

impl<E, W, L> CognitiveLoop<E, W, L>
where
    E: ActionExecutor,
    W: WorldStateProvider,
    L: LearningAdapter,
{
    pub fn new(
        state: Arc<Mutex<CognitiveState>>,
        env_executor: E,
        world_provider: W,
        learner: L,
    ) -> Self {
        Self {
            state,
            planner: Planner::new(),
            env_executor,
            world_provider,
            learner,
        }
    }

    /// Runs a single cognitive cycle reacting to an input stimulus.
    pub async fn step(&mut self, stimulus: Stimulus) -> Result<()> {
        let mut state = self.state.lock().await;

        // 1. Update curiosity based on novelty (placeholder heuristic).
        let novelty_score = 0.7; // TODO: derive from learning/perception
        update_curiosity(&mut state, novelty_score);

        // 2. Goal formation.
        let candidate_goals = generate_goals_from_stimulus(&state, &stimulus);
        let primary = match select_primary_goal(&state, &candidate_goals) {
            Some(g) => g,
            None => {
                info!("No primary goal selected for stimulus '{}'", stimulus.content);
                return Ok(());
            }
        };

        let motivation_score = evaluate_goal_motivation(&state, &primary);
        info!(
            "Selected goal '{}' with motivation score {:.3}",
            primary.id, motivation_score
        );
        state.context.active_goal = Some(primary.clone());

        // 3. Planning.
        let world = self.world_provider.current_world_state();
        let available_actions = vec![]; // TODO: inject domain actions
        let plan = self
            .planner
            .plan_auto(&world, &primary, &available_actions)?;

        if plan.actions.is_empty() {
            warn!("Planner returned empty plan for goal {}", primary.id);
            return Ok(());
        }

        state.context.active_plan = Some(plan.clone());

        // 4. Thought trace.
        let mut trace = ThoughtTrace::new(&primary.id);
        trace.add_step(
            format!("Selected goal '{}' based on stimulus '{}'", primary.id, stimulus.content),
            0.9,
        );
        trace.add_step(
            format!("Generated plan with {} actions", plan.actions.len()),
            0.8,
        );

        // 5. Execution.
        let mut executor = PlanExecutor::new(plan.clone(), &mut self.env_executor);
        let status = executor.run_to_completion()?;
        let success = matches!(status, ExecutionStatus::Completed);

        update_energy_after_outcome(&mut state.energy, success);

        // 6. Self-summary (for logging / introspection).
        let summary = build_self_summary(&state);
        info!("Self-summary: {}", summary.explanation);

        // 7. Write episode + thought trace to Narrative Memory.
        record_episode(&state, &trace, success);

        // 8. Learning adapter hook.
        self.learner.update_from_episode(&state, &trace, success);

        Ok(())
    }
}
