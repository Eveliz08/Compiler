use core::str;

use crate::{lexer_parser::tokens::Span, types_tree::tree_node::TypeNode};

#[derive(Debug, PartialEq, Clone)]
pub struct NumberLiteralNode {
    pub value: f64,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}
impl NumberLiteralNode {
    pub fn new(value: &str, span: Span) -> Self {
        NumberLiteralNode {
            value: value.parse::<f64>().unwrap(),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BooleanLiteralNode {
    pub value: bool,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}
impl BooleanLiteralNode {
    pub fn new(value: bool, span: Span) -> Self {
        match value {
            true => BooleanLiteralNode {
                value: true,
                node_type: None,
                span,
            },
            false => BooleanLiteralNode {
                value: false,
                node_type: None,
                span,
            },
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteralNode {
    pub value: String,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}
impl StringLiteralNode {
    pub fn new(value: &str, span: Span) -> Self {
        StringLiteralNode {
            value: value.to_string(),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdentifierNode {
    pub value: String,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}
impl IdentifierNode {
    pub fn new(value: &str, span: Span) -> Self {
        IdentifierNode {
            value: value.to_string(),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
