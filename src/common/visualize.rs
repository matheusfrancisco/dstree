//! Tree visualization utilities.
//!
//! This module provides helper functions and traits for visualizing tree structures
//! in ASCII art format, useful for debugging and educational purposes.
//!
//! # Future Enhancements
//!
//! - ASCII art tree rendering
//! - DOT format export for Graphviz
//! - JSON serialization for web visualization
//! - Interactive CLI visualization

use colored::Colorize;
use std::fmt;

/// Trait for types that can be visualized as trees.
///
/// Implement this trait to provide custom visualization for your tree structure.
///
/// # Examples
///
/// ```rust,ignore
/// use dstree::common::visualize::Visualize;
///
/// impl Visualize for MyTree {
///     fn visualize(&self) -> String {
///         // Custom visualization logic
///         format!("Tree with {} nodes", self.len())
///     }
/// }
/// ```
pub trait Visualize {
    /// Returns a string representation of the tree structure.
    ///
    /// The format should be human-readable and suitable for console output.
    fn visualize(&self) -> String;

    /// Returns a colored string representation of the tree structure.
    ///
    /// This method uses the `colored` crate to add ANSI color codes
    /// for better visual distinction of different tree elements.
    fn visualize_colored(&self) -> String {
        self.visualize() // Default implementation without colors
    }
}

/// Helper struct for building ASCII tree visualizations.
///
/// This builder assists in creating consistent tree visualizations
/// using box-drawing characters.
///
/// # Examples
///
/// ```rust
/// use dstree::common::visualize::TreeVisualBuilder;
///
/// let mut builder = TreeVisualBuilder::new();
/// let viz = builder
///     .add_line("Root: 5")
///     .add_line("├─ Left: 3")
///     .add_line("└─ Right: 7")
///     .build();
/// println!("{}", viz);
/// ```
#[derive(Debug, Clone)]
pub struct TreeVisualBuilder {
    lines: Vec<String>,
    indent_level: usize,
}

impl TreeVisualBuilder {
    /// Creates a new `TreeVisualBuilder`.
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            indent_level: 0,
        }
    }

    /// Adds a line to the visualization.
    pub fn add_line(&mut self, line: impl Into<String>) -> &mut Self {
        let indent = "  ".repeat(self.indent_level);
        self.lines.push(format!("{}{}", indent, line.into()));
        self
    }

    /// Adds a node with the specified value and optional children indicator.
    pub fn add_node<T: fmt::Display>(&mut self, value: &T, has_children: bool) -> &mut Self {
        let prefix = if has_children { "├─" } else { "└─" };
        self.add_line(format!("{} {}", prefix, value));
        self
    }

    /// Increases the indentation level for nested nodes.
    pub fn indent(&mut self) -> &mut Self {
        self.indent_level += 1;
        self
    }

    /// Decreases the indentation level.
    pub fn dedent(&mut self) -> &mut Self {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
        self
    }

    /// Builds the final visualization string.
    pub fn build(&self) -> String {
        self.lines.join("\n")
    }

    /// Builds a colored visualization string.
    pub fn build_colored(&self) -> String {
        self.lines
            .iter()
            .map(|line| {
                // Color different parts of the tree differently
                if line.contains("├─") || line.contains("└─") {
                    line.bright_blue().to_string()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for TreeVisualBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Formats a tree node value for display.
///
/// This function provides consistent formatting for node values
/// across different tree visualizations.
///
/// # Examples
///
/// ```rust
/// use dstree::common::visualize::format_node_value;
///
/// let formatted = format_node_value(&42);
/// assert_eq!(formatted, "42");
/// ```
pub fn format_node_value<T: fmt::Display>(value: &T) -> String {
    format!("{}", value)
}

/// Formats a tree node value with color.
///
/// # Examples
///
/// ```rust
/// use dstree::common::visualize::format_node_value_colored;
///
/// let formatted = format_node_value_colored(&42);
/// // Returns colored string representation
/// ```
pub fn format_node_value_colored<T: fmt::Display>(value: &T) -> String {
    format!("{}", value).green().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_visual_builder() {
        let mut builder = TreeVisualBuilder::new();
        let viz = builder
            .add_line("Root")
            .indent()
            .add_line("Child 1")
            .add_line("Child 2")
            .dedent()
            .build();

        assert!(viz.contains("Root"));
        assert!(viz.contains("Child 1"));
        assert!(viz.contains("Child 2"));
    }

    #[test]
    fn test_add_node() {
        let mut builder = TreeVisualBuilder::new();
        builder.add_node(&5, true);
        builder.add_node(&3, false);

        let viz = builder.build();
        assert!(viz.contains("├─ 5"));
        assert!(viz.contains("└─ 3"));
    }

    #[test]
    fn test_format_node_value() {
        assert_eq!(format_node_value(&42), "42");
        assert_eq!(format_node_value(&"hello"), "hello");
    }

    #[test]
    fn test_indent_dedent() {
        let mut builder = TreeVisualBuilder::new();
        assert_eq!(builder.indent_level, 0);

        builder.indent();
        assert_eq!(builder.indent_level, 1);

        builder.indent();
        assert_eq!(builder.indent_level, 2);

        builder.dedent();
        assert_eq!(builder.indent_level, 1);

        builder.dedent();
        assert_eq!(builder.indent_level, 0);

        // Should not go below 0
        builder.dedent();
        assert_eq!(builder.indent_level, 0);
    }
}
