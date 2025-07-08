//! Defines the `SemanticError` enum and related error reporting utilities.
//!
//! `SemanticError` encapsulates all types of semantic-level errors encountered during
//! semantic analysis, such as type mismatches, undefined identifiers, or misuse of operators.

use crate::tokens::{OperatorToken, Span};
use crate::types_tree::tree_node::TypeNode;

/// Represents all the possible semantic errors that can be encountered during analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    /// Division by zero attempted.
    DivisionByZero(Span),

    /// Use of an identifier that has not been declared.
    UndefinedIdentifier(String, Span),

    /// Condition expression is of invalid type (must be boolean).
    InvalidConditionType(TypeNode, Span),

    /// Invalid binary operation between two types.
    InvalidBinaryOperation(TypeNode, TypeNode, OperatorToken, Span),

    /// Invalid unary operation for a given type.
    InvalidUnaryOperation(TypeNode, OperatorToken, Span),

    /// A function with the same name has already been defined.
    RedefinitionOfFunction(String, Span),

    /// A function is used without being declared.
    UndeclaredFunction(String, Span),

    /// Catch-all for unknown errors.
    UnknownError(String, Span),

    /// A function is called with incorrect number of arguments.
    InvalidArgumentsCount(usize, usize, String, Span),

    /// A type argument does not match the expected type.
    InvalidTypeArgument(String, String, String, usize, String, Span),

    /// The return value of a function body does not match the declared return type.
    InvalidFunctionReturn(TypeNode, TypeNode, String, Span),

    /// A variable is declared more than once in the same scope.
    RedefinitionOfVariable(String, Span),

    /// A referenced type does not exist.
    UndefinedType(String, Span),

    /// A parameter name is repeated in the same function/type definition.
    ParamNameAlreadyExist(String, String, String, Span),

    /// A type with the same name is already defined.
    RedefinitionOfType(String, Span),

    /// A cycle was detected in the type inheritance hierarchy.
    CycleDetected(String, Span),

    /// Wrong number of arguments passed to a type definition.
    InvalidTypeArgumentCount(usize, usize, String, Span),

    /// A method was accessed that does not exist on the given type.
    InvalidTypeFunctionAccess(String, String, Span),

    /// Attempted access to a private type property.
    InvalidTypePropertyAccess(String, String, Span),

    /// Property does not exist in the given type.
    InvalidTypeProperty(String, String, Span),

    /// Attempted to print an unsupported value type.
    InvalidPrint(String, Span),

    /// Invalid iterable passed to a `for` loop (should be `range()`).
    InvalidIterable(String, usize, Span),
}

impl SemanticError {
    /// Returns a human-readable message describing the error.
    pub fn message(&self) -> String {
        match self {
            SemanticError::DivisionByZero(_) => "Division by zero is not allowed".to_string(),
            SemanticError::UndefinedIdentifier(id, _) => {
                format!("Undefined identifier: {id}")
            }
            SemanticError::InvalidConditionType(t, _) => {
                format!("Invalid condition type: {}", t.type_name)
            }
            SemanticError::InvalidBinaryOperation(l, r, op, _) => format!(
                "Invalid binary operation between types {} and {} with operator {}",
                l.type_name, r.type_name, op
            ),
            SemanticError::InvalidUnaryOperation(t, op, _) => format!(
                "Invalid unary operation on type {} with operator {}",
                t.type_name, op
            ),
            SemanticError::RedefinitionOfFunction(name, _) => {
                format!("Function '{name}' is already defined")
            }
            SemanticError::UndeclaredFunction(name, _) => {
                format!("Function '{name}' is not defined")
            }
            SemanticError::InvalidArgumentsCount(found, expected, fname, _) => {
                format!("Function '{fname}' expects {expected} arguments, found {found}")
            }
            SemanticError::InvalidTypeArgument(_, found, expected, pos, stmt_name, _) => {
                format!(
                    "{stmt_name}: Argument {} should be {expected}, found {found}",
                    pos + 1
                )
            }
            SemanticError::InvalidFunctionReturn(body, ret, fname, _) => format!(
                "Function '{fname}' should return {}, found {}",
                ret.type_name, body.type_name
            ),
            SemanticError::RedefinitionOfVariable(var, _) => {
                format!("Variable '{var}' is already defined")
            }
            SemanticError::UndefinedType(ty, _) => {
                format!("Type '{ty}' is not defined")
            }
            SemanticError::ParamNameAlreadyExist(param, stmt_name, kind, _) => {
                format!("Duplicate parameter '{param}' in {kind} '{stmt_name}'")
            }
            SemanticError::RedefinitionOfType(ty, _) => {
                format!("Type '{ty}' is already defined")
            }
            SemanticError::CycleDetected(node, _) => {
                format!("Type dependency cycle detected: {node}")
            }
            SemanticError::InvalidTypeArgumentCount(found, expected, ty, _) => {
                format!("Type '{ty}' expects {expected} arguments, found {found}")
            }
            SemanticError::InvalidTypeFunctionAccess(ty, fn_name, _) => {
                format!("Type '{ty}' has no method '{fn_name}'")
            }
            SemanticError::InvalidTypePropertyAccess(ty, prop, _) => {
                format!("Property '{prop}' of type '{ty}' is private")
            }
            SemanticError::InvalidTypeProperty(ty, prop, _) => {
                format!("Type '{ty}' has no property '{prop}'")
            }
            SemanticError::InvalidPrint(ty, _) => {
                format!("Cannot print values of type '{ty}'")
            }
            SemanticError::InvalidIterable(fn_name, cnt, _) => {
                format!("For loops require range() function, found '{fn_name}({cnt} arguments)'")
            }
            SemanticError::UnknownError(msg, _) => msg.clone(),
        }
    }

