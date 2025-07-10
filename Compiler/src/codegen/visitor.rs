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
       
        let function_name = node.name.clone();
        let params = node.params.clone();
        let return_type = node.return_type.clone();
        let function_global_name = format!("@{}", function_name);
        self.ctx.push_scope();
        let mut llvm_args: Vec<String> = params.iter().map(|param| {
            let llvm_type = to_llvm_type(param.signature.clone());
            let register_name = format!("%{}.{}", param.name.clone(), self.ctx.current_scope_id());
            self.ctx.add_var(register_name.clone(), llvm_type.clone());
            format!("ptr {}", register_name)
        }).collect();
        if let Some (type_name) = self.ctx.current_self_type.clone() {
            self.ctx.add_var(format!("%self.{}",self.ctx.current_scope_id()), type_name);
            llvm_args.insert(0, format!("ptr %self.{}",self.ctx.current_scope_id()));
        }
        self.ctx.append_line(format!("define {} {}({}) {{", to_llvm_type(return_type.clone()), function_global_name, llvm_args.join(", ")));
        let llvm_body = node.body.accept(self);
        self.ctx.append_line(format!("ret {} {}", llvm_body.llvm_type, llvm_body.register));
        self.ctx.append_line("}".to_string());
        self.ctx.pop_scope();
        ResultCodegen::new(function_global_name, to_llvm_type(return_type.clone()),return_type.clone())
        

    }

    fn visit_literal_number(&mut self, node: &mut NumberLiteralNode) -> ResultCodegen {
        let mut raw = node.value.to_string();
        if !raw.contains('.') {
            raw.push_str(".0");
        }
        let temp = self.ctx.fresh_temp_var("Number".to_string());
        self.ctx
            .append_line(format!("{} = fadd double 0.0, {}", temp, raw));
        ResultCodegen::new(temp, "double".to_string(),"Number".to_string())
    }

    fn visit_literal_boolean(&mut self, node: &mut BooleanLiteralNode) -> ResultCodegen {
        let value = if node.value { "1" } else { "0" };
        let temp = self.ctx.fresh_temp_var("Boolean".to_string());
        self.ctx
            .append_line(format!("{} = add i1 {}, 0", temp, value));
        ResultCodegen::new(temp, "i1".to_string(), "Boolean".to_string())
    }

    fn visit_literal_string(&mut self, node: &mut StringLiteralNode) -> ResultCodegen {
        let temp = self.ctx.fresh_temp_var("String".to_string());
        let len = node.value.len();
        let global_const = self.ctx.add_string_const(node.value.clone(), len.clone());
        self.ctx.string_literals_map.insert(temp.clone(), node.value.clone());
        self.ctx.append_line(format!("{} = getelementptr [{} x i8], ptr {}, i32 0, i32 0",temp,len + 1,global_const));
        ResultCodegen::new(temp, "ptr".to_string(), "String".to_string())
    }

    fn visit_identifier(&mut self, node: &mut IdentifierNode) -> ResultCodegen {
        let value = node.value.clone();
        let llvm_type = to_llvm_type(node.node_type.clone().unwrap().type_name);
        if self.ctx.is_global_const(&value) {
            let register = self.ctx.fresh_temp_var(node.node_type.clone().unwrap().type_name);
            let global_ref = format!("@{}", value);
            self.ctx.append_line(format!(
                "{} = load {}, ptr {}",register, llvm_type, global_ref // Usar directamente el nombre (@PI, @E)
            ));
             ResultCodegen::new(register, llvm_type,node.node_type.clone().unwrap().type_name)
        } 
        else {
            let register = self.ctx.fresh_temp_var(llvm_type.clone());
            self.ctx.append_line(format!(
                 "{} = load {}, ptr {}", register, llvm_type.clone(), self.ctx.get_var(value)
            ));
            ResultCodegen::new(register, llvm_type.clone(),node.node_type.clone().unwrap().type_name)
        }
    }

    fn visit_function_call(&mut self,  node: &mut FunctionCallNode) -> ResultCodegen {
        let name = node.function_name.clone();
        let llvm_args: Vec<String> = node.arguments.iter().map(|arg| {
            let arg_val = arg.clone().accept(self);
            let arg_reg = self.ctx.fresh_temp_var(arg_val.llvm_type.clone());
            self.ctx.append_line(format!("{} = alloca {}", arg_reg.clone(), arg_val.llvm_type));
            self.ctx.append_line(format!(
                "store {} {}, ptr {}",
                arg_val.llvm_type, arg_val.register, arg_reg.clone()
            ));
            format!("ptr {}", arg_reg)
        }).collect();
        let node_type = to_llvm_type(node.node_type.clone().unwrap().type_name);
        let temp = self.ctx.fresh_temp_var(node_type.clone());
        self.ctx.append_line(format!(
            "{} = call {} @{}({})",
            temp, node_type, name, llvm_args.join(", ")
        ));
        ResultCodegen::new(temp, node_type,node.node_type.clone().unwrap().type_name)

    }

    fn visit_while_loop(&mut self, node: &mut WhileNode) -> ResultCodegen {
        let id_cond = self.ctx.fresh_id();
        let id_loop = self.ctx.fresh_id();
        let cond_label = format!("while_cond.{}",id_cond);
        let loop_label = format!("while_loop.{}",id_loop);
        let id_exit = self.ctx.fresh_id();
        let exit_label = format!("while_exit.{}",id_exit);
        let node_type = to_llvm_type(node.node_type.clone().unwrap().type_name);

        let result_reg = self.ctx.fresh_temp_var(node_type.clone());
        self.ctx.append_line(format!("{} = alloca {}", result_reg.clone() ,node_type.clone()));
        
        self.ctx.append_line(format!("br label %{}\n\n", cond_label));

        self.ctx.append_line(format!("{}:", cond_label));
        let cond_register = node.condition.accept(self);
        self.ctx.append_line(format!(
            "br i1 {}, label %{}, label %{}\n\n",
            cond_register.register, loop_label, exit_label
        ));
        self.ctx.append_line(format!("{}:", loop_label));
        let body_register = node.body.accept(self);
        self.ctx.append_line(format!(
            "store {} {}, ptr {}\n\n",
            node_type.clone(), body_register.register, result_reg.clone()
        ));
        self.ctx.append_line(format!("br label %{}\n\n", cond_label));

        self.ctx.append_line(format!("{}:", exit_label));
        let return_reg = self.ctx.fresh_temp_var(node_type.clone());
        self.ctx.append_line(format!(
            "{} = load {}, ptr {}\n",
            return_reg.clone(), node_type.clone(), result_reg.clone()
        ));
        ResultCodegen::new(return_reg, node_type,node.node_type.clone().unwrap().type_name)
    }

    fn visit_for_loop(&mut self, node: &mut ForNode) -> ResultCodegen {
        let start_reg = node.start.accept(self);
        let end_reg = node.end.accept(self);
        self.ctx.push_scope();
        let for_condition_id = self.ctx.fresh_id();
        let for_condition_label = format!("for_condition.{}", for_condition_id); 
        let for_body_id = self.ctx.fresh_id();
        let for_body_label = format!("for_body.{}", for_body_id);
        let for_exit_id = self.ctx.fresh_id();
        let for_exit_label = format!("for_exit.{}", for_exit_id);
        let index_reg = format!("%{}.{}", node.variable, self.ctx.current_scope_id());
        let comp_reg = self.ctx.fresh_temp_var("Number".to_string());
        self.ctx.append_line(format!(
            "{} = fcmp ole double {}, {}",
            comp_reg, start_reg.register, end_reg.register
        ));
        let step_reg = self.ctx.fresh_temp_var("Number".to_string());
        self.ctx.append_line(format!(
            "{} = select i1 {}, double 1.0, double -1.0",
            step_reg, comp_reg
        ));
        self.ctx.add_var(index_reg.clone(), start_reg.llvm_type.clone());
        self.ctx.append_line(format!(
            "{} = alloca {}",
            index_reg, start_reg.llvm_type
        )); 
        self.ctx.append_line(format!(
            "store {} {}, ptr {}",
            start_reg.llvm_type, start_reg.register, index_reg
        ));

        self.ctx.append_line(format!("br label %{}\n\n", for_condition_label));
        self.ctx.append_line(format!("{}:", for_condition_label));
        let curr = self.ctx.fresh_temp_var("Number".to_string());
        self.ctx.append_line(format!(
            "{} = load {}, ptr {}\n",
            curr, start_reg.llvm_type, index_reg
        ));
        let comp_up = self.ctx.fresh_temp_var("Number".to_string());
        self.ctx.append_line(format!(
            "{} = fcmp ole double {}, {}",
            comp_up, curr, end_reg.register
        ));
        let comp_down = self.ctx.fresh_temp_var("Number".to_string());
        self.ctx.append_line(format!(
            "{} = fcmp oge double {}, {}",
            comp_down, curr, end_reg.register
        ));
        let condition = self.ctx.fresh_temp_var("Boolean".to_string());
        self.ctx.append_line(format!(
            "{} = select i1 {}, i1 {}, i1 {}",
            condition, comp_reg ,comp_up, comp_down
        ));
        self.ctx.append_line(format!(
            "br i1 {}, label %{}, label %{}\n\n",
            condition, for_body_label, for_exit_label
        ));
        self.ctx.append_line(format!("{}:", for_body_label));
        let body_result = node.body.accept(self);
        let step_val = self.ctx.fresh_temp_var("Number".to_string());
        self.ctx.append_line(format!(
            "{} = fadd double {}, {}\n",
            step_val, curr, step_reg
        ));
        self.ctx.append_line(format!(
            "store double {}, ptr {}\n\n",
            step_val, index_reg
        ));
        self.ctx.append_line(format!("br label %{}\n\n", for_condition_label));
        self.ctx.append_line(format!("{}:", for_exit_label));
        self.ctx.pop_scope();
        ResultCodegen::new(body_result.register, body_result.llvm_type, body_result.ast_type)
    }

    fn visit_code_block(&mut self, node: &mut BlockNode) -> ResultCodegen {
        self.ctx.push_scope();
        let mut result = ResultCodegen::new("".to_string(), "".to_string(),"".to_string());
        for expr in node.expression_list.expressions.iter_mut() {
            let current = expr.accept(self);
            result = current;
        }
        self.ctx.pop_scope();
        result
    }

    fn visit_binary_op(&mut self, node: &mut BinaryOpNode) -> ResultCodegen {
        let left_val = node.left.accept(self);
        let right_val = node.right.accept(self);
        let op = node.operator.clone();
        match op {
            OperatorToken::PLUS | OperatorToken::MINUS | OperatorToken::MUL | OperatorToken::DIV => {
                let opcode = match op {
                    OperatorToken::PLUS => "fadd",
                    OperatorToken::MINUS => "fsub",
                    OperatorToken::MUL => "fmul",
                    OperatorToken::DIV => "fdiv",
                    _ => unreachable!(),
                };

                let temp = self.ctx.fresh_temp_var("Number".to_string());
                self.ctx.append_line(format!(
                    "{} = {} double {}, {}",
                    temp, opcode, left_val.register, right_val.register
                ));
                ResultCodegen::new(temp, "double".to_string(),"Number".to_string())
            }

            OperatorToken::MOD => {
                if !self.ctx.used_runtime_funcs.contains("fmod") {
                    self.ctx.add_global_decl("declare double @fmod(double, double)".to_string());
                    self.ctx.used_runtime_funcs.insert("fmod".to_string());
                }

                let temp = self.ctx.fresh_temp_var("Number".to_string());
                self.ctx.append_line(format!(
                    "{} = call double @fmod(double {}, double {})",
                    temp, left_val.register, right_val.register
                ));
                ResultCodegen::new(temp, "double".to_string(),"Number".to_string())
            }

            OperatorToken::POW => {
                if !self.ctx.used_runtime_funcs.contains("pow") {
                    self.ctx.add_global_decl("declare double @pow(double, double)".to_string());
                    self.ctx.used_runtime_funcs.insert("pow".to_string());
                }

                let temp = self.ctx.fresh_temp_var("Number".to_string());
                self.ctx.append_line(format!(
                    "{} = call double @pow(double {}, double {})",
                    temp, left_val.register, right_val.register
                ));
                ResultCodegen::new(temp, "double".to_string(),"Number".to_string())
            }

            OperatorToken::EQ
            | OperatorToken::NEQ
            | OperatorToken::GT
            | OperatorToken::GTE
            | OperatorToken::LT
            | OperatorToken::LTE => {
                if self.ctx.get_temp_var_type(&left_val.register) == "Boolean"
                    && self.ctx.get_temp_var_type(&right_val.register) == "Boolean"
                {
                    let cmp_op = match op {
                        OperatorToken::EQ => "eq",
                        OperatorToken::NEQ => "ne",
                        _ => panic!("Invalid comparison operator for booleans: {:?}", op),
                    };

                    let temp = self.ctx.fresh_temp_var("Boolean".to_string());
                    self.ctx.append_line(format!(
                        "{} = icmp {} i1 {}, {}",
                        temp, cmp_op, left_val.register, right_val.register
                    ));
                    ResultCodegen::new(temp, "i1".to_string(),"Boolean".to_string())
                } else {
                    let cmp_op = match op {
                        OperatorToken::EQ => "oeq",
                        OperatorToken::NEQ => "one",
                        OperatorToken::GT => "ogt",
                        OperatorToken::GTE => "oge",
                        OperatorToken::LT => "olt",
                        OperatorToken::LTE => "ole",
                        _ => unreachable!(),
                    };

                    let temp = self.ctx.fresh_temp_var("Boolean".to_string());
                    self.ctx.append_line(format!(
                        "{} = fcmp {} double {}, {}",
                        temp, cmp_op, left_val.register, right_val.register
                    ));
                    ResultCodegen::new(temp, "i1".to_string(),"Boolean".to_string())
                }
            }

            OperatorToken::AND | OperatorToken::OR => {
                let opcode = match op {
                    OperatorToken::AND => "and",
                    OperatorToken::OR => "or",
                    _ => unreachable!(),
                };

                let temp = self.ctx.fresh_temp_var("Boolean".to_string());
                self.ctx.append_line(format!(
                    "{} = {} i1 {}, {}",
                    temp, opcode, left_val.register, right_val.register
                ));
                ResultCodegen::new(temp, "i1".to_string(),"Boolean".to_string())
            }

            OperatorToken::CONCAT => {
                let len1 = self.ctx.fresh_temp_var("Number".to_string());
                let len2 = self.ctx.fresh_temp_var("Number".to_string());
                let total_len = self.ctx.fresh_temp_var("Number".to_string());
                let total_len_plus_one = self.ctx.fresh_temp_var("Number".to_string());
                let result_ptr = self.ctx.fresh_temp_var("String".to_string());
                let copy_reg = self.ctx.fresh_temp_var("ptr".to_string());
                let result = self.ctx.fresh_temp_var("String".to_string());
                self.ctx.append_line(
                    format!(
                        "{} = call i32 @strlen(ptr {})\n
                        {} = call i32 @strlen(ptr {})\n
                        {} = add i32 {}, {}\n
                        {} = add i32 {}, 1\n
                        {} = call ptr @malloc(i32 {})\n
                        {} = call ptr @strcpy(ptr {}, ptr {})\n 
                        {} = call ptr @strcat(ptr {}, ptr {})\n
                        ",len1,left_val.register,len2, right_val.register,
                        total_len,len1,len2,total_len_plus_one,total_len,
                        result_ptr,total_len_plus_one,copy_reg,result_ptr,left_val.register,
                        result,result_ptr,right_val.register
                    )
                );
                ResultCodegen::new(result, "ptr".to_string(),"String".to_string())
            }

            _ => panic!("Unsupported binary operator: {:?}", op),
        }
    }

    fn visit_unary_op(&mut self, node: &mut UnaryOpNode) -> ResultCodegen {
        let operand_val = node.operand.accept(self);
        let op = node.operator.clone();
        match op {
            OperatorToken::NEG => {
                let temp = self.ctx.fresh_temp_var("Number".to_string());
                self.ctx.append_line(format!("{} = fsub double 0.0, {}", temp, operand_val.register));
                ResultCodegen::new(temp, "double".to_string(),"Number".to_string())
            }
            OperatorToken::NOT => {
                let temp = self.ctx.fresh_temp_var("Boolean".to_string());
                self.ctx.append_line(format!("{} = xor i1 {}, true", temp, operand_val.register));
                ResultCodegen::new(temp, "i1".to_string(),"Boolean".to_string())
            }
            _ => panic!("Unsupported unary operator: {:?}", op),
        }
    }

    fn visit_if_else(&mut self, node: &mut IfElseNode) -> ResultCodegen {
        let node_type = node.node_type.clone().unwrap().type_name;
        let node_type_llvm = to_llvm_type(node_type.clone());
        let result_reg = self.ctx.fresh_temp_var(node_type.clone());
        let exit_id = self.ctx.fresh_id();
        let exit_label = format!("if_else_exit.{}", exit_id);
        self.ctx.append_line(format!("{} = alloca {}", result_reg, node_type_llvm));
        let cond_reg = node.condition.accept(self);
        let if_id = self.ctx.fresh_id();
        let if_true_label = format!("if_true.{}", if_id);
        let if_false_label = format!("if_false.{}", if_id);
        self.ctx.append_line(format!(
            "br i1 {}, label %{}, label %{}",
            cond_reg.register, if_true_label, if_false_label
        ));
        self.ctx.append_line(format!("{}:", if_true_label));
        let if_expr = node.if_expression.accept(self);
        self.ctx.append_line(format!(
            "store {} {}, ptr {}\n",
            node_type_llvm, if_expr.register, result_reg
        ));
        self.ctx.append_line(format!("br label %{}\n\n", exit_label));
        self.ctx.append_line(format!("{}:", if_false_label));
        if node.elifs.len() > 0 {
            for (cond, expr) in node.elifs.iter_mut() {
                let elif_id = self.ctx.fresh_id();
                let elif_label = format!("elif_true.{}", elif_id);
                let elif_false_label = format!("elif_false.{}", elif_id);
                let elif_cond_reg = if let Some(cond_expr) = cond {
                    cond_expr.accept(self)
                } else {
                    let else_reg = self.ctx.fresh_temp_var("Boolean".to_string());
                    self.ctx.append_line(format!(
                        "{} = add i1 0, 1", else_reg
                    ));
                    ResultCodegen::new(else_reg, "i1".to_string(),"Boolean".to_string())
                };
                if let Some(_) = cond {
                    self.ctx.append_line(format!(
                        "br i1 {}, label %{}, label %{}",
                        elif_cond_reg.register, elif_label, elif_false_label
                    ));
                } else {
                    self.ctx.append_line(format!("br label %{}\n\n", elif_label));
                }
                self.ctx.append_line(format!("{}:", elif_label));
                let elif_expr = expr.clone().accept(self);
                self.ctx.append_line(format!(
                    "store {} {}, ptr {}\n",
                    node_type_llvm, elif_expr.register, result_reg
                ));
                if let Some(_) = cond {
                    self.ctx.append_line(format!("br label %{}\n\n", exit_label));
                }
                
                if let Some(_) = cond {
                    self.ctx.append_line(format!("{}:", elif_false_label));
                } else {
                    self.ctx.append_line(format!("br label %{}\n\n", exit_label));
                }
            }
        } else {
            self.ctx.append_line(format!("br label %{}\n\n", exit_label));
        }
        self.ctx.append_line(format!("{}:", exit_label));
        let final_result = self.ctx.fresh_temp_var(node_type.clone());
        self.ctx.append_line(format!(
            "{} = load {}, ptr {}\n",
            final_result, node_type_llvm, result_reg
        ));
        ResultCodegen::new(final_result, node_type_llvm,node_type)
    }

    fn visit_let_in(&mut self, node: &mut LetInNode) -> ResultCodegen {
        self.ctx.push_scope();
        for assig in node.assignments.clone().iter_mut() {
            let identifier = assig.identifier.clone();
            let body = assig.expression.accept(self);
            let llvm_type = to_llvm_type(body.ast_type.clone());
            let register_name = format!("%{}.{}", identifier, self.ctx.current_scope_id());
            self.ctx.temp_var_types.insert(register_name.clone(), assig.node_type.clone().unwrap().type_name.clone());
            self.ctx.add_var(register_name.clone(), llvm_type.clone());
            self.ctx.append_line(format!(
                "{} = alloca {}",
                register_name.clone(), llvm_type
            ));
            self.ctx.append_line(format!(
                "store {} {}, ptr {}",
                llvm_type, body.register, register_name
            ));
        }
        let body_result = node.body.accept(self);
        self.ctx.pop_scope();
        ResultCodegen::new(body_result.register, body_result.llvm_type, body_result.ast_type)
    }

    fn visit_destructive_assign(&mut self, node: &mut DestructiveAssignNode) -> ResultCodegen {
        let expr_result = node.expression.accept(self);
        match *node.identifier.clone() {
            Expression::Identifier(id) => {
                let identifier_register = self.ctx.get_var(id.value.clone());
                self.ctx.append_line(format!(
                    "store {} {}, ptr {}",
                    expr_result.llvm_type, expr_result.register, identifier_register
                ));
            }
            Expression::TypePropAccess(obj) => {
                let obj_type = self.ctx.current_self_type.clone().unwrap();
                let prop_reg = self.ctx.fresh_temp_var(obj_type.clone());
                let prop_index = self.ctx.member_indices.get(&(obj_type.clone(), (*obj.member).clone())).unwrap();
                self.ctx.append_line(format!("{} = getelementptr %{}_type ,ptr %self.{}, i32 0 , i32 {}", prop_reg,obj_type.clone(),self.ctx.current_scope_id(),prop_index.clone()));
                self.ctx.append_line(format!("store {} {}, ptr {}" , expr_result.llvm_type, expr_result.register, prop_reg));
            }
            _ => panic!("Error: assigment not possible")
        }
        ResultCodegen::new(expr_result.register, expr_result.llvm_type, expr_result.ast_type)
    }

    fn visit_type_def(&mut self, node: &mut TypeDefNode) -> ResultCodegen {
        let type_name = node.identifier.clone();

        self.generate_type_constructor(node);
        self.ctx.current_self_type = Some(node.identifier.clone());
        for member in node.members.iter_mut() {
            match  member {
                TypeMember::Method(method) => {
                    method.name = format!("{}_{}",type_name.clone(),method.name.clone());
                    self.visit_function_def(method);
                    
                }
                _ => continue
            }
        }
        self.ctx.current_self_type = None;
        ResultCodegen::new(format!("%{}_type",type_name.clone()), format!("%{}_type",type_name.clone()), type_name.clone())

    }

    fn visit_type_instance(&mut self, node: &mut TypeInstanceNode) -> ResultCodegen { 
        let type_constructor = format!("@{}_new", node.type_name);
        let llvm_args: Vec<String> = node.arguments.iter().map(|arg| {
            let arg_val = arg.clone().accept(self);
            let arg_reg = self.ctx.fresh_temp_var(arg_val.llvm_type.clone());
            self.ctx.append_line(format!("{} = alloca {}", arg_reg.clone(), arg_val.llvm_type));
            self.ctx.append_line(format!(
                "store {} {}, ptr {}",
                arg_val.llvm_type, arg_val.register, arg_reg.clone()
            ));
            format!("ptr {}", arg_reg)
        }).collect();
        let args_str = llvm_args.join(", ");
        let result = self.ctx.fresh_temp_var(node.node_type.clone().unwrap().type_name);
        self.ctx.append_line(format!(
            "{} = call ptr {}({})",
            result.clone(), type_constructor, args_str
        ));
        ResultCodegen::new(result.clone(), "ptr".to_string(),node.type_name.clone())
    }

    fn visit_type_function_access(&mut self, node: &mut TypeFunctionAccessNode) -> ResultCodegen {
        let object = node.object.accept(self); 

        let mut curr_object_type = object.ast_type.clone();
        let function_name = node.member.function_name.clone();
        let mut curr_type_reg_ptr = object.register.clone();

        while ! self.ctx.func_indices.contains_key(&(curr_object_type.clone(),function_name.clone())) {
            let parent_opt = {
                self.ctx.type_inheritance.get(&curr_object_type.clone()).cloned()
            };
            if let Some(parent) = parent_opt {
                let parent_ptr_ptr = self.ctx.fresh_temp_var("ptr".to_string());
                self.ctx.append_line(format!("{} = getelementptr %{}_type, ptr {}, i32 0, i32 1", parent_ptr_ptr.clone(),curr_object_type.clone(), curr_type_reg_ptr.clone()));
                let parent_ptr = self.ctx.fresh_temp_var("ptr".to_string());
                self.ctx.append_line(format!("{} = load ptr, ptr {}", parent_ptr.clone(), parent_ptr_ptr.clone()));
                curr_object_type = parent; 
                curr_type_reg_ptr = parent_ptr.clone();
            } else {
                panic!("Method not found.")
            }
        }

        let function_index = *self.ctx.func_indices.get(&(curr_object_type.clone(), node.member.function_name.clone())).unwrap();
        let type_id_ptr = self.ctx.fresh_temp_var("ptr".to_string());
        self.ctx.append_line(format!("{} = getelementptr %{}_type, ptr {}, i32 0, i32 0", type_id_ptr, curr_object_type.clone(), curr_type_reg_ptr.clone()));
        let type_id = self.ctx.fresh_temp_var("i32".to_string());
        self.ctx.append_line(format!("{} = load i32, ptr {}", type_id, type_id_ptr));
        let func_ptr = self.ctx.fresh_temp_var("ptr".to_string());
        self.ctx.append_line(format!("{} = call ptr @get_vtable_method(i32 {},i32 {})", func_ptr, type_id, function_index));
        
        let return_type = node.node_type.clone().unwrap().type_name;
        let return_llvm = to_llvm_type(return_type.clone());
        let mut llvm_args: Vec<String> = Vec::new();
        for arg in node.member.arguments.iter_mut() {
            let arg_val = arg.accept(self);
            let arg_reg = self.ctx.fresh_temp_var(arg_val.llvm_type.clone());
            self.ctx.append_line(format!("{} = alloca {}", arg_reg.clone(), arg_val.llvm_type));
            self.ctx.append_line(format!(
                "store {} {}, ptr {}",
                arg_val.llvm_type, arg_val.register, arg_reg.clone()
            ));
            llvm_args.push(format!("ptr {}", arg_reg));
        }
        llvm_args.insert(0, format!("ptr {}", curr_type_reg_ptr.clone()));
        let temp = self.ctx.fresh_temp_var(return_type.clone());
        self.ctx.append_line(format!(
            "{} = call {} {}({})",
            temp.clone(), return_llvm, func_ptr, llvm_args.join(", ")
        ));
        ResultCodegen::new(temp, to_llvm_type(node.member.node_type.clone().unwrap().type_name), node.member.node_type.clone().unwrap().type_name)
    }

    fn visit_type_prop_access(&mut self, node: &mut TypePropAccessNode) -> ResultCodegen {
        let object = node.object.accept(self);
        let member_index = self.ctx.member_indices.get(&(object.ast_type.clone(), (*node.member).clone())).unwrap().clone();
        let node_type = to_llvm_type(node.node_type.clone().unwrap().type_name);
        let ptr_temp = self.ctx.fresh_temp_var(to_llvm_type(node_type.clone()));
        self.ctx.append_line(format!("{} = getelementptr %{}_type, ptr %self.{}, i32 0 , i32 {}", ptr_temp, object.ast_type.clone(), self.ctx.current_scope_id(), member_index));
        let result = self.ctx.fresh_temp_var(node_type.clone());
        self.ctx.append_line(format!("{} = load {}, ptr {}", result.clone(), node_type.clone(), ptr_temp.clone()));
        ResultCodegen::new(result, node_type.clone(), node.node_type.clone().unwrap().type_name)
    }
    
    fn visit_print(&mut self, node: &mut PrintNode) -> ResultCodegen {
        let arg = node.expression.accept(self);
        let call_reg = self.ctx.fresh_temp_var("i32".to_string());
        let new_line_reg = self.ctx.fresh_temp_var("ptr".to_string());
        let call_new_line = self.ctx.fresh_temp_var("ptr".to_string());
        let id = self.ctx.fresh_id();
        let new_line = format!("{} = getelementptr [2 x i8], ptr @.newline, i32 0, i32 0", new_line_reg);
        let new_line2 = format!("{} = call i32 (ptr, ...) @printf(ptr {})", call_new_line, new_line_reg);
        if arg.llvm_type == "i1" {
            self.ctx.append_line(format!("%bool_ptr{} = select i1 {}, ptr @.true_str, ptr @.false_str", id ,arg.register));
            self.ctx.append_line(format!("{} = call i32 (ptr, ...) @printf(ptr %bool_ptr{})", call_reg,id));
        } else if arg.llvm_type == "double" {
            self.ctx.append_line(format!("%fmt_dbl_ptr{} = getelementptr [4 x i8], ptr @.str.f, i32 0, i32 0", id)); 
            self.ctx.append_line(format!("{} = call i32 (ptr, ...) @printf(ptr %fmt_dbl_ptr{}, double {})", call_reg, id, arg.register));
        } else if arg.llvm_type == "ptr" {
            self.ctx.append_line(format!("{} = call i32 (ptr, ...) @printf(ptr {})", call_reg, arg.register));
            self.ctx.append_line(new_line);
            self.ctx.append_line(new_line2);
        } else {
            panic!("Unsupported expression type for print: {:?}", node.expression);
        }
        ResultCodegen::new(arg.register, arg.llvm_type, node.node_type.clone().unwrap().type_name)
    }
}