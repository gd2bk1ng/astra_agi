// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\query_executor.rs
//
//  Description: Executes QueryExpr against Ontology to retrieve matching entities.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-25
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use crate::knowledge::{Ontology, QueryExpr, LogicalOp, ComparisonOp, AttributeFilter, AttributeValue, Id};

impl Ontology {
    /// Evaluate a QueryExpr against the ontology, returning matching entities
    pub fn query(&self, expr: &QueryExpr) -> Vec<&crate::knowledge::Entity> {
        match expr {
            QueryExpr::Concept(concept_id) => {
                self.find_entities_by_concept(*concept_id)
            }
            QueryExpr::AttrFilter(filter) => {
                self.find_entities_by_attribute_filter(filter)
            }
            QueryExpr::Logical { op, exprs } => {
                let mut sets: Vec<Vec<&crate::knowledge::Entity>> = exprs.iter().map(|e| self.query(e)).collect();
                match op {
                    LogicalOp::And => {
                        // Intersection of all result sets
                        if sets.is_empty() {
                            vec![]
                        } else {
                            sets.iter().skip(1).fold(sets[0].clone(), |acc, s| {
                                acc.into_iter().filter(|e| s.contains(e)).collect()
                            })
                        }
                    }
                    LogicalOp::Or => {
                        // Union of all result sets
                        let mut union = Vec::new();
                        for s in sets {
                            for e in s {
                                if !union.contains(&e) {
                                    union.push(e);
                                }
                            }
                        }
                        union
                    }
                    LogicalOp::Not => {
                        // Not supported here - use QueryExpr::Not variant
                        vec![]
                    }
                }
            }
            QueryExpr::Not(sub_expr) => {
                let all_entities: Vec<&crate::knowledge::Entity> = self.entities.values().collect();
                let sub_results = self.query(sub_expr);
                all_entities.into_iter().filter(|e| !sub_results.contains(e)).collect()
            }
        }
    }

    /// Helper method to filter entities by attribute filter condition
    fn find_entities_by_attribute_filter(&self, filter: &AttributeFilter) -> Vec<&crate::knowledge::Entity> {
        self.entities.values().filter(|entity| {
            if let Some(val) = entity.attribute_values.get(&filter.attr_name) {
                Self::compare_attribute_values(val, &filter.op, &filter.value)
            } else {
                false
            }
        }).collect()
    }

    /// Compares two attribute values with a comparison operator
    fn compare_attribute_values(val: &AttributeValue, op: &ComparisonOp, cmp_val: &AttributeValue) -> bool {
        use AttributeValue::*;
        match (val, cmp_val) {
            (Integer(a), Integer(b)) => Self::compare_ord(*a, *b, op),
            (Float(a), Float(b)) => Self::compare_ord(*a, *b, op),
            (String(a), String(b)) => Self::compare_ord(a, b, op),
            (Boolean(a), Boolean(b)) => Self::compare_ord(a, b, op),
            // For Reference and mismatched types, only equality check
            (Reference(a), Reference(b)) => match op {
                ComparisonOp::Eq => a == b,
                ComparisonOp::Neq => a != b,
                _ => false,
            },
            _ => false,
        }
    }

    /// Helper generic comparison for types implementing Ord and PartialEq
    fn compare_ord<T: PartialOrd + PartialEq>(a: T, b: T, op: &ComparisonOp) -> bool {
        match op {
            ComparisonOp::Eq => a == b,
            ComparisonOp::Neq => a != b,
            ComparisonOp::Gt => a > b,
            ComparisonOp::Lt => a < b,
            ComparisonOp::Gte => a >= b,
            ComparisonOp::Lte => a <= b,
        }
    }
}
