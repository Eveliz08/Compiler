//! Defines the `Accept` trait for AST nodes to support the Visitor pattern.

use super::visitor_trait::Visitor;

/// Trait implemented by AST nodes that can be visited by a [`Visitor`].
///
/// This enables external operations (like code generation or semantic analysis)
/// to be performed on the AST without embedding logic inside the nodes themselves.
///
/// # Type Parameters
/// - `V`: A concrete type implementing the [`Visitor`] trait.
/// - `T`: The return type produced by the visitor operation.
///

pub trait Accept {
    /// Accepts a mutable visitor, allowing it to operate on the implementing node.
    ///
    /// # Arguments
    /// * `visitor` - A mutable reference to the visitor instance.
    ///
    /// # Returns
    /// A value of type `T` as determined by the visitor.
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T;
}
