//! # DSTree - Tree Data Structures in Rust
//!
//! A comprehensive library for learning Rust through tree data structure implementations.
//!
//! ## Overview
//!
//! This library provides various tree data structures with a focus on teaching Rust concepts:
//! - Ownership and borrowing patterns
//! - Smart pointers (Box, Rc, Arc, Weak)
//! - Interior mutability (RefCell, RwLock)
//! - Concurrent programming
//! - Generic programming and trait bounds
//!
//! ## Modules
//!
//! - `common`: Shared traits, error types, and utilities
//! - `binary`: Basic binary tree using Box<T>
//! - `bst`: Binary search tree with generic constraints
//! - `avl`: Self-balancing AVL tree (single-threaded and concurrent)
//!
//! ## Example
//!
//! ```rust,ignore
//! use dstree::bst::BST;
//!
//! let mut tree = BST::new();
//! tree.insert(5).unwrap();
//! tree.insert(3).unwrap();
//! tree.insert(7).unwrap();
//!
//! assert!(tree.contains(&5));
//! assert_eq!(tree.len(), 3);
//! ```
//!
//! Note: BST will be implemented in Phase 3.

// Re-export common types
pub mod common;

// Tree implementations
pub mod avl;
pub mod binary;
pub mod binary_traversal;
pub mod bst;
pub mod btree;

// Re-export commonly used types
pub use common::{
    error::{Result, TreeError},
    traits::{OrderedTree, Traversable, Tree, Visitor},
};
