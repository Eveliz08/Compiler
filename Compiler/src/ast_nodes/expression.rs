use super::binary_op::BinaryOpNode;
use super::block::{BlockNode, ExpressionList};
use super::destructive_assign::DestructiveAssignNode;
use super::for_loop::ForNode;
use super::function_call::FunctionCallNode;
use super::if_else::IfElseNode;
use super::let_in::{Assignment, LetInNode};
use super::literals::{BooleanLiteralNode, IdentifierNode, NumberLiteralNode, StringLiteralNode};
use super::unary_op::UnaryOpNode;
use super::while_loop::WhileNode;
use crate::ast_nodes::print::PrintNode;
use crate::ast_nodes::type_instance::TypeInstanceNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};
use crate::tokens::{OperatorToken, Span};
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(NumberLiteralNode),
    Boolean(BooleanLiteralNode),
    Str(StringLiteralNode),
    Identifier(IdentifierNode),
    FunctionCall(FunctionCallNode),
    WhileLoop(WhileNode),
    ForLoop(ForNode),
    CodeBlock(BlockNode),
    BinaryOp(BinaryOpNode),
    UnaryOp(UnaryOpNode),
    IfElse(IfElseNode),
    LetIn(LetInNode),
    DestructiveAssign(DestructiveAssignNode),
    TypeInstance(TypeInstanceNode),
    TypeFunctionAccess(TypeFunctionAccessNode),
    TypePropAccess(TypePropAccessNode),
    Print(PrintNode),
}

impl Expression {
    pub fn new_number(value: String, span: Span) -> Self {
        Expression::Number(NumberLiteralNode::new(&value, span))
    }

    pub fn new_boolean(value: bool, span: Span) -> Self {
        Expression::Boolean(BooleanLiteralNode::new(value, span))
    }

    pub fn new_string(value: String, span: Span) -> Self {
        Expression::Str(StringLiteralNode::new(&value, span))
    }

    pub fn new_identifier(value: String, span: Span) -> Self {
        Expression::Identifier(IdentifierNode::new(&value, span))
    }

    pub fn new_function_call(function: String, arguments: Vec<Expression>, span: Span) -> Self {
        Expression::FunctionCall(FunctionCallNode::new(function, arguments, span))
    }

    pub fn new_while_loop(condition: Expression, body: Expression, span: Span) -> Self {
        Expression::WhileLoop(WhileNode::new(condition, body, span))
    }

    pub fn new_for_loop(
        variable: String,
        start: Expression,
        end: Expression,
        body: Expression,
        span: Span,
    ) -> Self {
        Expression::ForLoop(ForNode::new(variable, start, end, body, span))
    }

    pub fn new_code_block(expression_list: ExpressionList) -> Self {
        Expression::CodeBlock(BlockNode::new(expression_list))
    }

    pub fn new_binary_op(
        left: Expression,
        operator: OperatorToken,
        right: Expression,
        span: Span,
    ) -> Self {
        Expression::BinaryOp(BinaryOpNode::new(left, operator, right, span))
    }

    pub fn new_unary_op(operator: OperatorToken, operand: Expression, span: Span) -> Self {
        Expression::UnaryOp(UnaryOpNode::new(operator, operand, span))
    }

    pub fn new_if_else(
        condition: Expression,
        if_expression: Expression,
        elifs: Vec<(Option<Expression>, Expression)>,
        span: Span,
    ) -> Self {
        Expression::IfElse(IfElseNode::new(condition, if_expression, elifs, span))
    }

    pub fn new_let_in(assignments: Vec<Assignment>, body: Expression, span: Span) -> Self {
        Expression::LetIn(LetInNode::new(assignments, body, span))
    }

    pub fn new_destructive_assign(identifier: Expression, expr: Expression, span: Span) -> Self {
        Expression::DestructiveAssign(DestructiveAssignNode::new(identifier, expr, span))
    }

    pub fn new_type_instance(type_name: String, type_args: Vec<Expression>, span: Span) -> Self {
        Expression::TypeInstance(TypeInstanceNode::new(type_name, type_args, span))
    }

    pub fn new_type_function_access(
        object: Expression,
        member: FunctionCallNode,
        span: Span,
    ) -> Self {
        Expression::TypeFunctionAccess(TypeFunctionAccessNode::new(object, member, span))
    }

    pub fn new_type_prop_access(object: Expression, member: String, span: Span) -> Self {
        Expression::TypePropAccess(TypePropAccessNode::new(object, member, span))
    }

    pub fn new_print(expression: Expression, span: Span) -> Self {
        Expression::Print(PrintNode::new(expression, span))
    }

    pub fn span(&self) -> Span {
        match self {
            Expression::Number(n) => n.span,
            Expression::Boolean(b) => b.span,
            Expression::Str(s) => s.span,
            Expression::Identifier(i) => i.span,
            Expression::FunctionCall(f) => f.span,
            Expression::WhileLoop(w) => w.span,
            Expression::ForLoop(f) => f.span,
            Expression::CodeBlock(b) => b.expression_list.expressions[0].span(),
            Expression::BinaryOp(b) => b.span,
            Expression::UnaryOp(u) => u.span,
            Expression::IfElse(i) => i.span,
            Expression::LetIn(l) => l.span,
            Expression::DestructiveAssign(d) => d.span,
            Expression::TypeInstance(t) => t.span,
            Expression::TypeFunctionAccess(t) => t.span,
            Expression::TypePropAccess(t) => t.span,
            Expression::Print(p) => p.span,
        }
    }
}

impl Accept for Expression {
    fn accept<V: Visitor<T>, T>(&mut self, visitor: &mut V) -> T {
        match self {
            Expression::Number(node) => visitor.visit_literal_number(node),
            Expression::Boolean(node) => visitor.visit_literal_boolean(node),
            Expression::Str(node) => visitor.visit_literal_string(node),
            Expression::Identifier(node) => visitor.visit_identifier(node),
            Expression::FunctionCall(node) => visitor.visit_function_call(node),
            Expression::WhileLoop(node) => visitor.visit_while_loop(node),
            Expression::ForLoop(node) => visitor.visit_for_loop(node),
            Expression::CodeBlock(node) => visitor.visit_code_block(node),
            Expression::BinaryOp(node) => visitor.visit_binary_op(node),
            Expression::UnaryOp(node) => visitor.visit_unary_op(node),
            Expression::IfElse(node) => visitor.visit_if_else(node),
            Expression::LetIn(node) => visitor.visit_let_in(node),
            Expression::DestructiveAssign(node) => visitor.visit_destructive_assign(node),
            Expression::TypeInstance(node) => visitor.visit_type_instance(node),
            Expression::TypeFunctionAccess(node) => visitor.visit_type_function_access(node),
            Expression::TypePropAccess(node) => visitor.visit_type_prop_access(node),
            Expression::Print(node) => visitor.visit_print(node),
        }
    }
}
