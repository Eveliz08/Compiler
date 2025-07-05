use super::expression::Expression;
use crate::{tokens::{OperatorToken, Span}, types_tree::tree_node::TypeNode};

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOpNode {
    pub operator: OperatorToken,
    pub operand: Box<Expression>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl UnaryOpNode {
    pub fn new(operator: OperatorToken, operand: Expression, span: Span) -> Self {
        UnaryOpNode {
            operator,
            operand: Box::new(operand),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
