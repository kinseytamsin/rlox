#![allow(non_camel_case_types)]

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TokenKind {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, COMMA, DOT, MINUS, PLUS,
    SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL, GREATER, GREATER_EQUAL, LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR, PRINT, RETURN, SUPER, THIS,
    TRUE, VAR, WHILE,

    COMMENT, INVALID_TOKEN, WHITESPACE, NEWLINE,

    EOF,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind,
           lexeme: String,
           literal: Option<Literal>,
           line: usize) -> Self
    {
        Self { kind, lexeme, literal, line }
    }
}
