// ============================================================================
//                     ASTRA AGI • ONTOLOGY GRAPH UTILITIES
//        Traversal, Connectivity & Pathfinding for Knowledge Relationships
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Component of Astra’s Knowledge Layer, providing graph‑based algorithms
//       for navigating ontology structures. This module enables efficient
//       traversal, neighborhood exploration, and shortest‑path discovery across
//       entity relationships, supporting reasoning, querying, and structural
//       analysis of the knowledge graph.
//
//   Core Functions:
//       • Breadth‑first search (BFS) traversal with depth limiting
//       • Shortest‑path computation between ontology entities
//       • Utility algorithms for exploring adjacency relationships
//       • Foundational graph operations used by higher‑level reasoning modules
//
//   File:        /src/knowledge/graph_utils.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-26
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::knowledge::{Ontology, Id};
use std::collections::{VecDeque, HashSet};

impl<S: crate::knowledge::storage::Storage> Ontology<S> {
    /// Breadth-first search traversal from a start entity ID
    pub fn bfs(&self, start_id: Id, max_depth: usize) -> Vec<Id> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back((start_id, 0));
        visited.insert(start_id);

        while let Some((current, depth)) = queue.pop_front() {
            result.push(current);
            if depth >= max_depth {
                continue;
            }
            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back((neighbor, depth + 1));
                    }
                }
            }
        }
        result
    }

    /// Find shortest path between two entities using BFS
    pub fn shortest_path(&self, start_id: Id, end_id: Id) -> Option<Vec<Id>> {
        use std::collections::HashMap;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut predecessors: HashMap<Id, Id> = HashMap::new();

        queue.push_back(start_id);
        visited.insert(start_id);

        while let Some(current) = queue.pop_front() {
            if current == end_id {
                // Reconstruct path
                let mut path = vec![end_id];
                let mut node = end_id;
                while let Some(&pred) = predecessors.get(&node) {
                    path.push(pred);
                    node = pred;
                }
                path.reverse();
                return Some(path);
            }
            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        predecessors.insert(neighbor, current);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        None
    }
}
