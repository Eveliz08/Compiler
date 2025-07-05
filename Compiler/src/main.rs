use std::fs;

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


use crate::parser::ProgramParser;

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
    match parser.parse(&input) {
        Ok(ast) => {
            println!("Parsing exitoso!");
            // Aquí puedes procesar el AST
        }
        Err(errors) => {
            println!("Errores de parsing:");
            for error in errors {
                println!("{}", error);
            }
        }
    }

}
