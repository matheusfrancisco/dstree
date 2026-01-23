//! Common types, traits, and utilities shared across all tree implementations.
//!
//! This module provides:
//! - Core trait definitions for tree operations
//! - Error types for tree operations
//! - Visualization utilities

pub mod error;
pub mod traits;
pub mod visualize;

// Re-export commonly used types
pub use error::{Result, TreeError};
pub use traits::{OrderedTree, Traversable, Tree, Visitor};
