use crate::{ast_nodes::expression::Expression, lexer_parser::tokens::Span, lexer_parser::types_tree::tree_node::TypeNode};

#[derive(Debug, PartialEq, Clone)]
pub struct WhileNode {
    pub condition: Box<Expression>,
    pub body: Box<Expression>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl WhileNode {
    pub fn new(condition: Expression, body: Expression, span: Span) -> Self {
        WhileNode {
            condition: Box::new(condition),
            body: Box::new(body),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
