// =============================================================================
//  Astra Executor Runtime (AER)
//  File: mod.rs
//
//  Description:
//  This module serves as the root of the runtime subsystem for Astra language programs.
//  It exposes the core runtime components: the Executor and Scheduler.
//  Provides a high-level Runtime struct to coordinate program execution, task scheduling,
//  and time-aware intent management.
//
//  This design supports extensibility, observability, and integration with other AGI modules.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-22
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

pub mod executor;
pub mod scheduler;
pub mod intent_manager;

use executor::Executor;
use scheduler::Scheduler;

/// Orchestrates the execution of Astra programs.
/// Manages executor and scheduler lifecycles and coordinates ticks.
pub struct Runtime {
    executor: Executor,
    scheduler: Scheduler,
}

impl Runtime {
    /// Creates a new Runtime instance.
    pub fn new() -> Self {
        Runtime {
            executor: Executor::new(),
            scheduler: Scheduler::new(),
        }
    }

    /// Starts the runtime components.
    pub fn start(&mut self) {
        self.scheduler.start();
        self.executor.start();
    }

    /// Parses and executes Astra source code.
    pub fn execute_program(&mut self, program: &str) {
        let ast = self.executor.parse(program).expect("Parsing failed");
        self.executor.execute(&ast);
    }

    /// Advances runtime by one tick.
    pub fn tick(&mut self) {
        self.scheduler.tick();
        self.executor.tick();
    }
}
