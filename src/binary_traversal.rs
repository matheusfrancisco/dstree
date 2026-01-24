use std::collections::VecDeque;
use std::fmt::Debug;

use crate::binary::{BinaryTree, Node};
use crate::common::traits::{Traversable, Visitor};

// the idea of visitor is to separete the traversal
// logic from the action taken at each node,
// allow the user to define custom actions
// for e.g if you want to collect values, print them,sum, find max, etc
// with visitor you can implement the action without changing the traversal logic
struct SumVisitor {
    sum: i32,
}

impl Visitor<i32> for SumVisitor {
    fn visit(&mut self, value: &i32) {
        self.sum += value;
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

struct DefaultVisitor {}

impl<T: Debug> Visitor<T> for DefaultVisitor {
    fn visit(&mut self, value: &T) {
        println!("{:?}", value);
    }
}

// Recursive inorder traversal
fn inoder_traversal<T: Debug>(tree: &BinaryTree<T>, visitor: &mut impl Visitor<T>) {
    fn inorder_helper<T: Debug>(node: &Option<Box<Node<T>>>, visitor: &mut impl Visitor<T>) {
        if let Some(n) = node {
            inorder_helper(&n.left, visitor);
            visitor.visit(&n.value);
            inorder_helper(&n.right, visitor);
        }
    }
    inorder_helper(&tree.root, visitor);
}

// Recursive preorder traversal
fn preorder_traversal<T: Debug>(tree: &BinaryTree<T>, visitor: &mut impl Visitor<T>) {
    fn preorder_helper<T: Debug>(node: &Option<Box<Node<T>>>, visitor: &mut impl Visitor<T>) {
        if let Some(n) = node {
            visitor.visit(&n.value);
            preorder_helper(&n.left, visitor);
            preorder_helper(&n.right, visitor);
        }
    }
    preorder_helper(&tree.root, visitor);
}

fn postorder_traversal<T: Debug>(tree: &BinaryTree<T>, visitor: &mut impl Visitor<T>) {
    fn postorder_helper<T: Debug>(node: &Option<Box<Node<T>>>, visitor: &mut impl Visitor<T>) {
        if let Some(n) = node {
            postorder_helper(&n.left, visitor);
            postorder_helper(&n.right, visitor);
            visitor.visit(&n.value);
        }
    }
    postorder_helper(&tree.root, visitor);
}

fn levelorder_traversal<T: Debug>(tree: &BinaryTree<T>, visitor: &mut impl Visitor<T>) {
    let height = tree.height();
    fn height_helper<T: Debug>(
        node: &Option<Box<Node<T>>>,
        level: usize,
        visitor: &mut impl Visitor<T>,
    ) {
        if let Some(n) = node {
            if level == 0 {
                visitor.visit(&n.value);
            } else {
                height_helper(&n.left, level - 1, visitor);
                height_helper(&n.right, level - 1, visitor);
            }
        }
    }
    for level in 0..height {
        height_helper(&tree.root, level, visitor);
    }
}

pub struct InOrderIter<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

pub struct InOrderIterMut<'a, T> {
    stack: Vec<&'a mut Node<T>>,
    current: Option<&'a mut Node<T>>,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for InOrderIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current.is_some() || !self.stack.is_empty() {
            while let Some(node) = self.current {
                self.stack.push(node);
                self.current = node.left.as_deref();
            }
            if let Some(node) = self.stack.pop() {
                self.current = node.right.as_deref();
                return Some(&node.value);
            }
        }
        None
    }
}

impl<T> BinaryTree<T> {
    pub fn get_root_mut(&mut self) -> Option<&mut Node<T>> {
        self.root.as_deref_mut()
    }

    pub fn inorder_iter(&self) -> InOrderIter<'_, T> {
        InOrderIter {
            stack: Vec::new(),
            current: self.get_root(),
        }
    }
    pub fn inorder_iter_mut(&mut self) -> InOrderIterMut<'_, T> {
        let current = self.root.as_deref_mut().map(|n| n as *mut Node<T>);

        InOrderIterMut {
            stack: Vec::new(),
            current: current.map(|ptr| unsafe { &mut *ptr }),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> Iterator for InOrderIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current.is_some() || !self.stack.is_empty() {
            while let Some(node) = self.current.take() {
                // Push the current node onto the stack
                self.stack.push(unsafe { &mut *(node as *mut Node<T>) });
                // Move to the left child
                self.current = node.left.as_deref_mut();
            }
            if let Some(node) = self.stack.pop() {
                // Move to the right child
                self.current = node.right.as_deref_mut();
                return Some(&mut node.value);
            }
        }
        None
    }
}

