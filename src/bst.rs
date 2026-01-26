//! Binary Search Tree implementation with Rc<RefCell<T>>.
//!
//! This module demonstrates:
//! - **Generic constraints**: `T: Ord` for ordered comparisons
//! - **Rc<RefCell<T>>**: Shared ownership with interior mutability
//! - **Borrowing patterns**: Why search takes `&T` not `T`
//! - **RefCell runtime borrow checking**: `.borrow()` and `.borrow_mut()`
//!
//! ## Why Rc<RefCell<T>> for BST?
//!
//! While a simple BST could use `Box<T>`, `Rc<RefCell<T>>` is used here to teach:
//! - Shared ownership patterns needed for balanced trees (AVL, Red-Black)
//! - Interior mutability for mutations through shared references
//! - Runtime borrow checking and its trade-offs
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

use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::common::traits::Tree;

pub struct Node<T: Ord> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

pub struct BST<T: Ord> {
    root: Option<Rc<RefCell<Node<T>>>>,
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
