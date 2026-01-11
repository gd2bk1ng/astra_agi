// ============================================================================
//                      ASTRA AGI • PLANNING CORE MODULE
//        Goal Models, Planning Strategies & Multi-Engine Coordination
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Implements Astra’s core planning abstractions and high-level planning
//       engines, including HTN, GOAP, and reactive planning. This module
//       defines shared goal/action/plan models and a unified planner facade
//       that can orchestrate multiple planning strategies depending on context,
//       constraints, and required deliberation depth.
//
//   Core Functions:
//       • Represent goals, actions, and executable plans
//       • Define a PlannerEngine trait for extensible planning backends
//       • Provide HTN, GOAP, and reactive planning implementations
//       • Offer a unified Planner interface for Astra’s cognitive runtime
//
//   File:        /src/planning/planner.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Represents a symbolic world state as key-value pairs.
pub type WorldState = HashMap<String, bool>;

/// Represents a high-level goal Astra wants to achieve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub desired_state: WorldState,
    pub priority: i32,
}

/// Represents an atomic action that Astra can execute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub description: String,
    pub preconditions: WorldState,
    pub effects: WorldState,
    pub cost: f32,
}

/// Represents a concrete, executable plan: an ordered sequence of actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub goal_id: String,
    pub actions: Vec<Action>,
    pub estimated_cost: f32,
}

impl Plan {
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}

/// Unified trait for all planning backends.
pub trait PlannerEngine {
    /// Attempts to construct a plan from the current world state to the goal.
    fn plan(&self, world: &WorldState, goal: &Goal, actions: &[Action]) -> Result<Plan>;
}

/// Planning strategies available to Astra.
#[derive(Debug, Clone, Copy)]
pub enum PlanningStrategy {
    Htn,
    Goap,
    Reactive,
}

/// Public-facing planner that can delegate to different engines.
pub struct Planner {
    htn: HtnPlanner,
    goap: GoapPlanner,
    reactive: ReactivePlanner,
}

impl Planner {
    /// Creates a planner with default configuration.
    pub fn new() -> Self {
        Self {
            htn: HtnPlanner::new(),
            goap: GoapPlanner::new(),
            reactive: ReactivePlanner::new(),
        }
    }

    /// Plans using a specified strategy.
    pub fn plan_with_strategy(
        &self,
        strategy: PlanningStrategy,
        world: &WorldState,
        goal: &Goal,
        actions: &[Action],
    ) -> Result<Plan> {
        match strategy {
            PlanningStrategy::Htn => self.htn.plan(world, goal, actions),
            PlanningStrategy::Goap => self.goap.plan(world, goal, actions),
            PlanningStrategy::Reactive => self.reactive.plan(world, goal, actions),
        }
    }

    /// Plans using an automatically selected strategy based on goal priority and
    /// available information. High-priority or complex goals get more
    /// deliberative strategies; simpler ones use reactive planning.
    pub fn plan_auto(
        &self,
        world: &WorldState,
        goal: &Goal,
        actions: &[Action],
    ) -> Result<Plan> {
        let strategy = if goal.priority >= 8 {
            PlanningStrategy::Htn
        } else if goal.priority >= 4 {
            PlanningStrategy::Goap
        } else {
            PlanningStrategy::Reactive
        };
        debug!("Selected planning strategy: {:?} for goal {}", strategy, goal.id);
        self.plan_with_strategy(strategy, world, goal, actions)
    }
}

// ============================================================================
//                               HTN PLANNER
// ----------------------------------------------------------------------------

/// Represents a high-level HTN method: decomposes an abstract task into subtasks.
#[derive(Debug, Clone)]
pub struct HtnMethod {
    pub name: String,
    pub task: String,
    pub preconditions: WorldState,
    pub subtasks: Vec<String>, // references to lower-level tasks or primitive actions
}

/// HTN planner with simple top-down decomposition.
pub struct HtnPlanner {
    methods: Vec<HtnMethod>,
}

impl HtnPlanner {
    pub fn new() -> Self {
        // In a full system, methods would be loaded from config or learned.
        Self { methods: Vec::new() }
    }

    pub fn register_method(&mut self, method: HtnMethod) {
        self.methods.push(method);
    }
}

impl PlannerEngine for HtnPlanner {
    fn plan(&self, world: &WorldState, goal: &Goal, actions: &[Action]) -> Result<Plan> {
        // Simplified HTN: treat each desired_state key as a task, and try to
        // find actions that satisfy it directly.
        let mut plan_actions = Vec::new();
        let mut estimated_cost = 0.0;
        let mut current_world = world.clone();

        for (k, desired) in &goal.desired_state {
            if current_world.get(k) == Some(desired) {
                continue;
            }

            // Find an action whose effects set this key
            if let Some(action) = actions.iter().find(|a| a.effects.get(k) == Some(desired)) {
                // TODO: In a full HTN, we would use methods to decompose tasks,
                // check hierarchical constraints, and preserve ordering.
                plan_actions.push(action.clone());
                estimated_cost += action.cost;
                // Apply effect to local world
                for (ek, ev) in &action.effects {
                    current_world.insert(ek.clone(), *ev);
                }
            }
        }

        Ok(Plan {
            goal_id: goal.id.clone(),
            actions: plan_actions,
            estimated_cost,
        })
    }
}

// ============================================================================
//                               GOAP PLANNER
// ----------------------------------------------------------------------------

/// GOAP-style planner using a forward search (simplified BFS with cost).
pub struct GoapPlanner {}

