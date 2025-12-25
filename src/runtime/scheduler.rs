// =============================================================================
//  Astra Executor Runtime (AER)
//  File: scheduler.rs
//
//  Description:
//  Task scheduler for Astra runtime, managing concurrency, time-awareness,
//  and priority-based task execution.
//  Supports preemptive and cooperative multitasking models,
//  scheduling tasks with delays and priorities,
//  enabling time-sliced execution for AGI workloads.
//
//  Designed for extensibility to support async tasks, dependencies,
//  and cancellation in future AGI runtime versions.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-22
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use std::collections::{BinaryHeap, HashMap};
use std::time::{Duration, Instant};
use std::cmp::Ordering;

/// Represents a scheduled task with priority and scheduled execution time.
pub struct ScheduledTask {
    pub id: usize,
    pub priority: u32,
    pub scheduled_time: Instant,
    pub task: Box<dyn FnMut() + Send>,
}

impl Eq for ScheduledTask {}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Max-heap by priority, then earliest scheduled_time
        other.priority.cmp(&self.priority)
            .then_with(|| self.scheduled_time.cmp(&other.scheduled_time))
    }
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Scheduler manages tasks, executing them according to priority and timing.
pub struct Scheduler {
    task_queue: BinaryHeap<ScheduledTask>,
    active_tasks: HashMap<usize, ScheduledTask>,
    next_task_id: usize,
}

impl Scheduler {
    /// Creates a new Scheduler instance.
    pub fn new() -> Self {
        Scheduler {
            task_queue: BinaryHeap::new(),
            active_tasks: HashMap::new(),
            next_task_id: 0,
        }
    }

    /// Initializes or resets the scheduler state.
    pub fn start(&mut self) {
        self.task_queue.clear();
        self.active_tasks.clear();
        self.next_task_id = 0;
    }

    /// Schedule a new task with priority and delay.
    ///
    /// # Arguments
    /// * `priority` - Task priority (higher runs first).
    /// * `delay` - Delay before execution.
    /// * `task` - Closure representing the task.
    ///
    /// Returns the assigned task ID.
    pub fn schedule_task<F>(&mut self, priority: u32, delay: Duration, task: F) -> usize
    where
        F: FnMut() + Send + 'static,
    {
        let id = self.next_task_id;
        self.next_task_id += 1;

        let scheduled_time = Instant::now() + delay;
        let scheduled_task = ScheduledTask {
            id,
            priority,
            scheduled_time,
            task: Box::new(task),
        };
        self.task_queue.push(scheduled_task);
        id
    }

    /// Advances the scheduler by executing all tasks scheduled up to now.
    pub fn tick(&mut self) {
        let now = Instant::now();

        while let Some(mut scheduled_task) = self.task_queue.peek_mut() {
            if scheduled_task.scheduled_time <= now {
                // Pop and run the task
                let mut task = self.task_queue.pop().unwrap();
                (task.task)();
                self.active_tasks.remove(&task.id);
            } else {
                break;
            }
        }
    }
}
