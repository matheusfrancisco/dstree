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

use std::fmt::Debug;

use rand::random;

use crate::common::traits::Tree;
use crate::common::visualize::{TreeVisualBuilder, Visualize};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>, // use option because a children may not exist
    pub right: Option<Box<Node<T>>>, // box<..> puts the node on the heap to allow for recursion
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    pub root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
    /// Creates a new empty Binary Tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn height(&self) -> usize {
        fn node_height<T>(node: &Option<Box<Node<T>>>) -> usize {
            match node {
                Some(n) => 1 + usize::max(node_height(&n.left), node_height(&n.right)),
                None => 0,
            }
        }
        node_height(&self.root)
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

    pub fn add_left(&mut self, value: T) {
        if let Some(ref mut root) = self.root {
            if root.left.is_none() {
                root.left = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
                return;
            }
            let mut current = root.as_mut();
            while let Some(ref mut left_child) = current.left {
                current = left_child.as_mut();
            }
            current.left = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
        }
    }

    pub fn add_right(&mut self, value: T) {
        // add right child to root
        if let Some(ref mut root) = self.root {
            if root.right.is_none() {
                root.right = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
                return;
            }
            let mut current = root.as_mut();
            while let Some(ref mut right_child) = current.right {
                current = right_child.as_mut();
            }
            current.right = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
        }
    }

    pub fn add_root(&mut self, value: T) {
        self.root = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
    }

    pub fn get_root(&self) -> Option<&Node<T>> {
        self.root.as_deref()
    }

    pub fn insert(&mut self, value: T) {
        self.insert_random(value);
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

fn visualize_node<T: std::fmt::Display>(
    node: &Node<T>,
    builder: &mut TreeVisualBuilder,
    prefix: &str,
    is_last: bool,
) {
    let connector = if is_last { "└─" } else { "├─" };
    builder.add_line(format!("{}{} {}", prefix, connector, node.value));

    let new_prefix = format!("{}{}  ", prefix, if is_last { " " } else { "│" });

    if let Some(ref left) = node.left {
        visualize_node(left.as_ref(), builder, &new_prefix, node.right.is_none());
    }

    if let Some(ref right) = node.right {
        visualize_node(right.as_ref(), builder, &new_prefix, true);
    }
}

impl<T: std::fmt::Display> Visualize for BinaryTree<T> {
    fn visualize(&self) -> String {
        let mut builder = TreeVisualBuilder::new();
        if let Some(root) = &self.root {
            visualize_node(root, &mut builder, "", true);
        } else {
            builder.add_line("(empty tree)");
        }

        builder.build()
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

    #[test]
    fn test_insert_random() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.insert_random(10);
        assert_eq!(tree.len(), 1);

        tree.insert_random(5);
        tree.insert_random(15);
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn test_with_visualization() {
        let mut tree = BinaryTree::new();
        tree.add_root(10);
        tree.add_left(5);
        tree.add_left(53);
        tree.add_left(533);
        tree.add_right(15);
        tree.add_right(154);
        tree.add_right(158);

        // Use the Visualize trait
        println!("{}", tree.visualize());

        // Or with TreeVisualBuilder for metadata
        let mut builder = TreeVisualBuilder::new();
        builder.add_line(format!("Tree with {} nodes", tree.len()));
        builder.add_line(format!("Height: {}", tree.height()));
        builder.add_line("");

        tree.insert_random(29);
        tree.insert_random(28);
        tree.insert_random(21);
        tree.insert_random(22);
        tree.insert_random(23);
        builder.add_line(tree.visualize());
        println!("{}", builder.build());
    }
}