    /// Returns the source `Span` where the error occurred.
    fn span(&self) -> &Span {
        match self {
            SemanticError::DivisionByZero(sp)
            | SemanticError::UndefinedIdentifier(_, sp)
            | SemanticError::InvalidConditionType(_, sp)
            | SemanticError::InvalidBinaryOperation(_, _, _, sp)
            | SemanticError::InvalidUnaryOperation(_, _, sp)
            | SemanticError::RedefinitionOfFunction(_, sp)
            | SemanticError::UndeclaredFunction(_, sp)
            | SemanticError::UnknownError(_, sp)
            | SemanticError::InvalidArgumentsCount(_, _, _, sp)
            | SemanticError::InvalidTypeArgument(_, _, _, _, _, sp)
            | SemanticError::InvalidFunctionReturn(_, _, _, sp)
            | SemanticError::RedefinitionOfVariable(_, sp)
            | SemanticError::UndefinedType(_, sp)
            | SemanticError::ParamNameAlreadyExist(_, _, _, sp)
            | SemanticError::RedefinitionOfType(_, sp)
            | SemanticError::CycleDetected(_, sp)
            | SemanticError::InvalidTypeArgumentCount(_, _, _, sp)
            | SemanticError::InvalidTypeFunctionAccess(_, _, sp)
            | SemanticError::InvalidTypePropertyAccess(_, _, sp)
            | SemanticError::InvalidTypeProperty(_, _, sp)
            | SemanticError::InvalidPrint(_, sp)
            | SemanticError::InvalidIterable(_, _, sp) => sp,
        }
    }

    /// Constructs a formatted report for the error using the source code.
    ///
    /// # Arguments
    /// * `input` - The full source input.
    /// * `missplacement` - Line adjustment (e.g. 1 to hide internal offset).
    pub fn report(&self, input: &str) -> String {
        let span = self.span();
        let (line, col, line_str, _) = get_line_context(input, span.start, 0);
        let caret = build_caret_point(col);

        let message = self.message();
        let location = format!("(line {line}, column {col})");

        format!(
            "\x1b[31mError {location}: {message}\n  {}\n  {}\x1b[0m",
            line_str, caret
        )
    }
}

/// Retrieves line and column number from a byte offset, for error context.
fn get_line_context(
    input: &str,
    offset: usize,
    missplacement: i32,
) -> (usize, usize, String, usize) {
    if input.is_empty() {
        return (1, 1, String::new(), 0);
    }
    let mut line_start = 0;
    let mut line_number = 1;
    for (idx, c) in input.char_indices() {
        if idx > offset {
            break;
        }
        if c == '\n' {
            line_number += 1;
            line_start = idx + 1;
        }
    }
    let rest = &input[line_start..];
    let line_end = rest
        .find('\n')
        .map(|p| line_start + p)
        .unwrap_or(input.len());
    let line_str = input[line_start..line_end].to_string();

    let byte_in_line = offset.saturating_sub(line_start);
    let chars_before = input[line_start..line_start + byte_in_line].chars().count();
    let column = chars_before + 1;
    let adjusted_line = (line_number as i32 - missplacement).max(1) as usize;

    (adjusted_line, column, line_str, line_start)
}

/// Builds a string that visually points to a column with a caret (`^`) for error reporting.
fn build_caret_point(col: usize) -> String {
    " ".repeat(col.saturating_sub(1)) + "^"
}
