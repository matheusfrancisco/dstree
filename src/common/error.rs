//! Error types for tree operations.
//!
//! This module defines all possible errors that can occur during tree operations,
//! using the `thiserror` crate for ergonomic error handling.
//!
//! # Rust Concepts
//!
//! - **thiserror**: Derives `Error` trait implementation automatically
//! - **Error variants**: Each represents a specific failure mode
//! - **Error context**: Structured data attached to errors for debugging
//!
//! # Examples
//!
//! ```rust
//! use dstree::common::error::{TreeError, Result};
//!
//! fn insert_unique(tree: &mut Vec<i32>, value: i32) -> Result<()> {
//!     if tree.contains(&value) {
//!         return Err(TreeError::NodeExists);
//!     }
//!     tree.push(value);
//!     Ok(())
//! }
//! ```

use thiserror::Error;

/// Errors that can occur during tree operations.
///
/// This enum represents all possible error conditions in tree operations.
/// Each variant provides context-specific information to help diagnose issues.
///
/// # Design Philosophy
///
/// Following rust-skills m06-error-handling:
/// - Typed errors for better error handling
/// - Rich context for debugging
/// - Clear, descriptive error messages
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum TreeError {
    /// A node already exists at the specified location.
    ///
    /// This error occurs when attempting to insert a node where one already exists,
    /// or when inserting a duplicate value in a tree that doesn't allow duplicates.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::NodeExists;
    /// assert_eq!(err.to_string(), "Node already exists");
    /// ```
    #[error("Node already exists")]
    NodeExists,

    /// The requested node was not found in the tree.
    ///
    /// This error occurs when attempting to access, remove, or modify a node
    /// that doesn't exist in the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::NodeNotFound;
    /// assert_eq!(err.to_string(), "Node not found");
    /// ```
    #[error("Node not found")]
    NodeNotFound,

    /// The tree is empty and the operation requires a non-empty tree.
    ///
    /// This error occurs when attempting operations like finding min/max
    /// or removing the root on an empty tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::EmptyTree;
    /// assert_eq!(err.to_string(), "Tree is empty");
    /// ```
    #[error("Tree is empty")]
    EmptyTree,

    /// An invalid operation was attempted.
    ///
    /// This error occurs when an operation is semantically invalid
    /// for the current state of the tree or the operation itself.
    ///
    /// # Fields
    ///
    /// - `reason`: A detailed explanation of why the operation is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::InvalidOperation {
    ///     reason: "Cannot rotate a leaf node".to_string()
    /// };
    /// ```
    #[error("Invalid operation: {reason}")]
    InvalidOperation {
        /// Detailed explanation of the invalid operation
        reason: String,
    },

    /// A tree invariant was violated.
    ///
    /// This error indicates a bug in the tree implementation. Tree invariants
    /// (like BST property, AVL balance factor, Red-Black tree colors) should
    /// always be maintained by the implementation.
    ///
    /// # Fields
    ///
    /// - `invariant`: Description of which invariant was violated
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::InvariantViolation {
    ///     invariant: "BST property: left child > parent".to_string()
    /// };
    /// ```
    #[error("Invariant violation: {invariant}")]
    InvariantViolation {
        /// Description of the violated invariant
        invariant: String,
    },

    /// An index is out of bounds.
    ///
    /// This error occurs in array-based trees (like B-Trees or Segment Trees)
    /// when accessing an invalid index.
    ///
    /// # Fields
    ///
    /// - `index`: The invalid index that was accessed
    /// - `size`: The valid size of the structure
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::IndexOutOfBounds {
    ///     index: 10,
    ///     size: 5
    /// };
    /// assert_eq!(err.to_string(), "Index out of bounds: 10 >= 5");
    /// ```
    #[error("Index out of bounds: {index} >= {size}")]
    IndexOutOfBounds {
        /// The index that was out of bounds
        index: usize,
        /// The valid size
        size: usize,
    },

    /// A lock was poisoned in a concurrent tree operation.
    ///
    /// This error occurs in concurrent tree implementations when a thread
    /// panics while holding a lock, leaving the lock in a poisoned state.
    ///
    /// # Rust Concepts
    ///
    /// - **Lock poisoning**: Rust's mechanism to detect data corruption from panics
    /// - **Concurrent safety**: Prevents use of potentially corrupted data
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::LockPoisoned;
    /// assert_eq!(err.to_string(), "Lock poisoned");
    /// ```
    #[error("Lock poisoned")]
    LockPoisoned,

    /// A duplicate value was encountered in a tree that doesn't allow duplicates.
    ///
    /// # Fields
    ///
    /// - `message`: Optional context about the duplicate
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::DuplicateValue {
    ///     message: Some("Value 5 already exists".to_string())
    /// };
    /// ```
    #[error("Duplicate value{}", .message.as_ref().map(|m| format!(": {}", m)).unwrap_or_default())]
    DuplicateValue {
        /// Optional context message
        message: Option<String>,
    },

    /// An overflow occurred during a tree operation.
    ///
    /// This can occur in operations like height calculation or size tracking
    /// when values exceed representable limits.
    ///
    /// # Example
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::Overflow {
    ///     operation: "height calculation".to_string()
    /// };
    /// ```
    #[error("Overflow in {operation}")]
    Overflow {
        /// The operation that caused the overflow
        operation: String,
    },
}

/// A specialized `Result` type for tree operations.
///
/// This is a type alias that uses `TreeError` as the error type.
/// It's provided for convenience and consistency across the crate.
///
/// # Rust Concepts
///
/// - **Type alias**: Shorthand for a common type combination
/// - **Result pattern**: Rust's standard error handling mechanism
///
/// # Examples
///
/// ```rust
/// use dstree::common::error::{Result, TreeError};
///
/// fn might_fail() -> Result<i32> {
///     if true {
///         Ok(42)
///     } else {
///         Err(TreeError::EmptyTree)
///     }
/// }
/// ```
pub type Result<T> = std::result::Result<T, TreeError>;

impl TreeError {
    /// Creates an `InvalidOperation` error with the given reason.
    ///
    /// This is a convenience constructor for the most commonly created error variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::invalid_operation("Cannot delete from empty tree");
    /// ```
    pub fn invalid_operation(reason: impl Into<String>) -> Self {
        TreeError::InvalidOperation {
            reason: reason.into(),
        }
    }

    /// Creates an `InvariantViolation` error with the given invariant description.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::invariant_violation("Balance factor out of range");
    /// ```
    pub fn invariant_violation(invariant: impl Into<String>) -> Self {
        TreeError::InvariantViolation {
            invariant: invariant.into(),
        }
    }

    /// Creates an `IndexOutOfBounds` error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::index_out_of_bounds(10, 5);
    /// assert_eq!(err.to_string(), "Index out of bounds: 10 >= 5");
    /// ```
    pub fn index_out_of_bounds(index: usize, size: usize) -> Self {
        TreeError::IndexOutOfBounds { index, size }
    }

    /// Creates a `DuplicateValue` error with an optional message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::duplicate_value(Some("Value 42 already exists"));
    /// ```
    pub fn duplicate_value(message: Option<impl Into<String>>) -> Self {
        TreeError::DuplicateValue {
            message: message.map(|s| s.into()),
        }
    }

    /// Creates an `Overflow` error for the given operation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dstree::common::error::TreeError;
    ///
    /// let err = TreeError::overflow("size counter");
    /// ```
    pub fn overflow(operation: impl Into<String>) -> Self {
        TreeError::Overflow {
            operation: operation.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(TreeError::NodeExists.to_string(), "Node already exists");
        assert_eq!(TreeError::NodeNotFound.to_string(), "Node not found");
        assert_eq!(TreeError::EmptyTree.to_string(), "Tree is empty");
    }

    #[test]
    fn test_invalid_operation_constructor() {
        let err = TreeError::invalid_operation("test reason");
        assert_eq!(err.to_string(), "Invalid operation: test reason");
    }

    #[test]
    fn test_invariant_violation_constructor() {
        let err = TreeError::invariant_violation("BST property violated");
        assert_eq!(err.to_string(), "Invariant violation: BST property violated");
    }

    #[test]
    fn test_index_out_of_bounds() {
        let err = TreeError::index_out_of_bounds(10, 5);
        assert_eq!(err.to_string(), "Index out of bounds: 10 >= 5");
    }

    #[test]
    fn test_duplicate_value_with_message() {
        let err = TreeError::duplicate_value(Some("Value 42 exists"));
        assert!(err.to_string().contains("Value 42 exists"));
    }

    #[test]
    fn test_duplicate_value_without_message() {
        let err = TreeError::duplicate_value(None::<String>);
        assert_eq!(err.to_string(), "Duplicate value");
    }

    #[test]
    fn test_overflow() {
        let err = TreeError::overflow("height calculation");
        assert_eq!(err.to_string(), "Overflow in height calculation");
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(TreeError::NodeExists, TreeError::NodeExists);
        assert_ne!(TreeError::NodeExists, TreeError::NodeNotFound);
    }

    #[test]
    fn test_result_type_alias() {
        let success: Result<i32> = Ok(42);
        assert!(success.is_ok());

        let failure: Result<i32> = Err(TreeError::EmptyTree);
        assert!(failure.is_err());
    }
}
