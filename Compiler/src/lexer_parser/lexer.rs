use regex::Regex;
use std::str::CharIndices;
use super::tokens::{Token, Span, KeywordToken, OperatorToken, DelimiterToken};

pub struct Lexer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    regex_cache: RegexCache,
}

struct RegexCache {
    identifier: Regex,
    number: Regex,
    string: Regex,
    whitespace: Regex,
}

impl RegexCache {
    fn new() -> Self {
        RegexCache {
            identifier: Regex::new(r"^[A-Za-z][A-Za-z_0-9]*").unwrap(),
            number: Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap(),
            string: Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(),
            whitespace: Regex::new(r"^[ \t\n\r]+").unwrap(),
        }
    }
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str) -> Self {
        let mut lexer = Lexer {
            text,
            chars: text.char_indices(),
            lookahead: None,
            regex_cache: RegexCache::new(),
        };
        lexer.bump();
        lexer
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn skip_whitespace(&mut self) {
        if let Some((pos, _)) = self.lookahead {
            if let Some(m) = self.regex_cache.whitespace.find(&self.text[pos..]) {
                for _ in 0..m.end() {
                    self.bump();
                }
            }
        }
    }

    pub fn next_token(&mut self) -> Option<(Token, Span)> {
        self.skip_whitespace();
        
        let (start, c) = self.lookahead?;
        
        // Verificar tokens de un solo carácter
        let single_char_token = match c {
            '(' => Some(Token::LParen(DelimiterToken::LPAREN)),
            ')' => Some(Token::RParen(DelimiterToken::RPAREN)),
            '{' => Some(Token::LBrace(DelimiterToken::LBRACE)),
            '}' => Some(Token::RBrace(DelimiterToken::RBRACE)),
            ';' => Some(Token::Semicolon(DelimiterToken::SEMICOLON)),
            ',' => Some(Token::Comma(DelimiterToken::COMMA)),
            ':' => Some(Token::Colon(DelimiterToken::COLON)),
            '.' => Some(Token::DotOp(OperatorToken::DOT)),
            '+' => Some(Token::Plus(OperatorToken::PLUS)),
            '-' => Some(Token::Minus(OperatorToken::MINUS)),
            '*' => Some(Token::Star(OperatorToken::MUL)),
            '/' => Some(Token::Slash(OperatorToken::DIV)),
            '%' => Some(Token::Mod(OperatorToken::MOD)),
            '^' => Some(Token::PowOp(OperatorToken::POW)),
            '!' => Some(Token::Not(OperatorToken::NOT)),
            _ => None,
        };

        if let Some(token) = single_char_token {
            self.bump();
            return Some((token, Span::new(start, start + c.len_utf8())));
        }

        // Verificar tokens de múltiples caracteres
        let remaining_text = &self.text[start..];
        
        // Operadores de comparación
        if remaining_text.starts_with("==") {
            self.bump(); self.bump();
            return Some((Token::Equal(OperatorToken::EQ), Span::new(start, start + 2)));
        }
        if remaining_text.starts_with("!=") {
            self.bump(); self.bump();
            return Some((Token::NotEqual(OperatorToken::NEQ), Span::new(start, start + 2)));
        }
        if remaining_text.starts_with(">=") {
            self.bump(); self.bump();
            return Some((Token::GreaterEqual(OperatorToken::GTE), Span::new(start, start + 2)));
        }
        if remaining_text.starts_with("<=") {
            self.bump(); self.bump();
            return Some((Token::LessEqual(OperatorToken::LTE), Span::new(start, start + 2)));
        }
        if remaining_text.starts_with("=>") {
            self.bump(); self.bump();
            return Some((Token::Arrow(DelimiterToken::ARROW), Span::new(start, start + 2)));
        }
        if remaining_text.starts_with(":=") {
            self.bump(); self.bump();
            return Some((Token::DestructiveAssignOp(OperatorToken::DASSIGN), Span::new(start, start + 2)));
        }
        if remaining_text.starts_with("&&") {
            self.bump(); self.bump();
            return Some((Token::And(OperatorToken::AND), Span::new(start, start + 2)));
        }
        if remaining_text.starts_with("||") {
            self.bump(); self.bump();
            return Some((Token::Or(OperatorToken::OR), Span::new(start, start + 2)));
        }

        // Identificadores y palabras clave
        if let Some(m) = self.regex_cache.identifier.find(remaining_text) {
            let ident = &remaining_text[..m.end()];
            self.bump_n(m.end());
            
            let keyword = match ident {
                "function" => Some(Token::Function(KeywordToken::FUNCTION)),
                "let" => Some(Token::Let(KeywordToken::LET)),
                "in" => Some(Token::In(KeywordToken::IN)),
                "if" => Some(Token::If(KeywordToken::IF)),
                "else" => Some(Token::Else(KeywordToken::ELSE)),
                "elif" => Some(Token::Elif(KeywordToken::ELIF)),
                "while" => Some(Token::While(KeywordToken::WHILE)),
                "for" => Some(Token::For(KeywordToken::FOR)),
                "type" => Some(Token::Type(KeywordToken::TYPE)),
                "inherits" => Some(Token::Inherits(KeywordToken::INHERITS)),
                "new" => Some(Token::New(KeywordToken::NEW)),
                "print" => Some(Token::Print(KeywordToken::PRINT)),
                "true" => Some(Token::True(KeywordToken::TRUE)),
                "false" => Some(Token::False(KeywordToken::FALSE)),
                _ => None,
            };
            
            if let Some(kw) = keyword {
                return Some((kw, Span::new(start, start + m.end())));
            } else {
                return Some((Token::Identifier(ident.to_string()), Span::new(start, start + m.end())));
            }
        }

        // Números
        if let Some(m) = self.regex_cache.number.find(remaining_text) {
            self.bump_n(m.end());
            return Some((Token::Num(m.as_str().to_string()), Span::new(start, start + m.end())));
        }

        // Strings
        if let Some(m) = self.regex_cache.string.find(remaining_text) {
            self.bump_n(m.end());
            let content = &remaining_text[1..m.end()-1]; // Remove quotes
            return Some((Token::Str(content.to_string()), Span::new(start, start + m.end())));
        }

        // Carácter no reconocido
        self.bump();
        Some((Token::Unknown(c), Span::new(start, start + c.len_utf8())))
    }

    fn bump_n(&mut self, n: usize) {
        for _ in 0..n {
            self.bump();
        }
    }
}