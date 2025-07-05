//! # helper_error_reporter
//!
//! Este módulo proporciona la estructura `HulkParser` y utilidades para el reporte amigable de errores de parsing en Hulk.
//!
//! ## Estructura principal
//!
//! - `HulkParser`  
//!   Wrapper sobre el parser generado por LALRPOP (`ProgramParser`), que agrega reporte de errores enriquecido y contextualizado.
//!
//! ## Métodos principales
//!
//! - `new()`  
//!   Crea una nueva instancia de `HulkParser`.
//!
//! - `parse(&self, input: &str) -> Result<Program, Vec<String>>`  
//!   Intenta parsear el código fuente. Si hay errores, devuelve una lista de mensajes de error formateados con colores, línea, columna, contexto y carets (`^`) indicando la posición exacta del error.
//!
//! ## Funciones auxiliares
//!
//! - `extract_line_info(input: &str, offset: usize) -> (usize, usize, String, usize)`  
//!   Calcula el número de línea, columna y el texto de la línea donde ocurrió el error.
//!
//! - `caret_for_point(col: usize) -> String`  
//!   Construye una cadena con espacios y un caret (`^`) para señalar la columna del error.
//!
//! - `caret_for_token(line: &str, col: usize, token: &str) -> String`  
//!   Construye una cadena con carets (`^`) subrayando el token inesperado.
//!
//! - `pretty_token(token: &str) -> String`  
//!   Limpia el token para mostrarlo de forma legible en los mensajes de error.
//!
//! ## Uso típico
//!
//! Se utiliza para parsear código fuente Hulk y reportar errores de sintaxis de forma clara y amigable, mostrando el contexto y la ubicación exacta del problema.
//!
//! ## Ejemplo de uso
//! ```rust
//! let parser = HulkParser::new();
//! match parser.parse("let x = 1") {
//!     Ok(ast) => { /* ... */ }
//!     Err(errors) => for err in errors { println!("{}", err); }
//! }
use std::collections::HashSet;

// use crate::ast_nodes::program::Program;
use lalrpop_util::ParseError;
include!(concat!(env!("OUT_DIR"), "/parser.rs"));


pub struct HulkParser {
    parser: ProgramParser,
}

impl HulkParser {
    pub fn new() -> Self {
        HulkParser {
            parser: ProgramParser::new(),
        }
    }

    fn extract_line_info(
        input: &str,
        offset: usize,
    ) -> (usize, usize, String, usize) {
        if input.is_empty() {
            return (1, 1, String::new(), 0);
        }

        let mut line_start = 0;
        let mut line_num = 1;

        for (idx, ch) in input.char_indices() {
            if idx > offset {
                break;
            }
            if ch == '\n' {
                line_num += 1;
                line_start = idx + 1;
            }
        }

        let rest = &input[line_start..];
        let line_end = match rest.find('\n') {
            Some(pos) => line_start + pos,
            None => input.len(),
        };

        let line_content = input[line_start..line_end].to_string();

        let byte_offset = offset - line_start;
        let substring = if byte_offset <= line_content.len() {
            &line_content[..byte_offset]
        } else {
            &line_content
        };
        let char_offset = substring.chars().count();
        let column = char_offset + 1;

        (line_num, column, line_content, line_start)
    }

    fn caret_for_point(col: usize) -> String {
        " ".repeat(col - 1) + "^"
    }

    fn caret_for_token(line: &str, col: usize, token: &str) -> String {
        let token_len = token.chars().count();
        let remaining = line.chars().skip(col - 1).count();
        let underline_len = token_len.min(remaining);
        let spaces = " ".repeat(col - 1);
        let carets = "^".repeat(underline_len);
        spaces + &carets
    }

    fn pretty_token(token: &str) -> String {
        token
            .replace('"', "")
            .replace('\\', "")
            .replace("r#", "")
            .replace('#', "")
    }

    pub fn parse(&self, input: &str) -> Result<Program, Vec<String>> {
        let mut issues = Vec::new();
        let result = self.parser.parse(input);

        match result {
            Ok(ast) => Ok(ast),
            Err(err) => match err {
                ParseError::InvalidToken { location } => {
                    let (line, col, line_str, _) =
                        Self::extract_line_info(input, location);
                    let caret = Self::caret_for_point(col);
                    issues.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Invalid token\n{}\n{}\x1b[0m",
                        line, col, line_str, caret
                    ));
                    Err(issues)
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let (line, col, line_str, _) =
                        Self::extract_line_info(input, location);
                    let caret = Self::caret_for_point(col);

                    let expected_clean: Vec<String> = expected
                        .iter()
                        .map(|s| Self::pretty_token(s))
                        .collect();
                    let mut unique_expected: HashSet<String> = expected_clean.into_iter().collect();
                    let mut sorted_expected: Vec<String> = unique_expected.drain().collect();
                    sorted_expected.sort();

                    issues.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Unexpected end of input. Expected one of: {}\n{}\n{}\x1b[0m",
                        line, col, sorted_expected.join(", "), line_str, caret
                    ));
                    Err(issues)
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    let (start, token_val, end) = token;
                    let token_value = &token_val.1;
                    let token_str = &input[start..end];

                    let (line, col, line_str, _) =
                        Self::extract_line_info(input, start);
                    let caret = Self::caret_for_token(&line_str, col, token_str);

                    let expected_clean: Vec<String> = expected
                        .iter()
                        .map(|s| Self::pretty_token(s))
                        .collect();
                    let mut unique_expected: HashSet<String> = expected_clean.into_iter().collect();
                    let mut sorted_expected: Vec<String> = unique_expected.drain().collect();
                    sorted_expected.sort();

                    issues.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Unexpected token `{}`. Expected one of: {}\n{}\n{}\x1b[0m",
                        line, col, token_value, sorted_expected.join(", "), line_str, caret
                    ));
                    Err(issues)
                }
                ParseError::ExtraToken { token } => {
                    let (start, token_val, end) = token;
                    let token_str = &input[start..end];

                    let (line, col, line_str, _) =
                        Self::extract_line_info(input, start);
                    let caret = Self::caret_for_token(&line_str, col, token_str);

                    issues.push(format!(
                        "\x1b[31mSyntax Error (line {}, column {}): Extra token `{}`\n{}\n{}\x1b[0m",
                        line, col, token_val.1, line_str, caret
                    ));
                    Err(issues)
                }
                ParseError::User { error } => {
                    issues.push(format!("\x1b[31mSyntax Error: {}\x1b[0m", error));
                    Err(issues)
                }
            },
        }
    }
}
