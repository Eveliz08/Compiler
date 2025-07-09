use super::utils::{to_llvm_type};
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
use crate::codegen::generator::Generator;
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
        ResultCodegen { register, llvm_type, ast_type }
    }
}

impl Visitor<ResultCodegen> for Generator {
    fn visit_function_def(&mut self, node: &mut FunctionDefNode) -> ResultCodegen {
       
        // let function_name = node.name.clone();
        // let params = node.params.clone();
        // let return_type = node.return_type.clone();
        // let function_global_name = format!("@{}", function_name);
        // self.ctx.push_scope();
        // let mut llvm_args: Vec<String> = params.iter().map(|param| {
        //     let llvm_type = to_llvm_type(param.signature.clone());
        //     let register_name = format!("%{}.{}", param.name.clone(), self.ctx.get_scope());
        //     self.ctx.add_variable(register_name.clone(), llvm_type.clone());
        //     format!("ptr {}", register_name)
        // }).collect();
        // if let Some (type_name) = self.ctx.current_self.clone() {
        //     self.ctx.add_variable(format!("%self.{}",self.ctx.get_scope()), type_name);
        //     llvm_args.insert(0, format!("ptr %self.{}",self.ctx.get_scope()));
        // }
        // self.ctx.add_line(format!("define {} {}({}) {{", to_llvm_type(return_type.clone()), function_global_name, llvm_args.join(", ")));
        // let llvm_body = node.body.accept(self);
        // self.ctx.add_line(format!("ret {} {}", llvm_body.llvm_type, llvm_body.register));
        // self.ctx.add_line("}".to_string());
        // self.ctx.exit_scope();
        // GeneratorResult::new(function_global_name, to_llvm_type(return_type.clone()),return_type.clone())
        todo!()
    }

    fn visit_literal_number(&mut self, node: &mut NumberLiteralNode) -> ResultCodegen {
        todo!()
    }

    fn visit_literal_boolean(&mut self, node: &mut BooleanLiteralNode) -> ResultCodegen {
        todo!()
    }

    fn visit_literal_string(&mut self, node: &mut StringLiteralNode) -> ResultCodegen {
        todo!()
    }

    fn visit_identifier(&mut self, node: &mut IdentifierNode) -> ResultCodegen {
        todo!()
    }

    fn visit_function_call(&mut self, node: &mut FunctionCallNode) -> ResultCodegen {
        todo!()
    }

    fn visit_while_loop(&mut self, node: &mut WhileNode) -> ResultCodegen {
        todo!()
    }

    fn visit_for_loop(&mut self, node: &mut ForNode) -> ResultCodegen {
        todo!()
    }

    fn visit_code_block(&mut self, node: &mut BlockNode) -> ResultCodegen {
        todo!()
    }

    fn visit_binary_op(&mut self, node: &mut BinaryOpNode) -> ResultCodegen {
        todo!()
    }

    fn visit_unary_op(&mut self, node: &mut UnaryOpNode) -> ResultCodegen {
        todo!()
    }

    fn visit_if_else(&mut self, node: &mut IfElseNode) -> ResultCodegen {
        todo!()
    }

    fn visit_let_in(&mut self, node: &mut LetInNode) -> ResultCodegen {
        todo!()
    }

    fn visit_destructive_assign(&mut self, node: &mut DestructiveAssignNode) -> ResultCodegen {
        todo!()
    }

    fn visit_type_def(&mut self, node: &mut TypeDefNode) -> ResultCodegen {
        todo!()
    }

    fn visit_type_instance(&mut self, node: &mut TypeInstanceNode) -> ResultCodegen {
        todo!()
    }

    fn visit_type_function_access(&mut self, node: &mut TypeFunctionAccessNode) -> ResultCodegen {
        todo!()
    }

    fn visit_type_prop_access(&mut self, node: &mut TypePropAccessNode) -> ResultCodegen {
        todo!()
    }

    fn visit_print(&mut self, node: &mut PrintNode) -> ResultCodegen {
        todo!()
    }
}