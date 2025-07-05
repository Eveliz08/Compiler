use std::fs;
use colored::*;

#[macro_use] extern crate lalrpop_util;

// Module declarations
mod error_reporter;
mod symbol_table;
mod ast_nodes;
mod codegen;
mod intermediate;
mod lexer_parser;
mod semantic;
mod visitor;

include!(concat!(env!("OUT_DIR"), "/parser.rs"));
use crate::error_reporter::HulkParser;

use crate::visitor::printer_visitor::PrinterVisitor;
use crate::visitor::accept::Accept;



fn main() {
    println!("Compilador iniciado");

    let input = r#"
        let x = 42;
        if x >= 10 {
            print("mayor o igual a 10");
        } else {
            print("menor que 10");
        }
        // Esto es un comentario
    "#;

    let input_hulk = fs::read_to_string("../script.hulk").expect("Failed to read input file");

    // Crear el parser
    let parser = HulkParser::new();

    // Intentar parsear el código
    match parser.parse(&input_hulk) {
        Ok(ast) => {
            println!("Parsing exitoso!");
            // Imprimir el AST de manera bonita y coloreada
            print_ast(&ast, 0);
        }
        Err(errors) => {
            println!("Errores de parsing:");
            for error in errors {
                println!("{}", error);
            }
            
        }
    }

}

fn print_ast(expr: &ast_nodes::expression::Expression, indent: usize) {
    use ast_nodes::expression::Expression;
    let pad = "  ".repeat(indent);
    match expr {
        Expression::Number(n) => {
            println!("{}{}", pad, format!("Number({})", n.value).yellow());
        }
        Expression::Boolean(b) => {
            println!("{}{}", pad, format!("Boolean({})", b.value).yellow());
        }
        Expression::Str(s) => {
            println!("{}{}", pad, format!("String({:?})", s.value).yellow());
        }
        Expression::Identifier(i) => {
            println!("{}{}", pad, format!("Identifier({})", i.value).cyan());
        }
        Expression::FunctionCall(f) => {
            println!("{}{}", pad, format!("FunctionCall({})", f.function_name).magenta());
            for arg in &f.arguments {
                print_ast(arg, indent + 1);
            }
        }
        Expression::WhileLoop(w) => {
            println!("{}{}", pad, "WhileLoop".blue());
            println!("{}{}", pad, "Condition:".bold());
            print_ast(&w.condition, indent + 1);
            println!("{}{}", pad, "Body:".bold());
            print_ast(&w.body, indent + 1);
        }
        Expression::ForLoop(f) => {
            println!("{}{}", pad, format!("ForLoop(var: {})", f.variable).blue());
            println!("{}{}", pad, "Start:".bold());
            print_ast(&f.start, indent + 1);
            println!("{}{}", pad, "End:".bold());
            print_ast(&f.end, indent + 1);
            println!("{}{}", pad, "Body:".bold());
            print_ast(&f.body, indent + 1);
        }
        Expression::CodeBlock(b) => {
            println!("{}{}", pad, "CodeBlock".green());
            for expr in b.expression_list.expressions.iter() {
                print_ast(expr, indent + 1);
            }
        }
        Expression::BinaryOp(b) => {
            println!("{}{}", pad, format!("BinaryOp({:?})", b.operator).red());
            print_ast(&b.left, indent + 1);
            print_ast(&b.right, indent + 1);
        }
        Expression::UnaryOp(u) => {
            println!("{}{}", pad, format!("UnaryOp({:?})", u.operator).red());
            print_ast(&u.operand, indent + 1);
        }
        Expression::IfElse(i) => {
            println!("{}{}", pad, "IfElse".blue());
            println!("{}{}", pad, "Condition:".bold());
            print_ast(&i.condition, indent + 1);
            println!("{}{}", pad, "If:".bold());
            print_ast(&i.if_expression, indent + 1);
            for (idx, (cond, expr)) in i.elifs.iter().enumerate() {
                println!("{}{}", pad, format!("Elif #{}", idx + 1).bold());
                if let Some(c) = cond {
                    print_ast(c, indent + 2);
                }
                print_ast(expr, indent + 2);
            }
        }
        Expression::LetIn(l) => {
            println!("{}{}", pad, "LetIn".green());
            for assign in &l.assignments {
                println!("{}{}", pad, format!("Assignment: {}", assign.identifier).cyan());
                print_ast(&assign.expression, indent + 2);
            }
            println!("{}{}", pad, "Body:".bold());
            print_ast(&l.body, indent + 1);
        }
        Expression::DestructiveAssign(d) => {
            println!("{}{}", pad, "DestructiveAssign".red());
            print_ast(&d.identifier, indent + 1);
            print_ast(&d.expression, indent + 1);
        }
        Expression::TypeInstance(t) => {
            println!("{}{}", pad, format!("TypeInstance({})", t.type_name).magenta());
            for arg in &t.arguments {
                print_ast(arg, indent + 1);
            }
        }
        Expression::TypeFunctionAccess(t) => {
            println!("{}{}", pad, "TypeFunctionAccess".magenta());
            print_ast(&t.object, indent + 1);
            print_ast(&Expression::FunctionCall(*t.member.clone()), indent + 1);
        }
        Expression::TypePropAccess(t) => {
            println!("{}{}", pad, format!("TypePropAccess({})", t.member).magenta());
            print_ast(&t.object, indent + 1);
        }
        Expression::Print(p) => {
            println!("{}{}", pad, "Print".green());
            print_ast(&p.expression, indent + 1);
        }
    }
}
