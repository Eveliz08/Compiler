//! Defines the `Visitor` trait for implementing the Visitor Pattern over the AST.
//!
//! The `Visitor<T>` trait allows different operations (e.g., code generation, pretty-printing,
//! semantic checking) to be performed on AST nodes by implementing a specific `visit_*` method
//! for each node type.

use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::for_loop::ForNode;
use crate::ast_nodes::function_call::FunctionCallNode;
use crate::ast_nodes::print::PrintNode;
use crate::ast_nodes::type_def::TypeDefNode;
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::if_else::IfElseNode;
use crate::ast_nodes::literals::{NumberLiteralNode, BooleanLiteralNode, StringLiteralNode, IdentifierNode};
use crate::ast_nodes::while_loop::WhileNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::function_def::FunctionDefNode;
use crate::ast_nodes::type_instance::TypeInstanceNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};

/// The core trait for visiting AST nodes.
///
/// Implementations of this trait define how to process or transform different node types in the AST.
///
/// # Type Parameters
/// - `T`: The result type returned from visiting each node (e.g., `String` for printers,
///   `Result<(), Error>` for analyzers, etc.).
pub trait Visitor<T> {
    /// Visit a function definition node.
    fn visit_function_def(&mut self, node: &mut FunctionDefNode) -> T;

    /// Visit a numeric literal node.
    fn visit_literal_number(&mut self, node: &mut NumberLiteralNode) -> T;

    /// Visit a boolean literal node.
    fn visit_literal_boolean(&mut self, node: &mut BooleanLiteralNode) -> T;

    /// Visit a string literal node.
    fn visit_literal_string(&mut self, node: &mut StringLiteralNode) -> T;

    /// Visit an identifier node.
    fn visit_identifier(&mut self, node: &mut IdentifierNode) -> T;

    /// Visit a function call node.
    fn visit_function_call(&mut self, node: &mut FunctionCallNode) -> T;

    /// Visit a `while` loop node.
    fn visit_while_loop(&mut self, node: &mut WhileNode) -> T;

    /// Visit a `for` loop node.
    fn visit_for_loop(&mut self, node: &mut ForNode) -> T;

    /// Visit a code block node.
    fn visit_code_block(&mut self, node: &mut BlockNode) -> T;

    /// Visit a binary operation node.
    fn visit_binary_op(&mut self, node: &mut BinaryOpNode) -> T;

    /// Visit a unary operation node.
    fn visit_unary_op(&mut self, node: &mut UnaryOpNode) -> T;

    /// Visit an `if-else` expression node.
    fn visit_if_else(&mut self, node: &mut IfElseNode) -> T;

    /// Visit a `let-in` expression node.
    fn visit_let_in(&mut self, node: &mut LetInNode) -> T;

    /// Visit a destructive assignment (`:=`) node.
    fn visit_destructive_assign(&mut self, node: &mut DestructiveAssignNode) -> T;

    /// Visit a type definition node.
    fn visit_type_def(&mut self, node: &mut TypeDefNode) -> T;

    /// Visit a type instantiation (`new Type(...)`) node.
    fn visit_type_instance(&mut self, node: &mut TypeInstanceNode) -> T;

    /// Visit a method call on a type instance (e.g., `obj.method()`).
    fn visit_type_function_access(&mut self, node: &mut TypeFunctionAccessNode) -> T;

    /// Visit a property access on a type instance (e.g., `obj.property`).
    fn visit_type_prop_access(&mut self, node: &mut TypePropAccessNode) -> T;

    /// Visit a `print(...)` expression node.
    fn visit_print(&mut self, node: &mut PrintNode) -> T;
}
