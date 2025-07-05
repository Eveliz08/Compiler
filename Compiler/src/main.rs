mod semantic;
mod intermediate;
mod codegen;
mod symbol_table;
mod lexer;
mod parser;

use lexer::lexer::{Lexer};
use parser::parser::Parser;

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

    let mut lexer = Lexer::new(input);

    // Mientras haya tokens, imprime el tipo, lexema, línea y columna de cada uno
    while let Some(token) = lexer.next_token() {
        let mut parser = Parser::new(0);
        let ast = parser.parse(&mut lexer);
        println!("{:#?}", ast);
        break; // Salimos del bucle porque el parser consume los tokens
    }
}
