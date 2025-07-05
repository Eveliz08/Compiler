use crate::lexer_parser::types_tree::tree_node::TypeNode;

use super::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionList {
    pub expressions: Box<Vec<Expression>>,
}

impl ExpressionList {
    pub fn new(expressions: Vec<Expression>) -> Self {
        ExpressionList {
            expressions: Box::new(expressions),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockNode {
    pub expression_list: Box<ExpressionList>,
    pub node_type: Option<TypeNode>,
}

impl BlockNode {
    pub fn new(expression_list: ExpressionList) -> Self {
        BlockNode {
            expression_list: Box::new(expression_list),
            node_type: None,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
