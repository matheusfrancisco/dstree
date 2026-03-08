#[derive(Debug)]
struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<Option<Box<BTreeNode>>>,
    is_leaf: bool,
}

impl BTreeNode {
    fn new_leaf() -> Self {
        BTreeNode {
            keys: vec![],
            children: vec![],
            is_leaf: true,
        }
    }

    fn new_internal() -> Self {
        BTreeNode {
            keys: vec![],
            children: vec![],
            is_leaf: false,
        }
    }

    fn find_pos(&self, key: i32) -> usize {
        //self.keys.partition_point(|k| *k < key)
        // Binary search to find the first position where key could be inserted
        let mut left = 0;
        let mut right = self.keys.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if self.keys[mid] < key {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    fn is_full(&self, order: usize) -> bool {
        self.keys.len() >= order - 1
    }
}

#[derive(Debug)]
struct BTree {
    root: Option<Box<BTreeNode>>,
    order: usize,
}

// insert 10, 20B-Trees¶, 30, 40, 50, 69, 70
impl BTree {
    fn new(order: usize) -> Self {
        BTree { root: None, order }
    }

    fn kmin(&self) -> usize {
        // ⌈order/2⌉ - 1
        ((self.order + 1) / 2) - 1
    }
    fn kmax(&self) -> usize {
        self.order - 1
    }
    fn is_full(&self) -> bool {
        // A node is full when it has kmax = order - 1 keys
        self.root
            .as_ref()
            .is_some_and(|node| node.keys.len() >= self.kmax())
    }

    fn insert(&mut self, key: i32) {
        // empty tree
        if self.root.is_none() {
            self.root = Some(Box::new(BTreeNode {
                keys: vec![key],
                children: vec![],
                is_leaf: true,
            }));
            return;
        }
        // Case 2: Root is full — split it BEFORE inserting
        // This is the ONLY place the tree grows taller.
        //
        // We create a new empty internal root, make the old root its
        // only child, then split that child. Now the new root has
        // 1 key and 2 children, and we can descend safely.
        if self.is_full() {
            let old_root = self.root.take().unwrap();
            // old root is moved out self.root is now none temporarily not good for concurrent
            let mut new_root = Box::new(BTreeNode::new_internal());
            new_root.children.push(Some(old_root));
            Self::split_child(&mut new_root, 0, self.order);
            self.root = Some(new_root);
        }
        // Case 3: Root is guaranteed non-full, descend
        // INVARIANT: root.keys.len() < kmax when we reach here
        let root = self.root.as_mut().unwrap();
        Self::insert_non_full(root, key, self.order);
    }
    /// Insert `key` into `node` which is guaranteed to be NON-FULL.
    ///
    /// PRECONDITION: node.keys.len() < kmax
    ///   (ensured by caller: either root was checked, or we split the child
    ///    before descending into it)
    ///
    /// POSTCONDITION: key is in the subtree rooted at node.
    ///   node.keys.len() <= kmax (no overflow because we had room)
    fn insert_non_full(node: &mut BTreeNode, key: i32, order: usize) {
        debug_assert!(
            node.keys.len() < order - 1 || node.is_leaf,
            "insert_non_full called on full non-leaf node"
        );
        let pos = node.find_pos(key);
        // ── Duplicate check ──
        if pos < node.keys.len() && node.keys[pos] == key {
            return; // Key already exists, do nothing
        }
        if node.is_leaf {
            // ── Leaf: insert directly ──
            // SAFE: node has < kmax keys (or == kmax only on first insert
            // to a leaf root — wait, no. If root is a leaf AND full,
            // we would have split it in `insert()`. So this is safe.
            //
            // Actually for leaf: insert can make it have kmax keys,
            // which is fine (kmax is the MAX, not overflow).
            // A node overflows at kmax+1 = order keys.
            // insert_non_full is called when keys.len() < kmax,
            // so after insert: keys.len() <= kmax. ✓
            node.keys.insert(pos, key);
        } else {
            // ── Internal: may need to split child before descending ──
            //
            // Check if children[pos] is full. If so, split it first.
            // This guarantees that when we recurse, the child is non-full.
            let mut target = pos;
            let child_is_full = node.children[target]
                .as_ref()
                .expect("internal node must have child")
                .is_full(order);
            if child_is_full {
                // Split the full child. This:
                //   - Adds 1 key (median) at node.keys[target]
                //   - Adds 1 child (right half) at node.children[target+1]
                //   - Shrinks children[target] to left half
                //
                // SAFE: node has < kmax keys, so it has room for the median.
                Self::split_child(node, target, order);
                // After split, node.keys[target] is the promoted median.
                // Decide which child to descend into:
                //   - If key > median: go right (target + 1)
                //   - If key == median: duplicate, return
                //   - If key < median: go left (target unchanged)
                if key == node.keys[target] {
                    return; // The promoted median IS our key — already inserted!
                }
                if key > node.keys[target] {
                    target += 1;
                }
            }
            // Recurse into the (guaranteed non-full) child
            let child = node.children[target]
                .as_mut()
                .expect("internal node must have child");
            Self::insert_non_full(child, key, order);
        }
    }

    /// Split `parent.children[index]` which MUST be full.
    /// Before:
    ///  parent.children[index] has kmax(full)
    ///  parent has <  kmax keys (has room)
    ///
    /// After:
    ///   parent gains 1 key (the median) at position `index`
    ///   parent gains 1 child (new right node) at position `index + 1`
    ///   parent.children[index] shrinks to left half
    ///
    /// Ownership: We take &mut parent, reach into children[index]
    /// via &mut, split it, then modify parent's keys/children.
    /// The borrow of the child is scoped within the block.
    fn split_child(parent: &mut BTreeNode, index: usize, order: usize) {
        // We need to:
        //   1. Read/modify children[index] (the full child)
        //   2. Modify parent.keys and parent.children
        //
        // To satisfy the borrow checker, we extract what we need
        // from the child first, then modify the parent.
        let child = parent.children[index]
            .as_mut()
            .expect("split_child: child must exist");

        let kmax = order - 1;
        debug_assert_eq!(child.keys.len(), kmax, "split_child: child must be full");
        // mid = kmax - 1 / 2
        //   order=4: kmax=3, mid=1 → left=[k0], median=k1, right=[k2]
        //   order=5: kmax=4, mid=2 → left=[k0,k1], median=k2, right=[k3]
        //   order=6: kmax=5, mid=2 → left=[k0,k1], median=k2, right=[k3,k4]
        let mid = kmax / 2;
        // Step 1: Split off the right portion of keys
        //   child.keys = [k0, ..., k_{mid}, k_{mid+1}, ..., k_{kmax-1}]
        //   After split_off(mid+1): right_keys = [k_{mid+1}, ...]
        //   child.keys = [k0, ..., k_{mid}]
        let right_keys = child.keys.split_off(mid + 1);
        // Step 2: Pop the median from the child
        //   child.keys was [k0, ..., k_{mid}], pop gives median = k_{mid}
        //   child.keys is now [k0, ..., k_{mid-1}] (the left half)
        let median = child.keys.pop().unwrap();
        // Step 3: Split children if internal
        //   A full internal node has kmax+1 = order children
        //   Left keeps children[0..=mid], right gets children[mid+1..]
        let right_children = if !child.is_leaf {
            child.children.split_off(mid + 1)
        } else {
            Vec::new()
        };
        let right_node = Box::new(BTreeNode {
            keys: right_keys,
            children: right_children,
            is_leaf: child.is_leaf,
        });
        // ← child borrow ends here (we're done touching parent.children[index])
        // Step 4: Insert median and right child into parent
        //   INV-SORTED: median sits between all keys in left and right
        //   INV-CHILDREN: parent gains +1 key and +1 child → balanced
        parent.keys.insert(index, median);
        parent.children.insert(index + 1, Some(right_node));
    }

    fn binary_search_pos(&self, keys: &[i32], key: i32) -> usize {
        let mut left = 0;
        let mut right = keys.len();
        while left < right {
            let mid = left + (right - left) / 2;
            if keys[mid] < key {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_btree_insert() {
    //     let mut btree = BTree::new(5);
    //     btree.insert(10);
    //     btree.insert(20);
    //     btree.insert(30);
    //     btree.insert(40);
    //     btree.insert(50);
    //     btree.insert(69);
    //     btree.insert(70);
    //     btree.insert(71);
    //     btree.insert(1);
    //     btree.insert(5);
    //     btree.insert(2);

    //     println!("{:#?}", btree);
    //     // Add assertions to verify the structure of the B-tree
    //     btree.insert(15);
    //     btree.insert(25);
    //     println!("{:#?}", btree);
    //     btree.insert(26);
    //     btree.insert(27);
    //     println!("{:#?}", btree);
    //     btree.insert(32);
    //     println!("{:#?}", btree);
    //     btree.insert(33);
    //     btree.insert(34);
    //     btree.insert(35);
    //     btree.insert(36);
    //     println!("{:#?}", btree);
    // }

    #[test]
    fn test_insert() {
        //282 314 307 289 393 299 337 407 354 302 462 347 448 482 293 399 418 468 471 436
        let items = vec![
            282, 314, 307, 289, 393, 299, 337, 407, 354, 302, 462, 347, 448, 482, 293, 399, 418,
            468, 471, 436,
        ];
        let mut btree = BTree::new(5);
        for item in items {
            btree.insert(item);
        }
        println!("{:#?}", btree);
    }
}
