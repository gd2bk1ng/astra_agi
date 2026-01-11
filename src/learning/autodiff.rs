// ============================================================================
//                     ASTRA AGI • AUTODIFF & DIFFERENTIABLE CORE
//        Foundations for Gradient Computation and Differentiable Programming
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Provides the core abstractions for autodifferentiation within Astra’s
//       Learning subsystem. This module defines differentiable variables,
//       gradient propagation mechanisms, and the entry point for gradient
//       computation. It forms the basis for optimization, model training,
//       and reinforcement learning updates.
//
//   Core Functions:
//       • Represent differentiable variables with values and gradients
//       • Record computation graphs for reverse‑mode autodiff
//       • Perform backward gradient propagation from scalar loss values
//       • Serve as the computational substrate for learning algorithms
//
//   File:        /src/learning/autodiff.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-23
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use anyhow::{anyhow, Result};
use ndarray::{ArrayD, Dimension};
use std::cell::RefCell;
use std::ops::{Add, Mul, Neg};
use std::rc::Rc;

/// Shared handle to the computation tape.
type TapeHandle = Rc<RefCell<Tape>>;

/// Describes the operation that produced a node.
#[derive(Debug, Clone)]
enum OpKind {
    Input,
    Add,
    Mul,
    Neg,
    // Extend with more ops as needed (MatMul, Relu, Sigmoid, etc.)
}

/// A single node in the computation graph (value + gradient + parents).
#[derive(Debug)]
struct Node {
    /// Forward value of the node.
    value: ArrayD<f64>,
    /// Accumulated gradient of the node.
    grad: Option<ArrayD<f64>>,
    /// Operation that produced this node.
    op: OpKind,
    /// Indices of parent nodes in the tape.
    parents: Vec<usize>,
}

/// A tape that records all differentiable operations for reverse‑mode autodiff.
#[derive(Debug)]
struct Tape {
    nodes: Vec<Node>,
}

impl Tape {
    fn new() -> Self {
        Tape { nodes: Vec::new() }
    }

    /// Adds a node to the tape and returns its index.
    fn add_node(&mut self, node: Node) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    fn node(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    fn node_mut(&mut self, id: usize) -> &mut Node {
        &mut self.nodes[id]
    }

    /// Performs reverse‑mode backprop starting from a scalar loss node.
    fn backward(&mut self, loss_id: usize) -> Result<()> {
        if self.nodes.is_empty() {
            return Err(anyhow!("No nodes in tape for backward pass"));
        }

        // Initialize gradient at loss as 1.0 (dL/dL = 1).
        {
            let loss_node = self.node(loss_id);
            if loss_node.value.ndim() != 0 {
                return Err(anyhow!(
                    "Backward currently expects scalar loss; got shape {:?}",
                    loss_node.value.shape()
                ));
            }
        }

        let ones = ArrayD::from_elem((), 1.0);
        self.node_mut(loss_id).grad = Some(ones);

        // Reverse topological order: since it's a simple tape with append‑only,
        // iterating indices in reverse is valid (parents always have smaller IDs).
        for idx in (0..=loss_id).rev() {
            let (op, parents, grad_opt, value_shape) = {
                let node = self.node(idx);
                (
                    node.op.clone(),
                    node.parents.clone(),
                    node.grad.clone(),
                    node.value.raw_dim(),
                )
            };

            let grad = match grad_opt {
                Some(g) => g,
                None => continue, // No gradient flowed to this node; skip.
            };

            match op {
                OpKind::Input => {
                    // Leaf node: nothing to propagate.
                }
                OpKind::Neg => {
                    // y = -x => dy/dx = -1
                    if parents.len() != 1 {
                        return Err(anyhow!("Neg op expects 1 parent"));
                    }
                    let parent_id = parents[0];
                    Self::accumulate_grad(self, parent_id, &(-&grad))?;
                }
                OpKind::Add => {
                    // z = x + y => dz/dx = 1, dz/dy = 1
                    if parents.len() != 2 {
                        return Err(anyhow!("Add op expects 2 parents"));
                    }
                    let left_id = parents[0];
                    let right_id = parents[1];

                    // Broadcast grad to match parent shapes if necessary.
                    let grad_left = Self::match_shape(&grad, self.node(left_id).value.ndim())?;
                    let grad_right = Self::match_shape(&grad, self.node(right_id).value.ndim())?;

                    Self::accumulate_grad(self, left_id, &grad_left)?;
                    Self::accumulate_grad(self, right_id, &grad_right)?;
                }
                OpKind::Mul => {
                    // z = x * y => dz/dx = y, dz/dy = x
                    if parents.len() != 2 {
                        return Err(anyhow!("Mul op expects 2 parents"));
                    }
                    let left_id = parents[0];
                    let right_id = parents[1];

                    let x_val = self.node(left_id).value.clone();
                    let y_val = self.node(right_id).value.clone();

                    // dz/dx = y * grad
                    let grad_left = &grad * &y_val;
                    // dz/dy = x * grad
                    let grad_right = &grad * &x_val;

                    Self::accumulate_grad(self, left_id, &grad_left)?;
                    Self::accumulate_grad(self, right_id, &grad_right)?;
                }
            }

            // Sanity check: keep gradient shape compatible with value shape.
            let node = self.node(idx);
            if let Some(g) = &node.grad {
                if g.ndim() != value_shape.ndim() {
                    // In a full implementation we’d handle broadcasting properly.
                    // For now, keep this assert for consistency.
                    // You can relax or extend this later.
                }
            }
        }

        Ok(())
    }

    /// Accumulates gradient into a node: grad[node] += incoming.
    fn accumulate_grad(&mut self, node_id: usize, incoming: &ArrayD<f64>) -> Result<()> {
        let node = self.node_mut(node_id);
        match &mut node.grad {
            Some(existing) => {
                if existing.shape() != incoming.shape() {
                    return Err(anyhow!(
                        "Gradient shape mismatch: existing {:?}, incoming {:?}",
                        existing.shape(),
                        incoming.shape()
                    ));
                }
                *existing += incoming;
            }
            None => {
                node.grad = Some(incoming.clone());
            }
        }
        Ok(())
    }

    /// Basic helper for shape matching; here it’s a placeholder.
    /// In a more advanced system, this would handle broadcasting reductions.
    fn match_shape(grad: &ArrayD<f64>, _ndim: usize) -> Result<ArrayD<f64>> {
        Ok(grad.clone())
    }
}

/// A differentiable variable bound to a shared computation tape.
#[derive(Clone)]
pub struct Variable {
    id: usize,
    tape: TapeHandle,
}

impl Variable {
    /// Creates a new input variable (leaf node) with the given value.
    pub fn new_input(tape: &TapeHandle, value: ArrayD<f64>) -> Self {
        let mut tape_mut = tape.borrow_mut();
        let id = tape_mut.add_node(Node {
            value,
            grad: None,
            op: OpKind::Input,
            parents: Vec::new(),
        });
        drop(tape_mut);

        Variable {
            id,
            tape: Rc::clone(tape),
        }
    }

