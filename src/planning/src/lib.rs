//! Astra Planning Crate
//!
//! Handles planning and decision-making.

/// Represents a goal.
pub struct Goal {
    pub description: String,
}

/// Represents a plan.
pub struct Plan {
    pub steps: Vec<String>,
}

/// Creates a plan to achieve a goal.
pub fn create_plan(goal: Goal) -> Plan {
    Plan {
        steps: vec![format!("Plan to achieve: {}", goal.description)],
    }
}

/// Evaluates multiple plans and returns the best one.
pub fn evaluate_plans(plans: &[Plan]) -> Plan {
    plans.first().cloned().unwrap_or_else(|| Plan { steps: vec![] })
}

/// Executes a given plan.
pub fn execute_plan(plan: &Plan) {
    for step in &plan.steps {
        println!("Executing step: {}", step);
    }
}
