// ============================================================================
//                        ASTRA AGI • QUERY DSL MODULE
//        Expressive Logical Query Language for Ontology Exploration
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Component of Astra’s Knowledge Layer, providing a domain‑specific
//       language (DSL) for constructing expressive, composable logical queries
//       over the ontology. This module enables structured filtering, concept
//       matching, attribute comparisons, and logical composition, forming the
//       foundation for high‑level semantic search and reasoning.
//
//   Core Functions:
//       • Define logical, comparison, and attribute‑based query primitives
//       • Support nested AND/OR/NOT expressions for complex filtering
//       • Enable concept‑based and attribute‑based entity selection
//       • Serve as the query representation consumed by the Query Executor
//
//   File:        /src/knowledge/query.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-25
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use crate::knowledge::{AttributeValue, Id};

/// Logical operators for composing queries
#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
    Not,
}

/// Comparison operators for attribute filters
#[derive(Debug, Clone)]
pub enum ComparisonOp {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
}

/// Represents a basic attribute filter condition
#[derive(Debug, Clone)]
pub struct AttributeFilter {
    pub attr_name: String,
    pub op: ComparisonOp,
    pub value: AttributeValue,
}

/// Represents a query expression node
#[derive(Debug, Clone)]
pub enum QueryExpr {
    /// Match entities having a specific concept (by ID)
    Concept(Id),

    /// Filter entities by attribute condition
    AttrFilter(AttributeFilter),

    /// Logical combination of sub-expressions
    Logical {
        op: LogicalOp,
        exprs: Vec<QueryExpr>,
    },

    /// Negation of a sub-expression
    Not(Box<QueryExpr>),
}

impl QueryExpr {
    /// Convenience constructor for AND of multiple expressions
    pub fn and(exprs: Vec<QueryExpr>) -> QueryExpr {
        QueryExpr::Logical { op: LogicalOp::And, exprs }
    }

    /// Convenience constructor for OR of multiple expressions
    pub fn or(exprs: Vec<QueryExpr>) -> QueryExpr {
        QueryExpr::Logical { op: LogicalOp::Or, exprs }
    }

    /// Convenience constructor for NOT of an expression
    pub fn not(expr: QueryExpr) -> QueryExpr {
        QueryExpr::Not(Box::new(expr))
    }
}
