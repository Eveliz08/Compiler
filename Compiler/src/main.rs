use colored::*;
use std::fs;

#[macro_use]
extern crate lalrpop_util;

// Module declarations
mod ast_nodes;
mod codegen;
mod error_reporter;
mod semantic_analyzer;
mod tokens;
mod types_tree;
mod visitor;
use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
include!(concat!(env!("OUT_DIR"), "/parser.rs"));
use crate::error_reporter::HulkParser;

use crate::visitor::accept::Accept;
use crate::visitor::printer_visitor::PrinterVisitor;

fn main() {
    println!("Compilador iniciado");

    let input_hulk = fs::read_to_string("../src/tests/test10.hulk").expect("Failed to read input file");

    // Crear el parser
    let parser = HulkParser::new();

    // Intentar parsear el código
    match parser.parse(&input_hulk) {
        Ok(mut ast) => {
            println!("Parsing exitoso!");
            // Imprimir el AST 
            print_ast_node(&ast, 0);
            let mut semantic_analyzer = SemanticAnalyzer::new();
            let result = semantic_analyzer.analyze(&mut ast);
            match result {
                Ok(_) => {
                    println!("Chequeo semántico exitoso!");
                    //Iniciar generacion de codigo
                    let mut codegen = codegen::generator::Generator::new();
                    codegen.generate(&mut ast);
                    println!("Código generado:");
                    println!("{}", codegen.code);
                    // Aquí podrías guardar el código generado en un archivo
                    codegen::writer::write_to_file(&codegen.code, "output.ll");
                    codegen::runner::run_llvm_ir("output.ll");

                }
                Err(errors) => {
                    println!("\x1b[31mSemantic Errors:");
                    for err in errors.iter() {
                        println!("{}", err.report(&input_hulk));
                    }
                    println!("\x1b[0m");
                    std::process::exit(3);
                }
            }
        }

        Err(parse_err) => {
            println!("\x1b[31mSyntax Error:\x1b[0m");
            for err in parse_err.iter() {
                println!("{}", err);
            }
            std::process::exit(1);
        }
    }
}

// Nueva función principal para imprimir cualquier nodo del AST
fn print_ast_node(node: &ast_nodes::program::Program, indent: usize) {
    let pad = "  ".repeat(indent);
    println!("{}Program", pad);
    for stmt in &node.statements {
        print_statement(stmt, indent + 1);
    }
}

fn print_statement(stmt: &ast_nodes::program::Statement, indent: usize) {
    use ast_nodes::program::Statement;
    let pad = "  ".repeat(indent);
    match stmt {
        Statement::StatementExpression(expr) => {
            println!("{}StatementExpression", pad);
            print_expression(expr, indent + 1);
        }
        Statement::StatementFunctionDef(func) => {
            println!("{}StatementFunctionDef", pad);
            // Aquí podrías imprimir detalles de la función si lo deseas
            // Por simplicidad, imprime el nombre
            println!("{}Function name: {}", pad, func.name);
        }
        Statement::StatementTypeDef(typ) => {
            println!("{}StatementTypeDef", pad);
            println!("{}Type name: {}", pad, typ.identifier);
        }
    }
}

// Renombrar la función original para imprimir expresiones
fn print_expression(expr: &ast_nodes::expression::Expression, indent: usize) {
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
            println!(
                "{}{}",
                pad,
                format!("FunctionCall({})", f.function_name).magenta()
            );
            for arg in &f.arguments {
                print_expression(arg, indent + 1);
            }
        }
        Expression::WhileLoop(w) => {
            println!("{}{}", pad, "WhileLoop".blue());
            println!("{}{}", pad, "Condition:".bold());
            print_expression(&w.condition, indent + 1);
            println!("{}{}", pad, "Body:".bold());
            print_expression(&w.body, indent + 1);
        }
        Expression::ForLoop(f) => {
            println!("{}{}", pad, format!("ForLoop(var: {})", f.variable).blue());
            println!("{}{}", pad, "Start:".bold());
            print_expression(&f.start, indent + 1);
            println!("{}{}", pad, "End:".bold());
            print_expression(&f.end, indent + 1);
            println!("{}{}", pad, "Body:".bold());
            print_expression(&f.body, indent + 1);
        }
        Expression::CodeBlock(b) => {
            println!("{}{}", pad, "CodeBlock".green());
            for expr in b.expression_list.expressions.iter() {
                print_expression(expr, indent + 1);
            }
        }
        Expression::BinaryOp(b) => {
            println!("{}{}", pad, format!("BinaryOp({:?})", b.operator).red());
            print_expression(&b.left, indent + 1);
            print_expression(&b.right, indent + 1);
        }
        Expression::UnaryOp(u) => {
            println!("{}{}", pad, format!("UnaryOp({:?})", u.operator).red());
            print_expression(&u.operand, indent + 1);
        }
        Expression::IfElse(i) => {
            println!("{}{}", pad, "IfElse".blue());
            println!("{}{}", pad, "Condition:".bold());
            print_expression(&i.condition, indent + 1);
            println!("{}{}", pad, "If:".bold());
            print_expression(&i.if_expression, indent + 1);
            for (idx, (cond, expr)) in i.elifs.iter().enumerate() {
                println!("{}{}", pad, format!("Elif #{}", idx + 1).bold());
                if let Some(c) = cond {
                    print_expression(c, indent + 2);
                }
                print_expression(expr, indent + 2);
            }
        }
        Expression::LetIn(l) => {
            println!("{}{}", pad, "LetIn".green());
            for assign in &l.assignments {
                println!(
                    "{}{}",
                    pad,
                    format!("Assignment: {}", assign.identifier).cyan()
                );
                print_expression(&assign.expression, indent + 2);
            }
            println!("{}{}", pad, "Body:".bold());
            print_expression(&l.body, indent + 1);
        }
        Expression::DestructiveAssign(d) => {
            println!("{}{}", pad, "DestructiveAssign".red());
            print_expression(&d.identifier, indent + 1);
            print_expression(&d.expression, indent + 1);
        }
        Expression::TypeInstance(t) => {
            println!(
                "{}{}",
                pad,
                format!("TypeInstance({})", t.type_name).magenta()
            );
            for arg in &t.arguments {
                print_expression(arg, indent + 1);
            }
        }
        Expression::TypeFunctionAccess(t) => {
            println!("{}{}", pad, "TypeFunctionAccess".magenta());
            print_expression(&t.object, indent + 1);
            print_expression(
                &ast_nodes::expression::Expression::FunctionCall((*t.member).clone()),
                indent + 1,
            );
        }
        Expression::TypePropAccess(t) => {
            println!(
                "{}{}",
                pad,
                format!("TypePropAccess({})", t.member).magenta()
            );
            print_expression(&t.object, indent + 1);
        }
        Expression::Print(p) => {
            println!("{}{}", pad, "Print".green());
            print_expression(&p.expression, indent + 1);
        }
    }
}
