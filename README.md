# DSTree - Learn Rust Through Tree Data Structures

A comprehensive Rust library for learning advanced Rust concepts through implementing various tree data structures. 
This project combines algorithm education with deep dives into Rust's ownership system, smart pointers, concurrency, and performance optimization.

## Overview

DSTree is an educational project that teaches Rust concepts progressively through implementing tree data structures. 
Each tree implementation introduces new Rust patterns and best practices, building from simple to complex:

- **Binary Tree**: Master `Box<T>` and recursive types
- **Binary Search Tree (BST)**: Learn generic constraints and borrowing patterns
- **AVL Tree**: Understand `Rc<RefCell<T>>` and interior mutability
- **Concurrent AVL**: Master `Arc<RwLock<T>>` and thread-safe programming
- **And more**: Red-Black Trees, B-Trees, Tries, Segment Trees

## Learning Objectives

Through this project, you will master:

### Ownership & Smart Pointers
- `Box<T>` for heap allocation and recursive types
- `Rc<T>` and `Arc<T>` for shared ownership
- `Weak<T>` for breaking reference cycles
- When to use each smart pointer type

### Interior Mutability
- `RefCell<T>` for runtime borrow checking
- `RwLock<T>` for concurrent access patterns
- Trade-offs between compile-time and runtime checks

### Generic Programming
- Type constraints (`T: Ord`, `T: Send + Sync`)
- Trait bounds and where clauses
- Zero-cost abstractions

### Concurrent Programming
- Thread safety with `Arc` and locks
- `Send` and `Sync` marker traits
- Deadlock prevention strategies
- Testing concurrent code with `loom`

### Performance Engineering
- Benchmarking with `criterion`
- Memory profiling
- Cache-aware data structures
- Property-based testing with `proptest`

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

## Getting Started

### Prerequisites

- Rust 1.70+ (uses 2024 edition)
- Cargo (comes with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/matheusfrancisco/dstree.git
cd dstree

# Build the project
cargo build

# Run tests
cargo test

# Generate and open documentation
cargo doc --open
```

### Quick Example

```rust
use dstree::bst::BST;
use dstree::common::traits::OrderedTree;

fn main() {
    let mut tree = BST::new();

    // Insert values
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    tree.insert(1).unwrap();
    tree.insert(9).unwrap();

    // Search for values
    assert!(tree.contains(&5));
    assert!(tree.contains(&1));
    assert!(!tree.contains(&10));

    // Find min and max
    assert_eq!(tree.min(), Some(&1));
    assert_eq!(tree.max(), Some(&9));

    println!("Tree has {} nodes", tree.len());
}
```

## Implementation Progress

- [x] Phase 1: Project Setup & Common Infrastructure
- [x] Phase 2: Binary Tree (Box<T>)
- [ ] Phase 3: Binary Search Tree (Generic Constraints)
- [ ] Phase 4: AVL Tree - Single-threaded (Rc<RefCell<T>>)
- [ ] Phase 5: AVL Tree - Concurrent (Arc<RwLock<T>>)
- [ ] Phase 6: Performance Optimization & Benchmarking
- [ ] Phase 7: Advanced Features (Parent Pointers, Iterators, CLI)

## Features

- **Comprehensive Documentation**: Every module includes detailed explanations of Rust concepts
- **Property-Based Testing**: Uses `proptest` to verify tree invariants
- **Concurrent Testing**: Uses `loom` to detect race conditions
- **Performance Benchmarking**: Uses `criterion` for statistical analysis
- **Memory Safety**: Validated with Miri for undefined behavior detection
- **CLI Tool**: Interactive visualization and testing (coming soon)

## Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test binary
cargo test bst

# Run with Miri for memory safety checking
cargo +nightly miri test

# Run property-based tests
cargo test --test property_tests

# Run concurrent tests with loom
RUSTFLAGS="--cfg loom" cargo test --test concurrent_tests
```

## Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench insertion

# Generate HTML reports (in target/criterion/)
cargo bench --bench insertion
```

## Documentation

Comprehensive documentation is available:

```bash
# Generate and open documentation
cargo doc --open
```

Each module includes:
- Detailed explanations of Rust concepts being taught
- Code examples with explanations
- Links to related rust-skills learning modules
- Performance characteristics (Big-O notation)

## Learning Path

This project is designed to be learned in order:

1. **Start with Binary Tree**: Understand `Box<T>` and recursive structures
2. **Move to BST**: Learn generic constraints and borrowing patterns
3. **Progress to AVL**: Master `Rc<RefCell<T>>` and interior mutability
4. **Explore Concurrency**: Implement thread-safe trees with `Arc<RwLock<T>>`
5. **Optimize Performance**: Benchmark and profile implementations
6. **Advanced Topics**: Parent pointers with `Weak<T>`, custom iterators

Each phase builds on the previous, introducing new concepts incrementally.

## Contributing

This is a personal learning project. Feel free to:
- Fork and create your own implementations
- Open issues for questions or discussions
- Share your learning insights

## License

MIT OR Apache-2.0


**Status**: Phase 1 Complete - Foundation & Common Infrastructure ✓
