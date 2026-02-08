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
use std::collections::VecDeque;

use crate::common::error::{Result, TreeError};
use crate::common::traits::{OrderedTree, Tree};
use crate::{Traversable, Visitor};

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
                                    //this height update is probably not so efficient
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

fn node_values<V, N>(nodes: V) -> Vec<N>
where
    V: IntoIterator<Item = N>,
{
    nodes.into_iter().collect()
}

impl<T: Ord + std::fmt::Debug> Traversable<T> for BST<T> {
    fn traverse_inorder<V: crate::Visitor<T>>(&self, visitor: &mut V) {
        fn inorder<T: Ord, V: crate::Visitor<T>>(node: &Option<Box<Node<T>>>, visitor: &mut V) {
            if let Some(n) = node {
                inorder(&n.left, visitor);
                visitor.visit(&n.value);
                inorder(&n.right, visitor);
            }
        }
        inorder(&self.root, visitor);
    }

    fn traverse_preorder<V: crate::Visitor<T>>(&self, visitor: &mut V) {
        if self.root.is_none() {
            return;
        }
        let mut q = VecDeque::<&Box<Node<_>>>::new();
        let root = self.root.as_ref().unwrap();
        q.push_back(root);
        while let Some(node) = q.pop_front() {
            visitor.visit(&node.value);
            if let Some(ref r) = node.right {
                q.push_front(r);
            }
            if let Some(ref l) = node.left {
                q.push_front(l);
            }
        }
    }

