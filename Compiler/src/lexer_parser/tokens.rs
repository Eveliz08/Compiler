#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Function(KeywordToken),
    Let(KeywordToken),
    In(KeywordToken),
    If(KeywordToken),
    Else(KeywordToken),
    Elif(KeywordToken),
    While(KeywordToken),
    For(KeywordToken),
    Type(KeywordToken),
    Inherits(KeywordToken),
    New(KeywordToken),
    Print(KeywordToken),
    True(KeywordToken),
    False(KeywordToken),
    
    // Identifiers and literals
    Identifier(String),
    Num(String),
    Str(String),
    
    // Operators
    Plus(OperatorToken),
    Minus(OperatorToken),
    Star(OperatorToken),
    Slash(OperatorToken),
    Mod(OperatorToken),
    PowOp(OperatorToken),
    Not(OperatorToken),
    Equal(OperatorToken),
    NotEqual(OperatorToken),
    Greater(OperatorToken),
    GreaterEqual(OperatorToken),
    Less(OperatorToken),
    LessEqual(OperatorToken),
    And(OperatorToken),
    Or(OperatorToken),
    DotOp(OperatorToken),
    DestructiveAssignOp(OperatorToken),
    
    // Delimiters
    LParen(DelimiterToken),
    RParen(DelimiterToken),
    LBrace(DelimiterToken),
    RBrace(DelimiterToken),
    Semicolon(DelimiterToken),
    Comma(DelimiterToken),
    Colon(DelimiterToken),
    Arrow(DelimiterToken),
    
    Unknown(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordToken {
    FUNCTION,
    LET,
    IN,
    IF,
    ELSE,
    ELIF,
    WHILE,
    FOR,
    TYPE,
    INHERITS,
    NEW,
    PRINT,
    TRUE,
    FALSE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorToken {
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    POW,
    NOT,
    EQ,
    NEQ,
    GT,
    GTE,
    LT,
    LTE,
    AND,
    OR,
    DOT,
    ASSIGN,
    DASSIGN,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DelimiterToken {
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOLON,
    COMMA,
    COLON,
    ARROW,
}