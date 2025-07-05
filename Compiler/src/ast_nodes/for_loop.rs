use crate::{ast_nodes::expression::Expression, lexer_parser::tokens::Span, types_tree::tree_node::TypeNode};

#[derive(Debug, PartialEq, Clone)]
pub struct ForNode {
    pub variable: String,
    pub start: Box<Expression>,
    pub end: Box<Expression>,
    pub body: Box<Expression>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl ForNode {
    pub fn new(
        variable: String,
        start: Expression,
        end: Expression,
        body: Expression,
        span: Span,
    ) -> Self {
        ForNode {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            body: Box::new(body),
            node_type: None,
            span,
        }
    }

    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