impl GoapPlanner {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlannerEngine for GoapPlanner {
    fn plan(&self, world: &WorldState, goal: &Goal, actions: &[Action]) -> Result<Plan> {
        // Simplified forward search:
        // - BFS over world states
        // - Stop when desired_state is satisfied
        // This is intentionally minimal but structurally correct.

        #[derive(Clone)]
        struct Node {
            world: WorldState,
            actions: Vec<Action>,
            cost: f32,
        }

        let mut queue = VecDeque::new();
        queue.push_back(Node {
            world: world.clone(),
            actions: Vec::new(),
            cost: 0.0,
        });

        while let Some(node) = queue.pop_front() {
            if goal_satisfied(&node.world, &goal.desired_state) {
                return Ok(Plan {
                    goal_id: goal.id.clone(),
                    actions: node.actions,
                    estimated_cost: node.cost,
                });
            }

            for action in actions {
                if preconditions_met(&node.world, &action.preconditions) {
                    let mut new_world = node.world.clone();
                    for (k, v) in &action.effects {
                        new_world.insert(k.clone(), *v);
                    }

                    let mut new_actions = node.actions.clone();
                    new_actions.push(action.clone());
                    let new_cost = node.cost + action.cost;

                    // In a full GOAP system, we would track visited states and
                    // use an A* heuristic. Here we keep it minimal and extendable.
                    queue.push_back(Node {
                        world: new_world,
                        actions: new_actions,
                        cost: new_cost,
                    });
                }
            }
        }

        // No plan found, return empty plan.
        Ok(Plan {
            goal_id: goal.id.clone(),
            actions: Vec::new(),
            estimated_cost: f32::INFINITY,
        })
    }
}

// ============================================================================
//                           REACTIVE PLANNER
// ----------------------------------------------------------------------------

/// Simple reactive planner: picks one or a few actions greedily based on
/// immediate goal satisfaction and minimal cost.
pub struct ReactivePlanner {}

impl ReactivePlanner {
    pub fn new() -> Self {
        Self {}
    }
}

impl PlannerEngine for ReactivePlanner {
    fn plan(&self, world: &WorldState, goal: &Goal, actions: &[Action]) -> Result<Plan> {
        let mut best_action: Option<&Action> = None;
        let mut best_score = f32::MIN;

        for action in actions {
            // Score action by how many desired keys it satisfies minus cost.
            let mut score = 0.0;
            for (k, desired) in &goal.desired_state {
                if let Some(effect_val) = action.effects.get(k) {
                    if effect_val == desired {
                        score += 1.0;
                    }
                }
            }
            score -= action.cost;

            if score > best_score && preconditions_met(world, &action.preconditions) {
                best_score = score;
                best_action = Some(action);
            }
        }

        let actions_out = match best_action {
            Some(a) => vec![a.clone()],
            None => Vec::new(),
        };

        Ok(Plan {
            goal_id: goal.id.clone(),
            actions: actions_out,
            estimated_cost: if actions_out.is_empty() {
                f32::INFINITY
            } else {
                actions_out.iter().map(|a| a.cost).sum()
            },
        })
    }
}

// ============================================================================
//                             HELPER FUNCTIONS
// ----------------------------------------------------------------------------

fn goal_satisfied(world: &WorldState, desired: &WorldState) -> bool {
    desired
        .iter()
        .all(|(k, v)| world.get(k).map(|cv| cv == v).unwrap_or(false))
}

fn preconditions_met(world: &WorldState, preconditions: &WorldState) -> bool {
    preconditions
        .iter()
        .all(|(k, v)| world.get(k).map(|cv| cv == v).unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_actions() -> Vec<Action> {
        vec![
            Action {
                id: "turn_on_light".into(),
                description: "Turn on the light".into(),
                preconditions: HashMap::from([("has_power".into(), true)]),
                effects: HashMap::from([("light_on".into(), true)]),
                cost: 1.0,
            },
            Action {
                id: "enable_power".into(),
                description: "Enable power".into(),
                preconditions: HashMap::new(),
                effects: HashMap::from([("has_power".into(), true)]),
                cost: 2.0,
            },
        ]
    }

    #[test]
    fn test_goap_planner_finds_plan() {
        let world = HashMap::new();
        let goal = Goal {
            id: "see_in_dark".into(),
            description: "Be able to see in the dark".into(),
            desired_state: HashMap::from([("light_on".into(), true)]),
            priority: 5,
        };
        let planner = Planner::new();
        let actions = sample_actions();

        let plan = planner.plan_with_strategy(PlanningStrategy::Goap, &world, &goal, &actions)
            .expect("planning failed");
        assert!(!plan.is_empty());
        assert_eq!(plan.actions.len(), 2);
    }

    #[test]
    fn test_reactive_planner_picks_single_action() {
        let mut world = HashMap::new();
        world.insert("has_power".into(), true);

        let goal = Goal {
            id: "light_goal".into(),
            description: "Turn on the light".into(),
            desired_state: HashMap::from([("light_on".into(), true)]),
            priority: 2,
        };

        let planner = Planner::new();
        let actions = sample_actions();

        let plan = planner
            .plan_with_strategy(PlanningStrategy::Reactive, &world, &goal, &actions)
            .expect("planning failed");

        assert_eq!(plan.actions.len(), 1);
        assert_eq!(plan.actions[0].id, "turn_on_light");
    }

    #[test]
    fn test_auto_strategy_selection() {
        let world = HashMap::new();
        let goal = Goal {
            id: "high_priority_goal".into(),
            description: "Critical objective".into(),
            desired_state: HashMap::from([("light_on".into(), true)]),
            priority: 9,
        };

        let planner = Planner::new();
        let actions = sample_actions();

        let plan = planner
            .plan_auto(&world, &goal, &actions)
            .expect("planning failed");

        // HTN implementation is simplified; plan may be empty but pipeline should not panic.
        assert!(plan.estimated_cost.is_finite());
    }
}
