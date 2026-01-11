// ============================================================================
//                          ASTRA AGI • PLAN EXECUTION CORE
//        Action Realization, Stepwise Progress & Execution Monitoring
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Implements Astra’s plan execution layer, responsible for taking
//       high-level plans produced by the Planner and realizing them as
//       concrete actions. This module provides stepwise execution, status
//       tracking, and integration points for environment adapters or actuators.
//       It is deliberately abstract, allowing different backends (simulation,
//       real-world, or API-driven) to plug in.
//
//   Core Functions:
//       • Represent execution state and progress
//       • Step through plans one action at a time
//       • Report success, failure, and partial completion
//       • Provide hooks for environment-specific action handlers
//
//   File:        /src/planning/executor.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::planner::{Action, Plan};
use anyhow::{anyhow, Result};
use log::{debug, info};

/// Represents the status of plan execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed(String),
}

/// Trait that environment adapters must implement in order to execute actions.
pub trait ActionExecutor {
    /// Executes a single action. Returns Ok(true) if successful, Ok(false) if
    /// the action failed in a recoverable way, and Err for critical errors.
    fn execute_action(&mut self, action: &Action) -> Result<bool>;
}

/// Simple in-memory executor that steps through a plan using an ActionExecutor.
pub struct PlanExecutor<E: ActionExecutor> {
    plan: Plan,
    index: usize,
    status: ExecutionStatus,
    env: E,
}

impl<E: ActionExecutor> PlanExecutor<E> {
    /// Creates a new plan executor.
    pub fn new(plan: Plan, env: E) -> Self {
        Self {
            plan,
            index: 0,
            status: ExecutionStatus::NotStarted,
            env,
        }
    }

    /// Returns the current execution status.
    pub fn status(&self) -> &ExecutionStatus {
        &self.status
    }

    /// Returns the underlying plan.
    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    /// Advances execution by one action step.
    pub fn step(&mut self) -> Result<()> {
        match self.status {
            ExecutionStatus::Completed | ExecutionStatus::Failed(_) => {
                return Err(anyhow!("Execution already finished"));
            }
            ExecutionStatus::NotStarted => {
                self.status = ExecutionStatus::InProgress;
            }
            ExecutionStatus::InProgress => {}
        }

        if self.index >= self.plan.actions.len() {
            self.status = ExecutionStatus::Completed;
            info!(
                "Plan {} completed successfully with {} actions",
                self.plan.goal_id,
                self.plan.actions.len()
            );
            return Ok(());
        }

        let action = &self.plan.actions[self.index];
        debug!("Executing action {} ({})", action.id, action.description);

        match self.env.execute_action(action) {
            Ok(true) => {
                self.index += 1;
                if self.index >= self.plan.actions.len() {
                    self.status = ExecutionStatus::Completed;
                    info!("Plan {} completed", self.plan.goal_id);
                }
                Ok(())
            }
            Ok(false) => {
                self.status = ExecutionStatus::Failed(format!(
                    "Action {} failed (recoverable) during plan {}",
                    action.id, self.plan.goal_id
                ));
                Ok(())
            }
            Err(e) => {
                self.status = ExecutionStatus::Failed(format!(
                    "Critical error executing action {}: {}",
                    action.id, e
                ));
                Err(e)
            }
        }
    }

    /// Executes the entire plan until completion or failure.
    pub fn run_to_completion(&mut self) -> Result<ExecutionStatus> {
        while self.status == ExecutionStatus::NotStarted
            || self.status == ExecutionStatus::InProgress
        {
            self.step()?;
        }
        Ok(self.status.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planner::{Action, Plan};
    use std::collections::HashMap;

    struct TestEnv {
        fail_on: Option<String>,
    }

    impl ActionExecutor for TestEnv {
        fn execute_action(&mut self, action: &Action) -> Result<bool> {
            if let Some(ref id) = self.fail_on {
                if &action.id == id {
                    return Ok(false);
                }
            }
            Ok(true)
        }
    }

    fn sample_plan() -> Plan {
        Plan {
            goal_id: "test_goal".into(),
            actions: vec![
                Action {
                    id: "a1".into(),
                    description: "First".into(),
                    preconditions: HashMap::new(),
                    effects: HashMap::new(),
                    cost: 1.0,
                },
                Action {
                    id: "a2".into(),
                    description: "Second".into(),
                    preconditions: HashMap::new(),
                    effects: HashMap::new(),
                    cost: 1.0,
                },
            ],
            estimated_cost: 2.0,
        }
    }

    #[test]
    fn executor_completes_successful_plan() {
        let plan = sample_plan();
        let env = TestEnv { fail_on: None };

        let mut executor = PlanExecutor::new(plan, env);
        let status = executor.run_to_completion().expect("execution failed");

        assert_eq!(status, ExecutionStatus::Completed);
    }

    #[test]
    fn executor_handles_recoverable_failure() {
        let plan = sample_plan();
        let env = TestEnv {
            fail_on: Some("a2".into()),
        };

        let mut executor = PlanExecutor::new(plan, env);
        let status = executor.run_to_completion().expect("execution failed");

        assert!(matches!(status, ExecutionStatus::Failed(_)));
    }
}