    fn traverse_postorder<V: crate::Visitor<T>>(&self, visitor: &mut V) {
        if self.root.is_none() {
            return;
        }
        let mut q = Vec::<&Box<Node<_>>>::new();
        let mut curr = self.root.as_ref();
        let mut last_visited: Option<&Box<Node<T>>> = None;

        while curr.is_some() || !q.is_empty() {
            //go left as far as possible
            print!("q [");
            for (i, n) in q.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{:?}", &n.value);
            }
            println!("]");

            while let Some(n) = curr {
                q.push(n);
                curr = n.left.as_ref();
            }
            println!("current {:?}", curr.map(|n| &n.value));

            print!("q [");
            for (i, n) in q.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{:?}", &n.value);
            }
            println!("]");
            println!("last_visited: {:?}", last_visited.map(|n| &n.value));
            let peek = match q.last().copied() {
                None => break,
                Some(n) => n,
            };
            if let Some(right) = peek.right.as_ref() {
                if last_visited.map(|v| std::ptr::eq(v, right)) != Some(true) {
                    curr = Some(right);
                    continue;
                } else {
                    println!("right child {:?} already visited", right.value);
                }
            }
            println!("visiting {:?} and popping from q", peek.value);
            q.pop();
            visitor.visit(&peek.value);
            last_visited = Some(peek);
        }
    }

    fn traverse_levelorder<V: crate::Visitor<T>>(&self, visitor: &mut V) {
        if self.root.is_none() {
            return;
        }
        let mut q = std::collections::VecDeque::<&Box<Node<_>>>::new();
        let root = self.root.as_ref().unwrap();

        q.push_back(root);
        visitor.visit(&root.value);
        while !q.is_empty() {
            for i in 0..q.len() {
                let node_value = q.pop_front().unwrap();
                if let Some(ref left) = node_value.left {
                    visitor.visit(&left.value);
                    q.push_back(left);
                }
                if let Some(ref right) = node_value.right {
                    visitor.visit(&right.value);
                    q.push_back(right);
                }
            }
        }
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
        fn find_node<T: Ord>(node: &Option<Box<Node<T>>>, value: &T) -> bool {
            match value.cmp(&node.as_ref().unwrap().value) {
                Ordering::Equal => true,
                Ordering::Less => find_node(&node.as_ref().unwrap().left, value),
                Ordering::Greater => find_node(&node.as_ref().unwrap().right, value),
            }
        }
        let node = &self.root;
        find_node(node, value)
    }

    fn min(&self) -> Option<&T> {
        fn most_left_iter<T: Ord>(mut node: &Box<Node<T>>) -> &T {
            while let Some(left) = &node.left {
                node = left;
            }
            &node.value
        }

        fn most_left<T: Ord>(node: &Box<Node<T>>) -> &T {
            match **node {
                Node {
                    left: None,
                    ref value,
                    ..
                } => value,
                Node {
                    left: Some(ref left),
                    ..
                } => most_left(left),
            }
        }
        match &self.root {
            None => None,
            Some(root) => Some(most_left(root)),
        }
    }
    fn max(&self) -> Option<&T> {
        fn most_right_iter<T: Ord>(mut node: &Node<T>) -> &T {
            while let Some(right) = &node.right {
                node = right;
            }
            &node.value
        }

        fn most_right<T: Ord>(node: &Node<T>) -> &T {
            match &node.right {
                None => &node.value,
                Some(right) => most_right(right),
            }
        }
        match &self.root {
            None => None,
            Some(root) => Some(most_right(root)),
        }
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

struct CollectVisitor {
    values: Vec<i32>,
}

impl Visitor<i32> for CollectVisitor {
    fn visit(&mut self, value: &i32) {
        self.values.push(*value); // Collect in vector
    }
}

#[cfg(test)]
mod tests {
    use crate::{OrderedTree, Traversable, Tree};

    #[test]
    fn test_inorder() {
        let mut bst = super::BST::new();
        let items = vec![30, 10, 8, 11, 1, 9, 60, 50, 70];
        for item in items {
            bst.insert(item).unwrap();
        }
        let mut visitor = super::CollectVisitor { values: vec![] };
        bst.traverse_inorder(&mut visitor);
        assert_eq!(visitor.values, vec![1, 8, 9, 10, 11, 30, 50, 60, 70]);
    }

    #[test]
    fn test_pos_order() {
        let mut bst = super::BST::new();
        let items = vec![30, 10, 8, 11, 1, 9, 60, 50, 70];
        for item in items {
            bst.insert(item).unwrap();
        }
        let mut visitor = super::CollectVisitor { values: vec![] };
        bst.traverse_postorder(&mut visitor);
        assert_eq!(visitor.values, vec![1, 9, 8, 11, 10, 50, 70, 60, 30]);
    }

    #[test]
    fn test_level_order() {
        let mut bst = super::BST::new();
        let items = vec![30, 10, 8, 11, 1, 9, 60, 50, 70];
        for item in items {
            bst.insert(item).unwrap();
        }
        let mut visitor = super::CollectVisitor { values: vec![] };
        bst.traverse_levelorder(&mut visitor);
        assert_eq!(visitor.values, vec![30, 10, 60, 8, 11, 50, 70, 1, 9]);
    }

    #[test]
    fn test_pre_order() {
        let mut bst = super::BST::new();
        let items = vec![30, 10, 8, 11, 1, 9, 60, 50, 70];
        for item in items {
            bst.insert(item).unwrap();
        }
        let mut visitor = super::CollectVisitor { values: vec![] };
        bst.traverse_preorder(&mut visitor);
        assert_eq!(visitor.values, vec![30, 10, 8, 1, 9, 11, 60, 50, 70]);
    }

    #[test]
    fn test_max_value() {
        let mut bst = super::BST::new();
        bst.insert(30).unwrap();
        bst.insert(10).unwrap();
        bst.insert(8).unwrap();
        bst.insert(2).unwrap();
        bst.insert(1).unwrap();
        bst.insert(60).unwrap();
        bst.insert(40).unwrap();
        bst.insert(32).unwrap();
        bst.insert(50).unwrap();
        bst.insert(70).unwrap();
        bst.insert(65).unwrap();
        bst.insert(67).unwrap();

        let max = bst.max();
        assert_eq!(max, Some(&70));
    }

    #[test]
    fn test_min_value() {
        let mut bst = super::BST::new();
        bst.insert(30).unwrap();
        bst.insert(10).unwrap();
        bst.insert(8).unwrap();
        bst.insert(2).unwrap();
        bst.insert(1).unwrap();
        bst.insert(60).unwrap();
        bst.insert(40).unwrap();
        bst.insert(32).unwrap();
        bst.insert(50).unwrap();
        bst.insert(70).unwrap();
        bst.insert(65).unwrap();
        bst.insert(67).unwrap();

        let min = bst.min();
        assert_eq!(min, Some(&1));
    }

    #[test]
    fn test_contains_value() {
        let mut bst = super::BST::new();
        bst.insert(30).unwrap();
        bst.insert(60).unwrap();
        bst.insert(40).unwrap();
        bst.insert(32).unwrap();
        bst.insert(50).unwrap();
        bst.insert(70).unwrap();
        bst.insert(65).unwrap();
        bst.insert(67).unwrap();

        let exp = bst.contains(&32);
        assert!(exp);
    }

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
