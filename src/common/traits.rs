//! Core trait definitions for tree data structures.
//!
//! This module defines common interfaces that all tree implementations should follow,
//! promoting code reuse and consistent APIs across different tree types.

use super::error::Result;

/// Common operations for all tree-like structures.
///
/// This trait defines the most basic operations that any tree should support,
/// regardless of its specific type or ordering properties.
///
/// # Examples
///
/// ```rust,ignore
/// use dstree::common::traits::Tree;
/// use dstree::bst::BST;
///
/// let mut tree: BST<i32> = BST::new();
/// assert!(tree.is_empty());
/// assert_eq!(tree.len(), 0);
/// ```
pub trait Tree<T> {
    /// Returns the number of elements in the tree.
    fn len(&self) -> usize;

    /// Returns `true` if the tree contains no elements.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dstree::common::traits::Tree;
    /// use dstree::bst::BST;
    ///
    /// let tree: BST<i32> = BST::new();
    /// assert!(tree.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes all elements from the tree.
    fn clear(&mut self);

    /// Returns the height of the tree.
    ///
    /// The height is defined as the number of edges on the longest path
    /// from the root to a leaf. An empty tree has height 0.
    fn height(&self) -> usize;
}

/// Trees that maintain elements in sorted order and support efficient search operations.
///
/// This trait extends `Tree` with operations that require ordering semantics.
/// The generic type `T` must implement `Ord` to enable comparison operations.
///
/// # Rust Concepts
///
/// - **Generic Constraints**: `T: Ord` ensures elements can be compared
/// - **Trait Inheritance**: Inherits all methods from `Tree<T>`
/// - **Borrowing**: Methods like `contains` take `&T` to avoid unnecessary copies
///
/// # Examples
///
/// ```rust,ignore
/// use dstree::common::traits::OrderedTree;
/// use dstree::bst::BST;
///
/// let mut tree = BST::new();
/// tree.insert(5).unwrap();
/// tree.insert(3).unwrap();
/// tree.insert(7).unwrap();
///
/// assert!(tree.contains(&5));
/// assert_eq!(tree.min(), Some(&3));
/// assert_eq!(tree.max(), Some(&7));
/// ```
pub trait OrderedTree<T: Ord>: Tree<T> {
    /// Inserts a value into the tree.
    ///
    /// # Rust Concepts
    ///
    /// - Takes ownership of `value` (moves it into the tree)
    /// - Requires `&mut self` for exclusive access during modification
    ///
    /// # Errors
    ///
    /// Returns an error if the insertion violates tree invariants
    /// (implementation-specific).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dstree::common::traits::OrderedTree;
    /// use dstree::bst::BST;
    ///
    /// let mut tree = BST::new();
    /// tree.insert(5).unwrap();
    /// assert_eq!(tree.len(), 1);
    /// ```
    fn insert(&mut self, value: T) -> Result<()>;

    /// Removes a value from the tree and returns it.
    ///
    /// # Rust Concepts
    ///
    /// - Borrows `value` (`&T`) since we only need to compare, not own
    /// - Returns ownership of the removed value to the caller
    ///
    /// # Errors
    ///
    /// Returns `TreeError::NodeNotFound` if the value doesn't exist.
    fn remove(&mut self, value: &T) -> Result<T>;

    /// Returns `true` if the tree contains the specified value.
    ///
    /// # Rust Concepts
    ///
    /// - Uses `&self` (immutable borrow) allowing multiple concurrent searches
    /// - Takes `&T` to avoid moving or copying the search value
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use dstree::common::traits::OrderedTree;
    /// use dstree::bst::BST;
    ///
    /// let mut tree = BST::new();
    /// tree.insert(5).unwrap();
    /// assert!(tree.contains(&5));
    /// assert!(!tree.contains(&10));
    /// ```
    fn contains(&self, value: &T) -> bool;

    /// Returns a reference to the minimum value in the tree.
    ///
    /// Returns `None` if the tree is empty.
    fn min(&self) -> Option<&T>;

    /// Returns a reference to the maximum value in the tree.
    ///
    /// Returns `None` if the tree is empty.
    fn max(&self) -> Option<&T>;
}

/// The Visitor pattern for tree traversal.
///
/// Implement this trait to define custom operations during tree traversal.
/// The visitor's `visit` method is called for each node in the traversal order.
///
/// # Design Pattern
///
/// This implements the classic Visitor pattern, separating traversal algorithm
/// from the operation performed on each node.
///
/// # Examples
///
/// ```rust
/// use dstree::common::traits::Visitor;
///
/// struct SumVisitor {
///     sum: i32,
/// }
///
/// impl Visitor<i32> for SumVisitor {
///     fn visit(&mut self, value: &i32) {
///         self.sum += value;
///     }
/// }
/// ```
pub trait Visitor<T> {
    /// Called for each node value during traversal.
    ///
    /// # Rust Concepts
    ///
    /// - Takes `&mut self` to allow the visitor to maintain state
    /// - Borrows `value` (`&T`) since we don't need ownership
    fn visit(&mut self, value: &T);
}

