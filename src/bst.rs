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
    fn remove_node_recursive(&mut self, value: &T) -> Result<T> {
        unimplemented!()
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
        unimplemented!()
    }

    fn remove(&mut self, value: &T) -> Result<T> {
        if self.root.is_none() {
            return Result::Err(TreeError::EmptyTree);
        }
        // first find the node
        let mut curr = &mut self.root;
        loop {
            let node_opt = curr.take(); // Take ownership immediately

            match node_opt {
                None => return Err(TreeError::NodeNotFound),
                Some(mut node) => {
                    match value.cmp(&node.value) {
                        Ordering::Less => {
                            *curr = Some(node); // Put it back
                            curr = &mut curr.as_mut().unwrap().left;
                        }
                        Ordering::Greater => {
                            *curr = Some(node); // Put it back
                            curr = &mut curr.as_mut().unwrap().right;
                        }
                        Ordering::Equal => {
                            self.len -= 1;
                            // We OWN node now, no borrow issues
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
                                    //children put back into the node
                                    node.left = Some(left);
                                    node.right = Some(right);
                                    // find in-order predecessor (rightmost of left subtree)
                                    let mut pred_parent = &mut node.left;
                                    while pred_parent.as_ref().unwrap().right.is_some() {
                                        pred_parent = &mut pred_parent.as_mut().unwrap().right;
                                    }
                                    // now pred_parent points to the predecessor's parent's child slot
                                    let mut predecessor = pred_parent.take().unwrap();
                                    *pred_parent = predecessor.left.take(); // Remove predecessor from its position
                                    // swap values: predecessor value goes to current node
                                    std::mem::swap(&mut node.value, &mut predecessor.value);
                                    *curr = Some(node); // Put back the current node
                                    self.update_height();
                                    return Ok(predecessor.value);
                                }
                            }
                        }
                    }
                }
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
    use crate::OrderedTree;

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
                            left: Some(Box::new(super::Node::new(68))),
                            right: None,
                        })),
                    })),
                    right: None,
                })),
            })),
        }));
        let removed = bst.remove(&70).unwrap();
        assert_eq!(removed, 70);
        println!("{:#?}", bst);
    }
}
