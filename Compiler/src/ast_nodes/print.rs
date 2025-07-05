use crate::{lexer_parser::tokens::Span, lexer_parser::types_tree::tree_node::TypeNode};

use super::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct PrintNode {
    pub expression: Box<Expression>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl PrintNode {
    pub fn new(expression: Expression, span: Span) -> Self {
        PrintNode {
            expression: Box::new(expression),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
