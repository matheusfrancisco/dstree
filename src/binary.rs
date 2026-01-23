//! Basic binary tree implementation using Box<T>.
//!
//! This module will teach:
//! - Why Box<T> is necessary for recursive types
//! - Ownership and move semantics
//! - Pattern matching on nested structures
//!

//! ## What is?
//! A Binary Tree is a hierarchical data structure where each node
//! can have at most two children: a left child and a right child.
//! [5]           ← Root Node
//! /   \
//! [3]   [7]        ← Internal Nodes
//! /  \   /  \
//! [1] [4][6] [9]      ← Leaf Nodes
//!
//!Key Concepts Taught:
//!Root: The topmost node of the tree.
//!Parent: A node with children
//!Children: nodes directly connected to a parent node
//!Leaf: A node with no children
//!Height: The length of the longest path from the root to a leaf.
//!Binary: Each node has at most two children.
//!
//!
//! Properties vs Other Trees:
//! - Unlike Binary Search Trees (BST), Binary Trees do not enforce any ordering on the nodes.
//! - They are more general and can represent various hierarchical structures.

use rand::random;

use crate::common::traits::Tree;

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>, // use option because a children may not exist
    right: Option<Box<Node<T>>>, // box<..> puts the node on the heap to allow for recursion
}

#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
    /// Creates a new empty Binary Tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // An iterative clear to avoid stack overflow on deep trees
    fn clear_it(&mut self) {
        if self.root.is_none() {
            return;
        }
        // use stack to iteratively clear nodes
        let mut stack = Vec::new();
        if let Some(root) = self.root.take() {
            stack.push(root);
        }

        while let Some(node) = stack.pop() {
            // take owenership of left and right children
            // to prevent recursive drops
            if let Some(left) = node.left {
                stack.push(left);
            }
            if let Some(right) = node.right {
                stack.push(right);
            }
            // Node is dropped here when it goes out of scope
        }
    }

    fn add_left(&mut self, parent: &mut Node<T>, value: T) {
        parent.left = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
    }

    fn add_right(&mut self, parent: &mut Node<T>, value: T) {
        parent.right = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
    }

    fn get_root(&self) -> Option<&Node<T>> {
        self.root.as_deref()
    }

    // insert will choose random side to insert the new value
    fn insert_random(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });
        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }
        let choose = [true, false];
        let side = choose[random::<usize>() % 2];
        let mut current = self.root.as_mut().unwrap();
        loop {
            match side {
                true => {
                    if current.left.is_none() {
                        current.left = Some(new_node);
                        break;
                    } else {
                        current = current.left.as_mut().unwrap();
                    }
                }
                false => {
                    if current.right.is_none() {
                        current.right = Some(new_node);
                        break;
                    } else {
                        current = current.right.as_mut().unwrap();
                    }
                }
            }
        }
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Tree<T> for BinaryTree<T> {
    fn len(&self) -> usize {
        // Helper function to recursively count nodes
        fn count_nodes<T>(node: &Option<Box<Node<T>>>) -> usize {
            match node {
                Some(n) => 1 + count_nodes(&n.left) + count_nodes(&n.right),
                None => 0,
            }
        }

        count_nodes(&self.root)
    }

    // Clears the tree, removing all nodes.
    fn clear(&mut self) {
        self.root = None;
    }

    fn height(&self) -> usize {
        // Helper function to recursively calculate height
        // this could lead to stack overflow on deep trees
        // fn node_height<T>(node: &Option<Box<Node<T>>>) -> usize {
        //     match node {
        //         Some(n) => 1 + usize::max(node_height(&n.left), node_height(&n.right)),
        //         None => 0,
        //     }
        // }
        // node_height(&self.root)

        // Use iterative approach to avoid stack overflow
        let mut max_height = 0;
        if self.root.is_none() {
            return 0;
        }
        let mut stack: Vec<(&Node<T>, usize)> = Vec::new();

        if let Some(root) = self.root.as_deref() {
            stack.push((root, 1));
        }

        while let Some((node, height)) = stack.pop() {
            max_height = usize::max(max_height, height);
            if let Some(ref left) = node.left {
                stack.push((left.as_ref(), height + 1));
            }
            if let Some(ref right) = node.right {
                stack.push((right.as_ref(), height + 1));
            }
        }
        max_height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_creation() {
        let tree: BinaryTree<i32> = BinaryTree { root: None };
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_binary_tree_is_empty() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_binary_tree_is_not_empty() {
        let node = Node {
            value: 10,
            left: None,
            right: None,
        };
        let tree = BinaryTree {
            root: Some(Box::new(node)),
        };
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_binary_tree_len() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree.len(), 0);

        tree.root = Some(Box::new(Node {
            value: 10,
            left: Some(Box::new(Node {
                value: 5,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                value: 15,
                left: None,
                right: None,
            })),
        }));
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn test_binary_tree_height() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree.height(), 0);

        tree.root = Some(Box::new(Node {
            value: 10,
            left: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 3,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            right: Some(Box::new(Node {
                value: 15,
                left: None,
                right: None,
            })),
        }));
        assert_eq!(tree.height(), 3);
    }
}
