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

use crate::cognition::{
    build_self_summary, generate_goals_from_stimulus, select_primary_goal,
    update_curiosity, CognitiveState, ThoughtTrace,
};
use crate::planning::executor::{ActionExecutor, PlanExecutor};
use crate::planning::planner::{Planner, WorldState};
use crate::cognition::motivation::{evaluate_goal_motivation, update_energy_after_outcome};
use crate::cognition::goal_formation::Stimulus;

/// Represents an interface that can provide world state from the environment.
pub trait WorldStateProvider {
    fn current_world_state(&self) -> WorldState;
}

/// High-level cognitive loop driver.
pub struct CognitiveLoop<E: ActionExecutor, W: WorldStateProvider> {
    pub state: CognitiveState,
    planner: Planner,
    env_executor: E,
    world_provider: W,
}

impl<E: ActionExecutor, W: WorldStateProvider> CognitiveLoop<E, W> {
    pub fn new(state: CognitiveState, env_executor: E, world_provider: W) -> Self {
        Self {
            state,
            planner: Planner::new(),
            env_executor,
            world_provider,
        }
    }

    /// Runs a single cognitive cycle reacting to an input stimulus.
    pub fn step(&mut self, stimulus: Stimulus) -> Result<()> {
        // 1. Update curiosity based on novelty (placeholder heuristic).
        let novelty_score = 0.7; // TODO: derive from learning/perception
        update_curiosity(&mut self.state, novelty_score);

        // 2. Goal formation.
        let candidate_goals = generate_goals_from_stimulus(&self.state, &stimulus);
        let primary = match select_primary_goal(&self.state, &candidate_goals) {
            Some(g) => g,
            None => {
                info!("No primary goal selected for stimulus '{}'", stimulus.content);
                return Ok(());
            }
        };

        let motivation_score = evaluate_goal_motivation(&self.state, &primary);
        info!(
            "Selected goal '{}' with motivation score {:.3}",
            primary.id, motivation_score
        );
        self.state.context.active_goal = Some(primary.clone());

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

        self.state.context.active_plan = Some(plan.clone());

        // 4. Thought trace (simple example).
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
        let success = matches!(status, crate::planning::executor::ExecutionStatus::Completed);

        update_energy_after_outcome(&mut self.state.energy, success);

        // 6. Self-summary (for logging / introspection).
        let summary = build_self_summary(&self.state);
        info!("Self-summary: {}", summary.explanation);

        // TODO: 7. Write episode + thought trace to Narrative Memory & Learning.
        // TODO: integrate with reflection loop and meta-learning.

        Ok(())
    }
}
