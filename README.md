# DSTree - Learn Rust Through Tree Data Structures

This is the place to keep learning the basics of rust and data structures.

## Project Structure

```
dstree/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # CLI tool for visualization
│   ├── common/             # Shared traits and utilities
│   │   ├── traits.rs       # Core tree traits
│   │   ├── error.rs        # Error types
│   │   └── visualize.rs    # Visualization helpers
│   ├── binary/             # Basic binary tree (Box<T>)
│   ├── bst/                # Binary search tree
│   └── avl/                # AVL tree (single & concurrent)
├── benches/                # Performance benchmarks
├── tests/                  # Integration tests
└── docs/                   # Learning guides
```


## Implementation Progress

- [x] Phase 1: Project Setup & Common Infrastructure
- [x] Phase 2: Binary Tree (Box<T>)
- [x] Phase 3: Binary Search Tree (Generic Constraints)
- [x] Phase 4: AVL Tree - Single-threaded 
- [ ] Phase 5: AVL Tree - Concurrent (Arc<RwLock<T>>)
- [ ] Phase 6: B-Tree (Generic Constraints)

