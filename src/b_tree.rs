// Rules
// 1. Every node has at most m children
//All leaf nodes of a B tree are at the same level, i.e. they have the same depth (height of the tree).
//The keys of each node of a B tree (in case of multiple keys), should be stored in the ascending order.
//In a B tree, all non-leaf nodes (except root node) should have at least m/2 children.
//All nodes (except root node) should have at least m/2 - 1 keys.
//If the root node is a leaf node (only node in the tree), then it will have no children and will
//have at least one key. If the root node is a non-leaf node, then it will have at least 2 children and at least one key.
//A non-leaf node with n-1 key values should have n non NULL children.

//struct BTreeNode {
//    keys: Vec<i32>,
//    children: Vec<Node>,
//}
//
// M = order = max children per node (Knuth definition, matches the rules above).
// max children = M, max keys = M-1, min keys = M/2-1, min children = M/2.
const M: usize = 5; // order: max children = 6, max keys = 5, min keys = 2
//
#[derive(Debug)]
struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<Box<BTreeNode>>,
    is_leaf: bool,
}


impl BTreeNode {
    fn new_leaf() -> Box<Self> {
        Box::new(Self {
            keys: Vec::new(),
            children: Vec::new(),
            is_leaf: true,
        })
    }
    fn new_internal() -> Box<Self> {
        Box::new(Self {
            keys: Vec::new(),
            children: Vec::new(),
            is_leaf: false,
        })
    }
    fn is_full(&self) -> bool {
        self.keys.len() == M - 1 // M-1 = max keys
    }
}

#[derive(Debug)]
struct BTree {
    root: Box<BTreeNode>,
}

impl BTree {
    fn new() -> Self {
        Self {
            root: BTreeNode::new_leaf(),
        }
    }

    fn search(&self, key: i32) -> bool {
        Self::search_node(&self.root, key)
    }

    fn search_node(node: &BTreeNode, key: i32) -> bool {
        let i = node.keys.partition_point(|&k| k < key);
        if i < node.keys.len() && node.keys[i] == key {
            return true;
        }
        if node.is_leaf {
            return false;
        }
        Self::search_node(&node.children[i], key)
    }

    // btree_insert: if movedown signals tree grew, create a new root.
    fn insert(&mut self, key: i32) {
        if let Some((pm, pmr)) = Self::movedown(key, &mut self.root) {
            let old_root = std::mem::replace(&mut self.root, BTreeNode::new_internal());
            self.root.keys.push(pm);
            self.root.children.push(old_root);
            self.root.children.push(pmr);
        }
    }

    // movedown: recurse to leaf; on the way back up, insert promoted key into current node
    // or split if full. Returns Some((pm, pmr)) if this node split and parent must handle it.
    // pmr_in is Option because at the leaf level there is no right child to pass up.
    fn movedown(k: i32, node: &mut BTreeNode) -> Option<(i32, Box<BTreeNode>)> {
        let b = node.keys.partition_point(|&key| key < k);

        // Key already exists — do nothing (nodesearch found it).
        if b < node.keys.len() && node.keys[b] == k {
            return None;
        }

        // Recurse into child, or signal "insert me" if we are at the leaf boundary (ptr == NULL in C).
        // pmr is Option: None when propagating from a leaf (no right child yet).
        let result: Option<(i32, Option<Box<BTreeNode>>)> = if node.is_leaf {
            Some((k, None)) // equivalent to ptr->bough[b] == NULL
        } else {
            Self::movedown(k, &mut node.children[b])
                .map(|(pm, pmr)| (pm, Some(pmr)))
        };

        match result {
            None => None,
            Some((pm, pmr)) => {
                if node.keys.len() < M - 1 {
                    // ptr->n < ATMOST: just insert
                    Self::putkey(pm, pmr, node, b);
                    None
                } else {
                    // ptr->n == ATMOST: split
                    let (new_pm, right) = Self::split(pm, pmr, node, b);
                    Some((new_pm, right))
                }
            }
        }
    }

    // putkey: insert key k with right-child pmr at position b in node.
    // Mirrors C's putkey(k, pmr, ptr, pos).
    fn putkey(k: i32, pmr: Option<Box<BTreeNode>>, node: &mut BTreeNode, b: usize) {
        node.keys.insert(b, k);
        if let Some(right) = pmr {
            node.children.insert(b + 1, right);
        }
    }

    // split: node is full (M keys). Insert k (with right-child pmr) using adaptive mid,
    // then promote the last key of the left half to the caller.
    // Mirrors C's split(k, pmr, ptr, pos, m, qmr).
    fn split(k: i32, pmr: Option<Box<BTreeNode>>, node: &mut BTreeNode, b: usize) -> (i32, Box<BTreeNode>) {
        let atleast = M.div_ceil(2) - 1; // min keys = M/2-1 (matches the rules: m/2 - 1 keys)
        let mid = if b <= atleast { atleast } else { atleast + 1 };

        let mut right = if node.is_leaf { BTreeNode::new_leaf() } else { BTreeNode::new_internal() };

        // right gets keys[mid..] (C: qmr->key[1..] = ptr->key[mid+1..ATMOST])
        right.keys = node.keys.split_off(mid);
        // For internal nodes, right gets children[mid+1..] (C: qmr->bough[1..] = ptr->bough[mid+1..ATMOST])
        if !node.is_leaf {
            right.children = node.children.split_off(mid + 1);
        }

        // Insert k into the correct half (C: putkey into ptr or qmr depending on pos <= ATLEAST)
        if b <= atleast {
            node.keys.insert(b, k); // insert into left
            if let Some(r) = pmr { node.children.insert(b + 1, r); }
        } else {
            right.keys.insert(b - mid, k); // insert into right (C: pos - mid)
            if let Some(r) = pmr { right.children.insert(b - mid, r); }
        }

        // *m = ptr->key[ptr->n]: last key of left is promoted
        let promoted = node.keys.pop().unwrap();
        // qmr->bough[0] = ptr->bough[ptr->n]: last child of left becomes first child of right
        if !node.is_leaf {
            right.children.insert(0, node.children.pop().unwrap());
        }

        (promoted, right)
    }

    fn to_sorted_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        Self::collect(&self.root, &mut result);
        result.sort();
        result
    }

    fn collect(node: &BTreeNode, result: &mut Vec<i32>) {
        result.extend_from_slice(&node.keys);
        for child in &node.children {
            Self::collect(child, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        //282 314 307 289 393 299 337 407 354 302 462 347 448 482 293 399 418 468 471 436
        let items = vec![
            282, 314, 307, 289, 393, 299, 337, 407, 354, 302, 462, 347, 448, 482, 293, 399, 418,
            468, 471, 436,
        ];
        let mut btree = BTree::new();
        for item in items {
            btree.insert(item);
        }
        println!("{:#?}", btree);
    }
}
