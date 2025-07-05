#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // Genera el módulo del parser

pub mod tokens;
pub mod lexer;
pub mod ast_nodes; // Tus nodos AST