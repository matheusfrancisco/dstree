//! Binary Search Tree implementation with Box<T>.
//!
//! This module demonstrates:
//! - **Generic constraints**: `T: Ord` for ordered comparisons
//! - **Box<T>**: Single ownership with heap allocation
//! - **Borrowing patterns**: Why search takes `&T` not `T`
//! - **Recursive algorithms**: Insert and search follow BST property
//!
//! ## BST Property (Invariant)
//!
//! For every node:
//! - All values in left subtree < node.value
//! - All values in right subtree > node.value
//! - Both subtrees are also BSTs
//!
//! ```text
//!       5
//!      / \
//!     3   7
//!    / \   \
//!   1   4   9
//! ```
//!
//! In-order traversal gives sorted sequence: [1, 3, 4, 5, 7, 9]

use std::cmp::Ordering;

use crate::common::error::{Result, TreeError};
use crate::common::traits::{OrderedTree, Tree};

#[derive(Debug)]
pub struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Option<Box<Node<T>>>,
    len: usize,    // holds the number of elements in the BST
    height: usize, // holds the height of the BST
}

impl<T> Default for BST<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BST<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        BST {
            root: None,
            len: 0,
            height: 0,
        }
    }
}

impl<T: Ord> BST<T> {
    /// Recursive remove: returns (new_subtree, Option<removed_value>).
    /// None means "not found".
    fn remove_from(node: Option<Box<Node<T>>>, value: &T) -> (Option<Box<Node<T>>>, Option<T>) {
        let mut node = match node {
            None => return (None, None), // Not found
            Some(n) => n,
        };
        match value.cmp(&node.value) {
            Ordering::Less => {
                let (new_left, result) = Self::remove_from(node.left.take(), value);
                node.left = new_left;
                (Some(node), result)
            }
            Ordering::Greater => {
                let (new_right, result) = Self::remove_from(node.right.take(), value);
                node.right = new_right;
                (Some(node), result)
            }
            Ordering::Equal => {
                let removed = node.value;
                match (node.left.take(), node.right.take()) {
                    (None, None) => (None, Some(removed)),             //Leaf node
                    (Some(left), None) => (Some(left), Some(removed)), //Only left child
                    (None, Some(right)) => (Some(right), Some(removed)), //Only right child
                    (Some(left), Some(right)) => {
                        // Replace node's value with predecessor's value
                        let (new_left, pred_value) = Self::take_rightmost(left);
                        node.left = new_left;
                        node.right = Some(right);
                        node.value = pred_value;
                        (Some(node), Some(removed))
                    }
                }
            }
        }
    }
    /// Helper to take the rightmost node from a subtree.
    fn take_rightmost(mut node: Box<Node<T>>) -> (Option<Box<Node<T>>>, T) {
        match node.right.take() {
            None => (node.left.take(), node.value),
            Some(right) => {
                let (new_right, value) = Self::take_rightmost(right);
                node.right = new_right;
                (Some(node), value)
            }
        }
    }

