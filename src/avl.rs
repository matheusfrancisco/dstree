use crate::Tree;

#[derive(Debug)]
struct AvlNode<T: Ord> {
    value: T,
    height: usize,
    left: Option<Box<AvlNode<T>>>,
    right: Option<Box<AvlNode<T>>>,
}

#[derive(Debug)]
struct AvlTree<T: Ord> {
    root: Option<Box<AvlNode<T>>>,
    len: usize,
    height: usize,
}

impl<T: Ord> AvlNode<T> {
    fn new(value: T) -> Self {
        AvlNode {
            value,
            height: 0,
            left: None,
            right: None,
        }
    }

    fn new_empty() -> Self
    where
        T: Default,
    {
        AvlNode {
            value: T::default(),
            height: 0,
            left: None,
            right: None,
        }
    }
}

impl<T: Ord> Tree<T> for AvlTree<T> {
    fn len(&self) -> usize {
        self.len
    }

    fn height(&self) -> usize {
        self.height
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn clear(&mut self) {
        self.root = None;
        self.len = 0;
        self.height = 0;
    }
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl<T: Ord + std::fmt::Debug + Clone> AvlTree<T> {
    fn new() -> Self {
        AvlTree {
            root: None,
            len: 0,
            height: 0,
        }
    }

    fn height_of(node: &Option<Box<AvlNode<T>>>) -> isize {
        node.as_ref().map_or(-1, |n| n.height as isize)
    }

    fn update_node_height(node: &mut AvlNode<T>) {
        node.height =
            (1 + std::cmp::max(Self::height_of(&node.left), Self::height_of(&node.right))) as usize;
    }

    fn update_height(node: &mut Option<Box<AvlNode<T>>>) {
        if let Some(n) = node {
            Self::update_node_height(n);
        }
    }

    fn insert(&mut self, value: T) -> bool {
        let inserted = Self::insert_recursive(&mut self.root, value.clone());
        if inserted {
            self.len += 1;
        }

        self.height = Self::height_of(&self.root) as usize;
        inserted
    }

    fn insert_recursive(tree: &mut Option<Box<AvlNode<T>>>, value: T) -> bool {
        if let Some(node) = tree {
            let inserted = match value.cmp(&node.value) {
                std::cmp::Ordering::Less => Self::insert_recursive(&mut node.left, value),
                std::cmp::Ordering::Greater => Self::insert_recursive(&mut node.right, value),
                std::cmp::Ordering::Equal => return false,
            };
            if inserted {
                Self::rebalance(tree);
            }
            return inserted;
        }
        *tree = Some(Box::new(AvlNode::new(value)));
        true
    }
    //to complexity and with clone need to fix it
    fn rebalance(tree: &mut Option<Box<AvlNode<T>>>) {
        let mut node = *tree.take().unwrap();
        Self::update_node_height(&mut node);
        let bf = Self::balance_factor(&node);
        match bf {
            2 => {
                // Left heavy
                if Self::balance_factor(node.left.as_ref().unwrap()) < 0 {
                    //LR case
                    let left_child = *node.left.take().unwrap();
                    node.left = Some(Self::rotate_left(left_child));
                    Self::update_node_height(&mut node);
                }
                //LL Case
                *tree = Some(Self::rotate_right(node));
            }
            -2 => {
                // Right heavy
                if Self::balance_factor(node.right.as_ref().unwrap()) > 0 {
                    //RL case
                    let right_child = *node.right.take().unwrap();
                    node.right = Some(Self::rotate_right(right_child));
                    Self::update_node_height(&mut node);
                }
                //RR Case
                *tree = Some(Self::rotate_left(node));
            }
            _ => *tree = Some(Box::new(node)),
        }
    }

    fn balance_factor(node: &AvlNode<T>) -> isize {
        Self::height_of(&node.left) as isize - Self::height_of(&node.right) as isize
    }

    fn rotate_right(mut node: AvlNode<T>) -> Box<AvlNode<T>> {
        //      node              new_root
        //      /  \                /    \
        // new_root  C    →        A     node
        //   /  \                        /  \
        //  A    B                      B    C

        let mut new_root = node.left.take().unwrap(); // Take LEFT
        node.left = new_root.right.take(); // B goes to node.left
        Self::update_node_height(&mut node); // Update node first
        new_root.right = Some(Box::new(node)); // node becomes right child
        Self::update_node_height(&mut new_root);
        new_root
    }

    fn rotate_left(mut node: AvlNode<T>) -> Box<AvlNode<T>> {
        //   node                  new_root
        //   /  \                   /    \
        //  A   new_root   →      node    C
        //        /  \            /  \
        //       B    C          A    B

        let mut new_root = node.right.take().unwrap(); // Take RIGHT
        node.right = new_root.left.take(); // B goes to node.right
        Self::update_node_height(&mut node); // Update node first
        new_root.left = Some(Box::new(node)); // node becomes left child
        Self::update_node_height(&mut new_root);
        new_root
    }

    // not so efficient way to insert a value, we are just
    // traversing the tree to find the correct position for the new value and then we are updating the height of the nodes on the path back to the root
    fn insert_iter(&mut self, value: T) -> bool {
        if self.root.is_none() {
            self.root = Some(Box::new(AvlNode::new(value)));
            self.len += 1;
            return true;
        }

        let mut curr = &mut self.root;
        // we need some way to keep the path remember

        let mut path: Vec<Direction> = Vec::new();

        //basically we are traversing the tree to find the correct position for the new value
        while let Some(node) = curr {
            match value.cmp(&node.value) {
                std::cmp::Ordering::Less => {
                    path.push(Direction::Left);
                    curr = &mut node.left;
                }
                std::cmp::Ordering::Greater => {
                    path.push(Direction::Right);
                    curr = &mut node.right;
                }
                std::cmp::Ordering::Equal => return false,
            }
        }

        // insert the new node
        *curr = Some(Box::new(AvlNode::new(value)));

        // not efficiently updating the height of the nodes, we are just traversing back up the path and updating the height of each node
        // this complexity is O(n*m)
        for len in (0..path.len()).rev() {
            let curr = Self::mut_at_path(&mut self.root, &path[0..len]);
            Self::update_height(curr);
            Self::rebalance(curr);
        }

        self.len += 1;
        self.height = Self::height_of(&self.root) as usize;
        true
    }

    fn mut_at_path<'a>(
        root: &'a mut Option<Box<AvlNode<T>>>,
        path: &'a [Direction],
    ) -> &'a mut Option<Box<AvlNode<T>>> {
        let mut curr = root;
        for step in path {
            curr = match step {
                Direction::Left => &mut curr.as_mut().unwrap().left,
                Direction::Right => &mut curr.as_mut().unwrap().right,
            };
        }
        curr
    }