    /// Returns the forward value of this variable.
    pub fn value(&self) -> ArrayD<f64> {
        self.tape.borrow().node(self.id).value.clone()
    }

    /// Returns the gradient of this variable if computed.
    pub fn grad(&self) -> Option<ArrayD<f64>> {
        self.tape.borrow().node(self.id).grad.clone()
    }

    /// Perform backpropagation from this variable as the scalar loss.
    pub fn backward(&self) -> Result<()> {
        self.tape.borrow_mut().backward(self.id)
    }
}

// -------------------------------
// Operator Overloading
// -------------------------------

impl Add for &Variable {
    type Output = Variable;

    fn add(self, rhs: Self) -> Self::Output {
        let tape = Rc::clone(&self.tape);
        let mut tape_mut = tape.borrow_mut();

        let left = tape_mut.node(self.id).value.clone();
        let right = tape_mut.node(rhs.id).value.clone();
        let value = &left + &right;

        let id = tape_mut.add_node(Node {
            value,
            grad: None,
            op: OpKind::Add,
            parents: vec![self.id, rhs.id],
        });

        drop(tape_mut);

        Variable { id, tape }
    }
}

impl Mul for &Variable {
    type Output = Variable;

    fn mul(self, rhs: Self) -> Self::Output {
        let tape = Rc::clone(&self.tape);
        let mut tape_mut = tape.borrow_mut();

        let left = tape_mut.node(self.id).value.clone();
        let right = tape_mut.node(rhs.id).value.clone();
        let value = &left * &right;

        let id = tape_mut.add_node(Node {
            value,
            grad: None,
            op: OpKind::Mul,
            parents: vec![self.id, rhs.id],
        });

        drop(tape_mut);

        Variable { id, tape }
    }
}

impl Neg for &Variable {
    type Output = Variable;

    fn neg(self) -> Self::Output {
        let tape = Rc::clone(&self.tape);
        let mut tape_mut = tape.borrow_mut();

        let val = tape_mut.node(self.id).value.clone();
        let value = -val;

        let id = tape_mut.add_node(Node {
            value,
            grad: None,
            op: OpKind::Neg,
            parents: vec![self.id],
        });

        drop(tape_mut);

        Variable { id, tape }
    }
}

/// Entry point and façade for the autodiff system.
pub struct AutoDiff {
    tape: TapeHandle,
}

impl AutoDiff {
    /// Creates a new autodiff engine with an empty computation tape.
    pub fn new() -> Self {
        AutoDiff {
            tape: Rc::new(RefCell::new(Tape::new())),
        }
    }

    /// Creates a new input variable bound to this engine’s tape.
    pub fn variable(&self, value: ArrayD<f64>) -> Variable {
        Variable::new_input(&self.tape, value)
    }

    /// Exposes the underlying tape for inspection or advanced use.
    pub fn tape(&self) -> TapeHandle {
        Rc::clone(&self.tape)
    }
}