impl<'a, T> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = InOrderIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<T> Traversable<T> for BinaryTree<T> {
    fn traverse_levelorder<V: crate::Visitor<T>>(&self, visitor: &mut V) {
        let mut q: VecDeque<&Node<T>> = VecDeque::new();
        if let Some(root) = self.get_root() {
            q.push_back(root);
        }
        while let Some(node) = q.pop_front() {
            visitor.visit(&node.value);
            if let Some(left) = node.left.as_deref() {
                q.push_back(left);
            }
            if let Some(right) = node.right.as_deref() {
                q.push_back(right);
            }
        }

        // with vec
        //let mut queue: Vec<&Node<T>> = Vec::new();
        //if let Some(root) = self.get_root() {
        //    queue.push(root);
        //}

        //while !queue.is_empty() {
        //    let node = queue.remove(0);
        //    visitor.visit(&node.value);
        //    if let Some(left) = node.left.as_deref() {
        //        queue.push(left);
        //    }
        //    if let Some(right) = node.right.as_deref() {
        //        queue.push(right);
        //    }
        //}
    }

    // Iterative Preorder traversal
    fn traverse_preorder<V: crate::Visitor<T>>(&self, visitor: &mut V) {
        let mut stack: Vec<&Node<T>> = Vec::new();
        let mut current = self.get_root();
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                visitor.visit(&node.value);
                stack.push(node);
                current = node.left.as_deref();
            }
            if let Some(node) = stack.pop() {
                current = node.right.as_deref();
            }
        }
    }

    // Iterative Inorder travers
    fn traverse_inorder<V: crate::Visitor<T>>(&self, visitor: &mut V) {
        let mut stack: Vec<&Node<T>> = Vec::new();
        let mut current = self.get_root();
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node);
                current = node.left.as_deref();
            }
            if let Some(node) = stack.pop() {
                visitor.visit(&node.value);
                current = node.right.as_deref();
            }
        }
    }

    fn traverse_postorder<V: crate::Visitor<T>>(&self, visitor: &mut V) {
        let mut stack1: Vec<&Node<T>> = Vec::new();
        let mut stack2: Vec<&Node<T>> = Vec::new();

        if let Some(root) = self.get_root() {
            stack1.push(root);
        }

        while let Some(node) = stack1.pop() {
            stack2.push(node);
            if let Some(left) = node.left.as_deref() {
                stack1.push(left);
            }
            if let Some(right) = node.right.as_deref() {
                stack1.push(right);
            }
        }

        while let Some(node) = stack2.pop() {
            visitor.visit(&node.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::BinaryTree;

    #[test]
    fn test_tree_inter() {
        let mut tree = BinaryTree::new();
        tree.add_root(4);
        let mut left = Box::new(Node::new(2));
        let mut right = Box::new(Node::new(6));
        right.left = Some(Box::new(Node::new(5)));
        right.right = Some(Box::new(Node::new(7)));
        left.left = Some(Box::new(Node::new(1)));
        left.right = Some(Box::new(Node::new(3)));
        tree.root.as_mut().unwrap().left = Some(left);
        tree.root.as_mut().unwrap().right = Some(right);

        let mut result = Vec::new();
        for value in &tree {
            result.push(*value);
        }

        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_iter_inorder() {
        let mut tree = BinaryTree::new();
        // layout is to bad here
        tree.add_root(4);
        let mut left = Box::new(Node::new(2));
        let mut right = Box::new(Node::new(6));
        right.left = Some(Box::new(Node::new(5)));
        right.right = Some(Box::new(Node::new(7)));
        left.left = Some(Box::new(Node::new(1)));
        left.right = Some(Box::new(Node::new(3)));
        tree.root.as_mut().unwrap().left = Some(left);
        tree.root.as_mut().unwrap().right = Some(right);

        let iter = tree.inorder_iter();
        let mut result = Vec::new();
        for value in iter {
            result.push(*value);
        }

        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_iter_mut_value_inorder() {
        let mut tree = BinaryTree::new();
        // layout is to bad here
        tree.add_root(4);
        let mut left = Box::new(Node::new(2));
        let mut right = Box::new(Node::new(6));
        right.left = Some(Box::new(Node::new(5)));
        right.right = Some(Box::new(Node::new(7)));
        left.left = Some(Box::new(Node::new(1)));
        left.right = Some(Box::new(Node::new(3)));
        tree.root.as_mut().unwrap().left = Some(left);
        tree.root.as_mut().unwrap().right = Some(right);

        //mute the first
        let mut iter_mut = tree.inorder_iter_mut();
        if let Some(value) = iter_mut.next() {
            *value = 10;
        }
        let iter = tree.inorder_iter();
        let mut result = Vec::new();
        for value in iter {
            result.push(*value);
        }
        assert_eq!(result, vec![10, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_inorder_traversal_collect() {
        let mut tree = BinaryTree::new();
        // layout is to bad here
        tree.add_root(4);
        let mut left = Box::new(Node::new(2));
        let mut right = Box::new(Node::new(6));
        right.left = Some(Box::new(Node::new(5)));
        right.right = Some(Box::new(Node::new(7)));
        left.left = Some(Box::new(Node::new(1)));
        left.right = Some(Box::new(Node::new(3)));
        tree.root.as_mut().unwrap().left = Some(left);
        tree.root.as_mut().unwrap().right = Some(right);

        let mut collector = CollectVisitor { values: Vec::new() };
        tree.traverse_inorder(&mut collector);

        assert_eq!(collector.values, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_inorder_req_traversal_collect() {
        let mut tree = BinaryTree::new();
        // layout is to bad here
        tree.add_root(4);
        let mut left = Box::new(Node::new(2));
        let mut right = Box::new(Node::new(6));
        right.left = Some(Box::new(Node::new(5)));
        right.right = Some(Box::new(Node::new(7)));
        left.left = Some(Box::new(Node::new(1)));
        left.right = Some(Box::new(Node::new(3)));
        tree.root.as_mut().unwrap().left = Some(left);
        tree.root.as_mut().unwrap().right = Some(right);

        let mut collector = CollectVisitor { values: Vec::new() };
        inoder_traversal(&tree, &mut collector);
        assert_eq!(collector.values, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_preorder() {
        let mut tree = BinaryTree::new();
        // layout is to bad here
        tree.add_root(4);
        let mut left = Box::new(Node::new(2));
        let mut right = Box::new(Node::new(6));
        right.left = Some(Box::new(Node::new(5)));
        right.right = Some(Box::new(Node::new(7)));
        left.left = Some(Box::new(Node::new(1)));
        left.right = Some(Box::new(Node::new(3)));
        tree.root.as_mut().unwrap().left = Some(left);
        tree.root.as_mut().unwrap().right = Some(right);

        let mut collector = CollectVisitor { values: Vec::new() };
        tree.traverse_preorder(&mut collector);

        assert_eq!(collector.values, vec![4, 2, 1, 3, 6, 5, 7]);

        // Test recursive preorder traversal

        let mut collector = CollectVisitor { values: Vec::new() };

        preorder_traversal(&tree, &mut collector);
        assert_eq!(collector.values, vec![4, 2, 1, 3, 6, 5, 7]);
    }

    #[test]
    fn test_postorder() {
        let mut tree = BinaryTree::new();
        // layout is to bad here
        tree.add_root(4);
        let mut left = Box::new(Node::new(2));
        let mut right = Box::new(Node::new(6));
        right.left = Some(Box::new(Node::new(5)));
        right.right = Some(Box::new(Node::new(7)));
        left.left = Some(Box::new(Node::new(1)));
        left.right = Some(Box::new(Node::new(3)));
        tree.root.as_mut().unwrap().left = Some(left);
        tree.root.as_mut().unwrap().right = Some(right);

        let mut collector = CollectVisitor { values: Vec::new() };
        tree.traverse_postorder(&mut collector);

        assert_eq!(collector.values, vec![1, 3, 2, 5, 7, 6, 4]);

        // recursive postorder traversal
        let mut collector = CollectVisitor { values: Vec::new() };
        postorder_traversal(&tree, &mut collector);
        assert_eq!(collector.values, vec![1, 3, 2, 5, 7, 6, 4]);
    }

    #[test]
    fn test_levelorder() {
        let mut tree = BinaryTree::new();
        // layout is to bad here
        tree.add_root(4);
        let mut left = Box::new(Node::new(2));
        let mut right = Box::new(Node::new(6));
        right.left = Some(Box::new(Node::new(5)));
        right.right = Some(Box::new(Node::new(7)));
        left.left = Some(Box::new(Node::new(1)));
        left.right = Some(Box::new(Node::new(3)));
        tree.root.as_mut().unwrap().left = Some(left);
        tree.root.as_mut().unwrap().right = Some(right);

        let mut collector = CollectVisitor { values: Vec::new() };
        tree.traverse_levelorder(&mut collector);

        assert_eq!(collector.values, vec![4, 2, 6, 1, 3, 5, 7]);

        // recursive levelorder traversal
        let mut collector = CollectVisitor { values: Vec::new() };
        levelorder_traversal(&tree, &mut collector);
        assert_eq!(collector.values, vec![4, 2, 6, 1, 3, 5, 7]);
    }
}
