use crate::{tokens::Span, types_tree::tree_node::TypeNode};

use super::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParams {
    pub name: String,
    pub signature: String,
    pub span: Span,
}

impl FunctionParams {
    pub fn new(name: String, signature: String, span: Span) -> Self {
        FunctionParams {
            name,
            signature,
            span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefNode {
    pub name: String,
    pub params: Vec<FunctionParams>,
    pub return_type: String,
    pub body: Expression,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl FunctionDefNode {
    pub fn new_expr(
        name: String,
        params: Vec<FunctionParams>,
        return_type: String,
        expr: Expression,
        span: Span,
    ) -> Self {
        FunctionDefNode {
            name,
            params,
            return_type,
            body: expr,
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
