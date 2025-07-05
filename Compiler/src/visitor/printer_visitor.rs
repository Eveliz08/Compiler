//! `PrinterVisitor` implements the [`Visitor`] trait to produce string representations of AST nodes.
//!
//! This is primarily useful for pretty-printing the AST back into a readable, source-like format.

use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::function_call::FunctionCallNode;
use crate::ast_nodes::print::PrintNode;
use crate::ast_nodes::type_def::{TypeDefNode, TypeMember};
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::if_else::IfElseNode;
use crate::ast_nodes::literals::{NumberLiteralNode, BooleanLiteralNode, StringLiteralNode, IdentifierNode};
use crate::ast_nodes::while_loop::WhileNode;
use crate::ast_nodes::for_loop::ForNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::function_def::FunctionDefNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::program::Program;
use crate::ast_nodes::type_instance::TypeInstanceNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};

use super::visitor_trait::Visitor;
use super::accept::Accept;

/// A visitor that converts AST nodes into their source-code-like string representation.
///
/// This is useful for debugging, testing, or pretty-printing the AST back into
/// a format resembling the original source code.
pub struct PrinterVisitor;

impl PrinterVisitor {
    /// Prints the full program by visiting each top-level statement.
    ///
    /// # Arguments
    /// * `node` - The root `Program` node to print.
    ///
    /// # Returns
    /// A string representing the entire program.
    pub fn print_program(&mut self, node: &mut Program) -> String {
        let statements: Vec<String> = node.statements.iter_mut()
            .map(|statement| format!("{} ;\n", statement.accept(self)))
            .collect();
        format!("{}", statements.join("\n")) 
    }
}

impl Visitor<String> for PrinterVisitor {
    fn visit_function_def(&mut self, node: &mut FunctionDefNode) -> String {
        let name = &node.name;
        let params: Vec<String> = node.params.iter()
            .map(|param| format!("{}: {}", param.name, param.signature))
            .collect();
        let body = node.body.accept(self);
        format!("function {} ({}) : {} {{ \n{}\n}}" , name, params.join(", "), node.return_type, body)
    }

    fn visit_literal_number(&mut self, node: &mut NumberLiteralNode) -> String {
        format!("{}", node.value)
    }

    fn visit_literal_boolean(&mut self, node: &mut BooleanLiteralNode) -> String {
        format!("{}", node.value)
    }

    fn visit_literal_string(&mut self, node: &mut StringLiteralNode) -> String {
        format!("\"{}\"", node.value)
    }

    fn visit_identifier(&mut self, node: &mut IdentifierNode) -> String {
        format!("{}", node.value.clone())
    }

    fn visit_function_call(&mut self, node: &mut FunctionCallNode) -> String {
        let args: Vec<String> = node.arguments.iter_mut()
            .map(|arg| arg.accept(self))
            .collect();
        format!("{}({})", node.function_name, args.join(", "))
    }

    fn visit_while_loop(&mut self, node: &mut WhileNode) -> String {
        let condition = node.condition.accept(self);
        let body = node.body.accept(self);
        format!("while ({}) {{\n{}\n}}", condition, body)
    }

    fn visit_for_loop(&mut self, node: &mut ForNode) -> String {
        let variable = &node.variable;
        let start = node.start.accept(self);
        let end = node.end.accept(self);
        let body = node.body.accept(self);
        format!("for ({} in range({}, {})) {{\n{}\n}}", variable, start, end, body)
    }

    fn visit_code_block(&mut self, node: &mut BlockNode) -> String {
        let expressions: Vec<String> = node.expression_list.expressions.iter_mut()
            .map(|expr| expr.accept(self))
            .collect();
        format!("{}", expressions.join("\n"))
    }

    fn visit_binary_op(&mut self, node: &mut BinaryOpNode) -> String {
        let left = node.left.accept(self);
        let right = node.right.accept(self);
        format!("{} {} {}", left, node.operator, right)
    }

    fn visit_unary_op(&mut self, node: &mut UnaryOpNode) -> String {
        let operand = node.operand.accept(self);
        format!("{} {}", node.operator, operand)
    }

    fn visit_if_else(&mut self, node: &mut IfElseNode) -> String {
        let condition = node.condition.accept(self);
        let if_body = node.if_expression.accept(self);
        let mut result = format!("if ({}) {{\n{}\n}}", condition, if_body);

        for (condition, body) in node.elifs.iter() {
            let expr_body = body.clone().accept(self);
            if let Some(cond) = condition {
                let elif_condition = cond.clone().accept(self);
                result.push_str(&format!(" elif ({}) {{\n{}\n}}", elif_condition, expr_body));
            } else {
                result.push_str(&format!(" else {{\n{}\n}}", expr_body));
            }
        }

        result
    }

    fn visit_let_in(&mut self, node: &mut LetInNode) -> String {
        let assignments: Vec<String> = node.assignments.iter_mut()
            .map(|assignment| format!("{} = {}", assignment.identifier, assignment.expression.accept(self)))
            .collect();
        let body = node.body.accept(self);
        format!("let {} in {}", assignments.join(", "), body)
    }

    fn visit_destructive_assign(&mut self, node: &mut DestructiveAssignNode) -> String {
        let id = &node.identifier.accept(self);
        let expr = &node.expression.accept(self);
        format!("{} := {}", id, expr)
    }

    fn visit_type_def(&mut self, node: &mut TypeDefNode) -> String {
        let type_name = node.identifier.clone();
        let type_params: Vec<String> = node.params.iter()
            .map(|param| format!("{}: {}", param.name, param.signature))
            .collect();

        let members: Vec<String> = node.members.iter_mut()
            .map(|member| match member {
                TypeMember::Property(assign) => {
                    let name = &assign.identifier;
                    let value = assign.expression.accept(self);
                    format!("{} = {}\n", name, value)
                }
                TypeMember::Method(method) => {
                    format!("{}\n", self.visit_function_def(method))
                }
            })
            .collect();

        if let Some(parent) = &node.parent {
            let parent_args: Vec<String> = node.parent_args.iter_mut()
                .map(|arg| arg.accept(self))
                .collect();
            return format!(
                "type {} {} inherits {}({}) {{\n{}\n}}",
                type_name,
                if type_params.is_empty() {
                    "".to_string()
                } else {
                    format!("( {} )", type_params.join(", "))
                },
                parent,
                parent_args.join(", "),
                members.join("\n")
            );
        }

        format!(
            "type {} {} {{\n{}\n}}",
            type_name,
            if type_params.is_empty() {
                "".to_string()
            } else {
                format!("( {} )", type_params.join(", "))
            },
            members.join("\n")
        )
    }

    fn visit_type_instance(&mut self, node: &mut TypeInstanceNode) -> String {
        let type_name = &node.type_name;
        let type_args: Vec<String> = node.arguments.iter_mut()
            .map(|arg| arg.accept(self))
            .collect();
        format!("new {}({})", type_name, type_args.join(", "))
    }

    fn visit_type_function_access(&mut self, node: &mut TypeFunctionAccessNode) -> String {
        let object = node.object.accept(self);
        let member_call = self.visit_function_call(&mut node.member);
        format!("{}.{}", object, member_call)
    }

    fn visit_type_prop_access(&mut self, node: &mut TypePropAccessNode) -> String {
        let object = node.object.accept(self);
        let member = &node.member;
        format!("{}.{}", object, member)
    }

    fn visit_print(&mut self, node: &mut PrintNode) -> String {
        let expr = node.expression.accept(self);
        format!("print( {} )", expr)
    }
}
