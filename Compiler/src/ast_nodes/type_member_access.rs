use crate::{
    ast_nodes::{expression::Expression, function_call::FunctionCallNode},
    lexer_parser::tokens::Span,
    lexer_parser::types_tree::tree_node::TypeNode,
};

#[derive(Debug, PartialEq, Clone)]
pub struct TypePropAccessNode {
    pub object: Box<Expression>,
    pub member: Box<String>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl TypePropAccessNode {
    pub fn new(object: Expression, member: String, span: Span) -> Self {
        TypePropAccessNode {
            object: Box::new(object),
            member: Box::new(member),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeFunctionAccessNode {
    pub object: Box<Expression>,
    pub member: Box<FunctionCallNode>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl TypeFunctionAccessNode {
    pub fn new(object: Expression, member: FunctionCallNode, span: Span) -> Self {
        TypeFunctionAccessNode {
            object: Box::new(object),
            member: Box::new(member),
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