/// Trees that support various traversal orders.
///
/// This trait provides different ways to walk through all nodes in the tree.
/// Each traversal order serves different use cases.
///
/// # Traversal Orders
///
/// - **In-order**: Left subtree → Node → Right subtree (sorted order for BST)
/// - **Pre-order**: Node → Left subtree → Right subtree (useful for copying)
/// - **Post-order**: Left subtree → Right subtree → Node (useful for deletion)
/// - **Level-order**: Level by level, left to right (breadth-first)
///
/// # Examples
///
/// ```rust,ignore
/// use dstree::common::traits::{Traversable, Visitor};
/// use dstree::bst::BST;
///
/// struct PrintVisitor;
///
/// impl Visitor<i32> for PrintVisitor {
///     fn visit(&mut self, value: &i32) {
///         print!("{} ", value);
///     }
/// }
///
/// let mut tree = BST::new();
/// tree.insert(5);
/// tree.insert(3);
/// tree.insert(7);
///
/// let mut visitor = PrintVisitor;
/// tree.traverse_inorder(&mut visitor);  // Prints: 3 5 7
/// ```
pub trait Traversable<T> {
    /// Traverses the tree in in-order (left-node-right).
    ///
    /// For binary search trees, this visits nodes in sorted order.
    fn traverse_inorder<V: Visitor<T>>(&self, visitor: &mut V);

    /// Traverses the tree in pre-order (node-left-right).
    ///
    /// Useful for creating copies or serializing trees.
    fn traverse_preorder<V: Visitor<T>>(&self, visitor: &mut V);

    /// Traverses the tree in post-order (left-right-node).
    ///
    /// Useful for deletion or calculating aggregate properties.
    fn traverse_postorder<V: Visitor<T>>(&self, visitor: &mut V);

    /// Traverses the tree level by level (breadth-first).
    ///
    /// Visits all nodes at depth d before visiting nodes at depth d+1.
    fn traverse_levelorder<V: Visitor<T>>(&self, visitor: &mut V);
}

/// Trees that support range query operations.
///
/// This trait is typically implemented by structures like Segment Trees
/// that efficiently answer queries over ranges of data.
///
/// # Type Parameters
///
/// - `T`: The type of values stored and returned by queries
///
/// # Examples
///
/// ```rust,ignore
/// use dstree::common::traits::RangeQueryTree;
///
/// let mut tree = SegmentTree::new(vec![1, 3, 5, 7, 9, 11]);
///
/// // Query sum of elements from index 1 to 4
/// let sum = tree.range_query(1, 4);
/// assert_eq!(sum, 24);  // 3 + 5 + 7 + 9
///
/// // Update value at index 2
/// tree.update(2, 10);
/// ```
pub trait RangeQueryTree<T>: Tree<T> {
    /// Performs a query over the range [left, right].
    ///
    /// The specific operation (sum, min, max, etc.) depends on the implementation.
    ///
    /// # Arguments
    ///
    /// - `left`: Left boundary of the range (inclusive)
    /// - `right`: Right boundary of the range (inclusive)
    ///
    /// # Returns
    ///
    /// The result of the range operation.
    fn range_query(&self, left: usize, right: usize) -> T;

    /// Updates the value at the specified index.
    ///
    /// # Arguments
    ///
    /// - `index`: The position to update
    /// - `value`: The new value
    fn update(&mut self, index: usize, value: T);
}

/// Trees with parent pointers allowing upward traversal.
///
/// This trait provides navigation from child nodes to their parents,
/// enabling operations that require upward traversal in the tree.
///
/// # Rust Concepts
///
/// Implementations typically use `Weak<T>` for parent pointers to avoid
/// reference cycles that would cause memory leaks.
///
/// # Examples
///
/// ```rust,ignore
/// use dstree::common::traits::ParentTree;
///
/// let tree = BSTWithParent::new();
/// // ... insert nodes ...
///
/// if let Some(node_ref) = tree.root() {
///     if let Some(parent_ref) = tree.parent(&node_ref) {
///         println!("Found parent!");
///     }
/// }
/// ```
pub trait ParentTree<T>: Tree<T> {
    /// A reference to a node in the tree.
    ///
    /// The specific type depends on the implementation (e.g., `Rc<RefCell<Node<T>>>`)
    type NodeRef;

    /// Returns a reference to the parent of the given node.
    ///
    /// Returns `None` if the node is the root or has no parent.
    fn parent(&self, node: &Self::NodeRef) -> Option<Self::NodeRef>;

    /// Returns a reference to the root node.
    ///
    /// Returns `None` if the tree is empty.
    fn root(&self) -> Option<Self::NodeRef>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example test implementations
    struct MockTree {
        size: usize,
    }

    impl Tree<i32> for MockTree {
        fn len(&self) -> usize {
            self.size
        }

        fn clear(&mut self) {
            self.size = 0;
        }

        fn height(&self) -> usize {
            0
        }
    }

    #[test]
    fn test_is_empty_default_impl() {
        let tree = MockTree { size: 0 };
        assert!(tree.is_empty());

        let tree = MockTree { size: 5 };
        assert!(!tree.is_empty());
    }

    struct CollectVisitor {
        values: Vec<i32>,
    }

    impl Visitor<i32> for CollectVisitor {
        fn visit(&mut self, value: &i32) {
            self.values.push(*value);
        }
    }

    #[test]
    fn test_visitor_pattern() {
        let mut visitor = CollectVisitor { values: Vec::new() };
        visitor.visit(&1);
        visitor.visit(&2);
        visitor.visit(&3);

        assert_eq!(visitor.values, vec![1, 2, 3]);
    }
}
