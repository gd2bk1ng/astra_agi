# Astra Planning Crate

## Overview

Implements planning algorithms, goal management, and decision-making.

## Main Modules & Functions

- `create_plan(goal: Goal) -> Plan`  
  Generate action plans.

- `evaluate_plans(plans: &[Plan]) -> Plan`  
  Select best plan.

- `execute_plan(plan: &Plan)`  
  Execute plan actions.

## Usage

Used by runtime to translate intentions into actions.

## Documentation

```bash
cargo doc --open
```

License
MIT OR Apache 2.0
