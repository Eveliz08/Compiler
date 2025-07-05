use crate::{tokens::Span, types_tree::tree_node::TypeNode};

use super::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    //TODO Add optional Signature Assignment
    pub identifier: String,
    pub expression: Box<Expression>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl Assignment {
    pub fn new(identifier: String, expression: Expression, span: Span) -> Self {
        Assignment {
            identifier,
            expression: Box::new(expression),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LetInNode {
    pub assignments: Vec<Assignment>,
    pub body: Box<Expression>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl LetInNode {
    pub fn new(assignments: Vec<Assignment>, body: Expression, span: Span) -> Self {
        LetInNode {
            assignments,
            body: Box::new(body),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