    /// Iterative remove: finds the node by walking with `take()`/put-back, then handles
    /// 0/1/2 children in place. Same contract as `remove` (trait method).
    pub fn remove_iterative(&mut self, value: &T) -> Result<T> {
        if self.root.is_none() {
            return Err(TreeError::EmptyTree);
        }
        let mut curr = &mut self.root;
        loop {
            let node_opt = curr.take();
            match node_opt {
                None => return Err(TreeError::NodeNotFound),
                Some(mut node) => {
                    match value.cmp(&node.value) {
                        Ordering::Less => {
                            *curr = Some(node);
                            curr = &mut curr.as_mut().expect("curr is Some after put-back").left;
                        }
                        Ordering::Greater => {
                            *curr = Some(node);
                            curr = &mut curr.as_mut().expect("curr is Some after put-back").right;
                        }
                        Ordering::Equal => {
                            self.len -= 1;
                            match (node.left.take(), node.right.take()) {
                                (None, None) => {
                                    *curr = None;
                                    self.update_height();
                                    return Ok(node.value);
                                }
                                (Some(left), None) => {
                                    *curr = Some(left);
                                    self.update_height();
                                    return Ok(node.value);
                                }
                                (None, Some(right)) => {
                                    *curr = Some(right);
                                    self.update_height();
                                    return Ok(node.value);
                                }
                                (Some(left), Some(right)) => {
                                    node.left = Some(left);
                                    node.right = Some(right);

                                    // iterative find predecessor
                                    let mut pred_curr = &mut node.left;
                                    while pred_curr.as_ref().expect("has left").right.is_some() {
                                        pred_curr =
                                            &mut pred_curr.as_mut().expect("has left").right;
                                    }

                                    let pred_node = pred_curr.take().expect("predecessor exists");
                                    *pred_curr = pred_node.left;

                                    let removed =
                                        std::mem::replace(&mut node.value, pred_node.value);

                                    *curr = Some(node);
                                    self.update_height();
                                    return Ok(removed);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    /// Recomputes tree height from root (number of edges on longest root-to-leaf path).
    fn update_height(&mut self) {
        fn node_height<T: Ord>(node: &Option<Box<Node<T>>>) -> usize {
            match node {
                None => 0,
                Some(n) => 1 + usize::max(node_height(&n.left), node_height(&n.right)),
            }
        }
        self.height = node_height(&self.root).saturating_sub(1);
    }
}

impl<T: Ord> OrderedTree<T> for BST<T> {
    fn insert(&mut self, value: T) -> Result<()> {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(value)));
            self.len += 1;
            self.update_height();
            return Ok(());
        }

        fn insert_into<T: Ord>(node: &mut Box<Node<T>>, value: T) -> bool {
            match value.cmp(&node.value) {
                Ordering::Less => match node.left.as_mut() {
                    Some(left) => insert_into(left, value),
                    None => {
                        node.left = Some(Box::new(Node::new(value)));
                        true
                    }
                },
                Ordering::Greater => match node.right.as_mut() {
                    Some(right) => insert_into(right, value),
                    None => {
                        node.right = Some(Box::new(Node::new(value)));
                        true
                    }
                },

                Ordering::Equal => {
                    false // no duplicate
                }
            }
        }
        if insert_into(self.root.as_mut().unwrap(), value) {
            self.len += 1;
            self.update_height();
        };
        Ok(())
    }

    /// Remove using the recursive implementation.
    fn remove(&mut self, value: &T) -> Result<T> {
        if self.root.is_none() {
            return Err(TreeError::EmptyTree);
        }
        let (new_root, result) = Self::remove_from(self.root.take(), value);
        self.root = new_root;
        match result {
            None => Err(TreeError::NodeNotFound),
            Some(v) => {
                self.len -= 1;
                self.update_height();
                Ok(v)
            }
        }
    }

    fn contains(&self, value: &T) -> bool {
        unimplemented!()
    }
    fn min(&self) -> Option<&T> {
        unimplemented!()
    }
    fn max(&self) -> Option<&T> {
        unimplemented!()
    }
}

impl<T: Ord> Tree<T> for BST<T> {
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn len(&self) -> usize {
        self.len
    }

    fn height(&self) -> usize {
        self.height
    }

    fn clear(&mut self) {
        self.root = None;
        self.len = 0;
        self.height = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::{OrderedTree, Tree};

    #[test]
    fn test_insert_bst() {
        let mut bst = super::BST::new();
        bst.insert(30).unwrap();
        bst.insert(60).unwrap();
        bst.insert(40).unwrap();
        bst.insert(32).unwrap();
        bst.insert(50).unwrap();
        bst.insert(70).unwrap();
        bst.insert(65).unwrap();
        bst.insert(67).unwrap();

        assert_eq!(bst.len(), 8);
        assert_eq!(bst.height(), 4);
        println!("{:#?}", bst);
    }

    #[test]
    fn test_bst_remove() {
        let mut bst = super::BST::new();
        bst.root = Some(Box::new(super::Node {
            value: 30,
            left: None,
            right: Some(Box::new(super::Node {
                value: 60,
                left: Some(Box::new(super::Node {
                    value: 40,
                    left: Some(Box::new(super::Node::new(32))),
                    right: Some(Box::new(super::Node::new(50))),
                })),
                right: Some(Box::new(super::Node {
                    value: 70,
                    left: Some(Box::new(super::Node {
                        value: 65,
                        left: None,
                        right: Some(Box::new(super::Node {
                            value: 67,
                            left: None,
                            right: Some(Box::new(super::Node::new(68))),
                        })),
                    })),
                    right: None,
                })),
            })),
        }));
        bst.len = 7;
        bst.height = 4;

        let removed = bst.remove(&70).unwrap();
        assert_eq!(removed, 70);
        println!("{:#?}", bst);
    }

    #[test]
    fn test_bst_only_left_remove() {
        let mut bst = super::BST::new();
        bst.root = Some(Box::new(super::Node {
            value: 30,
            left: None,
            right: Some(Box::new(super::Node {
                value: 60,
                left: Some(Box::new(super::Node {
                    value: 40,
                    left: Some(Box::new(super::Node::new(32))),
                    right: Some(Box::new(super::Node::new(50))),
                })),
                right: None,
            })),
        }));
        bst.len = 4;
        bst.height = 3;

        let removed = bst.remove(&60).unwrap();
        assert_eq!(removed, 60);
        println!("{:#?}", bst);
    }

    #[test]
    fn test_bst_test_two() {
        let mut bst = super::BST::new();
        bst.root = Some(Box::new(super::Node {
            value: 30,
            left: None,
            right: Some(Box::new(super::Node {
                value: 80,
                left: Some(Box::new(super::Node {
                    value: 60,
                    left: None,
                    right: Some(Box::new(super::Node {
                        value: 61,
                        left: None,
                        right: Some(Box::new(super::Node {
                            value: 65,
                            left: Some(Box::new(super::Node::new(63))),
                            right: None,
                        })),
                    })),
                })),
                right: Some(Box::new(super::Node::new(90))),
            })),
        }));
        bst.len = 7;
        bst.height = 5;
        let removed = bst.remove(&80).unwrap();
        assert_eq!(removed, 80);
        println!("{:#?}", bst);
    }

    #[test]
    fn test_bst_remove_iterative() {
        let mut bst = super::BST::new();
        bst.root = Some(Box::new(super::Node {
            value: 30,
            left: None,
            right: Some(Box::new(super::Node {
                value: 60,
                left: Some(Box::new(super::Node {
                    value: 40,
                    left: Some(Box::new(super::Node::new(32))),
                    right: Some(Box::new(super::Node::new(50))),
                })),
                right: Some(Box::new(super::Node {
                    value: 70,
                    left: Some(Box::new(super::Node {
                        value: 65,
                        left: None,
                        right: Some(Box::new(super::Node {
                            value: 67,
                            left: Some(Box::new(super::Node::new(68))),
                            right: None,
                        })),
                    })),
                    right: None,
                })),
            })),
        }));
        bst.len = 7;
        bst.height = 4;

        let removed = bst
            .remove_iterative(&70)
            .expect("remove_iterative(70) should succeed");
        assert_eq!(removed, 70);
    }

    #[test]
    fn test_bst_only_left_remove_iterative() {
        let mut bst = super::BST::new();
        bst.root = Some(Box::new(super::Node {
            value: 30,
            left: None,
            right: Some(Box::new(super::Node {
                value: 60,
                left: Some(Box::new(super::Node {
                    value: 40,
                    left: Some(Box::new(super::Node::new(32))),
                    right: Some(Box::new(super::Node::new(50))),
                })),
                right: None,
            })),
        }));
        bst.len = 4;
        bst.height = 3;

        let removed = bst
            .remove_iterative(&60)
            .expect("remove_iterative(60) should succeed");
        assert_eq!(removed, 60);
    }
}
