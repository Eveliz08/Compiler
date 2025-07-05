use crate::{
    ast_nodes::{
        expression::Expression,
        function_def::{FunctionDefNode, FunctionParams},
        let_in::Assignment,
    },
    lexer_parser::tokens::Span,
    lexer_parser::types_tree::tree_node::TypeNode,
};

pub struct TypeInherits {
    pub identifier: String,
    pub params: Vec<Expression>,
    pub span: Span,
}

impl TypeInherits {
    pub fn new(identifier: String, params: Vec<Expression>, span: Span) -> Self {
        TypeInherits {
            identifier,
            params,
            span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeMember {
    Property(Assignment),
    Method(FunctionDefNode),
}

impl TypeMember {
    pub fn new_property(assignment: Assignment) -> Self {
        TypeMember::Property(assignment)
    }

    pub fn new_method(method: FunctionDefNode) -> Self {
        TypeMember::Method(method)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDefNode {
    pub identifier: String,
    pub params: Vec<FunctionParams>,
    pub parent: Option<String>,
    pub parent_args: Vec<Expression>,
    pub members: Vec<TypeMember>,
    pub node_type: Option<TypeNode>,
    pub span: Span,
}

impl TypeDefNode {
    pub fn new(
        identifier: String,
        params: Vec<FunctionParams>,
        parent: Option<String>,
        parent_args: Vec<Expression>,
        members: Vec<TypeMember>,
        span: Span,
    ) -> Self {
        TypeDefNode {
            identifier,
            params,
            parent,
            parent_args,
            members,
            node_type: None,
            span,
        }
    }
    pub fn set_type(&mut self, node_type: TypeNode) {
        self.node_type = Some(node_type);
    }
}
