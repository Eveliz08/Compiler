//! Defines the core tokens and spans used by the lexer and parser.
//!
//! This module includes:
//! - `Span`: A byte range in the source code.
//! - `KeywordToken`: Reserved words in the language.
//! - `OperatorToken`: Language operators (e.g., +, ==, :=).
//! - `DelimiterToken`: Punctuation used to delimit expressions and blocks.

use std::fmt;

/// Represents a span (range) in the input source code using byte offsets.
///
/// Commonly used to track the location of tokens or AST nodes.
///
/// # Fields
/// * `start` - Starting byte offset (inclusive).
/// * `end` - Ending byte offset (exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Constructs a new `Span` from a start and end byte offset.
    ///
    /// # Arguments
    /// * `start` - Start byte offset.
    /// * `end` - End byte offset.
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

/// Represents the reserved keywords in the language.
///
/// These tokens cannot be used as identifiers.
#[derive(Debug, PartialEq, Clone)]
pub enum KeywordToken {
    PRINT,
    WHILE,
    FOR,
    ELIF,
    ELSE,
    IF,
    IN,
    LET,
    TRUE,
    FALSE,
    FUNCTION,
    INHERITS,
    TYPE,
    NEW,
}

/// Represents all possible operator tokens in the language,
/// including arithmetic, logical, and assignment operators.
#[derive(Debug, PartialEq, Clone)]
pub enum OperatorToken {
    MUL,
    DIV,
    PLUS,
    MINUS,
    MOD,
    POW,
    NEG,
    NOT,
    EQ,
    NEQ,
    GT,
    GTE,
    LT,
    LTE,
    ASSIGN,
    CONCAT,
    DASSIGN,
    OR,
    AND,
    DOT,
}

impl fmt::Display for OperatorToken {
    /// Converts an `OperatorToken` to its symbolic string representation.
    ///
    /// # Example
    /// ```
    /// use crate::tokens::OperatorToken;
    /// assert_eq!(OperatorToken::PLUS.to_string(), "+");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OperatorToken::MUL => "*",
            OperatorToken::DIV => "/",
            OperatorToken::PLUS => "+",
            OperatorToken::MINUS => "-",
            OperatorToken::MOD => "%",
            OperatorToken::POW => "^",
            OperatorToken::NEG => "-",
            OperatorToken::NOT => "!",
            OperatorToken::EQ => "==",
            OperatorToken::NEQ => "!=",
            OperatorToken::GT => ">",
            OperatorToken::GTE => ">=",
            OperatorToken::LT => "<",
            OperatorToken::LTE => "<=",
            OperatorToken::ASSIGN => "=",
            OperatorToken::CONCAT => "@",
            OperatorToken::DASSIGN => ":=",
            OperatorToken::AND => "&",
            OperatorToken::OR => "|",
            OperatorToken::DOT => ".",
        };
        write!(f, "{}", s)
    }
}

/// Represents punctuation and grouping symbols in the language.
///
/// Used for separating expressions, grouping tokens, and denoting blocks.
#[derive(Debug, PartialEq, Clone)]
pub enum DelimiterToken {
    SEMICOLON,
    COLON,
    COMMA,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    ARROW,
}
