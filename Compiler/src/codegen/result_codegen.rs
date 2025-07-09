use super::code_generator::CodeGenerator;
use super::llvm_utils::{to_llvm_type};
use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::expression::Expression;
use crate::ast_nodes::for_loop::ForNode;
use crate::ast_nodes::function_call::FunctionCallNode;
use crate::ast_nodes::function_def::FunctionDefNode;
use crate::ast_nodes::if_else::IfElseNode;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::literals::{
    BooleanLiteralNode, IdentifierNode, NumberLiteralNode, StringLiteralNode,
};
use crate::ast_nodes::type_def::{TypeDefNode, TypeMember};
use crate::ast_nodes::type_instance::TypeInstanceNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::while_loop::WhileNode;
use crate::ast_nodes::print::PrintNode;
use crate::tokens::OperatorToken;
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;


pub struct ResultCodegen {
    pub register: String,   
    pub llvm_type: String,
    pub ast_type : String,
}
impl ResultCodegen {
    pub fn new(register: String, llvm_type: String, ast_type: String) -> Self {
        GeneratorResult { register, llvm_type, ast_type }
    }
}

impl Visitor<ResultCodegen> for Generator {


}