    fn contains(&self, value: &T) -> bool {
        let mut curr = &self.root;

        while let Some(node) = curr {
            match value.cmp(&node.value) {
                std::cmp::Ordering::Less => curr = &node.left,
                std::cmp::Ordering::Greater => curr = &node.right,
                std::cmp::Ordering::Equal => return true,
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{Tree, avl::AvlTree};

    use super::AvlNode;

    #[test]
    fn test_avl_node_creation() {
        let node = AvlNode {
            value: 10,
            height: 1,
            left: None,
            right: None,
        };
        assert_eq!(node.value, 10);
        assert_eq!(node.height, 1);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_avl_node_new() {
        let node = AvlNode::new(20);
        assert_eq!(node.value, 20);
        assert_eq!(node.height, 0);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_insert_() {
        let mut tree = AvlTree::new();
        let items = vec![10, 20, 5, 15, 25, 10, 34];
        for item in items {
            tree.insert_iter(item);
        }

        assert_eq!(tree.len(), 6);

        assert_eq!(tree.height(), 2);
        assert!(tree.contains(&10));
        assert!(tree.contains(&20));
        assert!(tree.contains(&5));
        assert!(tree.contains(&15));
        assert!(tree.contains(&25));
        assert!(!tree.contains(&30));
        println!("{:#?}", tree);
    }

    #[test]
    fn test_insert_recursive() {
        let mut tree = AvlTree::new();
        let items = vec![10, 20, 5, 15, 25, 10, 34];
        for item in items {
            tree.insert(item);
        }

        assert_eq!(tree.len(), 6);

        assert_eq!(tree.height(), 2);
        assert!(tree.contains(&10));
        assert!(tree.contains(&20));
        assert!(tree.contains(&5));
        assert!(tree.contains(&15));
        assert!(tree.contains(&25));
        assert!(!tree.contains(&30));
    }

    #[test]
    fn test_rebalance_rr() {
        let mut tree = AvlTree::new();
        let items = vec![30, 20, 10];
        for item in items {
            tree.insert(item);
        }

        assert_eq!(tree.len(), 3);
        assert_eq!(tree.root.as_ref().unwrap().value, 20);
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.left.as_ref().unwrap().value, 10);
        assert_eq!(root.right.as_ref().unwrap().value, 30);
    }

    #[test]
    fn test_rebalance_lr() {
        let mut tree = AvlTree::new();
        let items = vec![30, 10, 20];
        for item in items {
            tree.insert(item);
        }

        assert_eq!(tree.len(), 3);
        assert_eq!(tree.root.as_ref().unwrap().value, 20);
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.left.as_ref().unwrap().value, 10);
        assert_eq!(root.right.as_ref().unwrap().value, 30);
    }
}
