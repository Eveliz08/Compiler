use super::expression::Expression;
use crate::{
    tokens::{OperatorToken, Span},
    types_tree::tree_node::TypeNode,
};

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOpNode {
    pub left: Box<Expression>,
    pub operator: OperatorToken,
    pub right: Box<Expression>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl BinaryOpNode {
    pub fn new(left: Expression, operator: OperatorToken, right: Expression, span: Span) -> Self {
        BinaryOpNode {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            node_type: None,
            span,
        }
    }

    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
