use std::iter::FusedIterator;
use std::str;

use crate::lox;
use crate::token::{*, TokenKind::*};

macro_rules! return_if_err {
    ($result: expr, $line: expr) => {
        if let Err(e) = $result {
            let error_kind = lox::ErrorKind::from(e);
            return Err(lox::Error::new($line, error_kind));
        }
    }
}

const KEYWORDS: phf::Map<&str, TokenKind> = phf_map! {
    "and"    => AND,
    "class"  => CLASS,
    "else"   => ELSE,
    "false"  => FALSE,
    "for"    => FOR,
    "fun"    => FUN,
    "if"     => IF,
    "nil"    => NIL,
    "or"     => OR,
    "print"  => PRINT,
    "return" => RETURN,
    "super"  => SUPER,
    "this"   => THIS,
    "true"   => TRUE,
    "var"    => VAR,
    "while"  => WHILE,
};

pub struct Scanner {
    source:  Vec<u8>,
    tokens:  Vec<Token>,
    start:   usize,
    current: usize,
    line:    usize,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        Self {
            source,
            tokens:  Vec::new(),
            start:   0,
            current: 0,
            line:    1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.len() == 0
    }

    fn add_token(&mut self, kind: TokenKind) -> lox::Result<()> {
        let Scanner {
            ref source,
            start,
            current,
            line,
            ..
        } = *self;
        let _text = str::from_utf8(&source[start..current]);
        return_if_err!(_text, line);
        let Scanner { ref mut tokens, .. } = *self;
        let text = _text.unwrap().to_string();
        tokens.push(Token::new(kind, text, None, line));
        Ok(())
    }

    fn add_token_with_value(&mut self,
                            kind: TokenKind,
                            literal: Literal) -> lox::Result<()> {
        let Scanner {
            ref source,
            start,
            current,
            line,
            ..
        } = *self;
        let _text = str::from_utf8(&source[start..current]);
        return_if_err!(_text, line);
        let Scanner { ref mut tokens, .. } = *self;
        let text = _text.unwrap().to_string();
        tokens.push(Token::new(kind, text, Some(literal), line));
        Ok(())
    }

    fn matches(&mut self, expected: u8) -> bool {
        let Scanner {
            ref source,
            current,
            ..
        } = *self;
        if self.is_at_end() || (source[current] != expected) {
            false
        } else {
            let Scanner { ref mut current, .. } = *self;
            *current += 1;
            true
        }
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.source.get(self.current + 1).copied()
    }

    fn string(&mut self) -> lox::Result<()> {
        while self.peek() != Some(b'"') && !self.is_at_end() {
            if self.peek() == Some(b'\n') {
                let Scanner { ref mut line, .. } = *self;
                *line += 1;
            }
            self.next();
        }

        if self.is_at_end() {
            let Scanner { line, .. } = *self;
            return Err(lox::Error::new(line,
                                       lox::ErrorKind::UnterminatedString));
        }

        // The closing ".
        self.next();

        let Scanner {
            ref source,
            line,
            start,
            current,
            ..
        } = *self;

        // Trim the surrounding quotes.
        let _value = str::from_utf8(&source[(start + 1)..(current - 1)]);
        return_if_err!(_value, line);
        let value = _value.unwrap().to_string();
        match self.add_token_with_value(STRING, Literal::String(value)) {
            Ok(_)  => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn number(&mut self) -> lox::Result<()> {
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.next();
        }

        // Look for a fractional part.
        if self.peek() == Some(b'.')
           && self.peek_next().map_or(false, |c| c.is_ascii_digit())
        {
            // Consume the "."
            self.next();

            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.next();
            }
        }

        let Scanner {
            ref source,
            line,
            start,
            current,
            ..
        } = *self;

        let _token_str = str::from_utf8(&source[start..current]);
        return_if_err!(_token_str, line);
        let token_str = _token_str.unwrap();
        let _value = token_str.parse::<f64>();
        return_if_err!(_value, line);
        let value = _value.unwrap();

        match self.add_token_with_value(NUMBER, Literal::Number(value)) {
            Ok(_)  => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn identifier(&mut self) -> lox::Result<()> {
        while self.peek().map_or(false, |c| match c {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_' => true,
            _ => false,
        }) { self.next(); }

        let Scanner { ref source, line, start, current, .. } = *self;

        // See if the identifier is a reserved word.
        let _text = str::from_utf8(&source[start..current]);
        return_if_err!(_text, line);
        let text = _text.unwrap();
        let kind = match KEYWORDS.get(text) {
            Some(&x) => x,
            None => IDENTIFIER,
        };
        match self.add_token(kind) {
            Ok(_)  => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn scan_token(&mut self) -> lox::Result<()> {
        let mut result = Ok(());
        let c = self.next().unwrap();
        let token = match c {
            b'(' => LEFT_PAREN,
            b')' => RIGHT_PAREN,
            b'{' => LEFT_BRACE,
            b'}' => RIGHT_BRACE,
            b',' => COMMA,
            b'.' => DOT,
            b'-' => MINUS,
            b'+' => PLUS,
            b';' => SEMICOLON,
            b'*' => STAR,
            b'!' => if self.matches(b'=') { BANG_EQUAL } else { BANG },
            b'=' => if self.matches(b'=') { EQUAL_EQUAL } else { EQUAL },
            b'<' => if self.matches(b'=') { LESS_EQUAL } else { LESS },
            b'>' => if self.matches(b'=') { GREATER_EQUAL } else { GREATER },
            b'/' => {
                if self.matches(b'/') {
                    // A comment goes until the end of the line.
                    while self.peek() != Some(b'\n') && !self.is_at_end() {
                        self.next();
                    }
                    COMMENT
                } else {
                    SLASH
                }
            }
            b' ' | b'\r' | b'\t' => WHITESPACE,
            b'\n' => NEWLINE,
            b'"' => STRING,
            b'0'..=b'9' => NUMBER,
            b'A'..=b'Z' | b'a'..=b'z' => IDENTIFIER,
            _   => {
                let Scanner { line, .. } = *self;
                result = Err(
                    lox::Error::new(line, lox::ErrorKind::UnexpectedCharacter)
                );
                INVALID_TOKEN
            },
        };

        match token {
            COMMENT | WHITESPACE | INVALID_TOKEN => (),
            STRING => { self.string()?; },
            NUMBER => { self.number()?; },
            IDENTIFIER => { self.identifier()?; },
            NEWLINE => {
                let Scanner { ref mut line, .. } = *self;
                *line += 1;
            },
            _ => { self.add_token(token)?; },
        }
        result
    }

    pub fn scan_tokens(mut self) -> lox::Result<Vec<Token>> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            let Scanner { ref mut start, current, .. } = self;
            *start = current;
            self.scan_token()?;
        }

        let Scanner { ref mut tokens, line, .. } = self;
        tokens.push(Token::new(EOF, String::from(""), None, line));
        Ok(self.tokens)
    }
}

impl Iterator for Scanner {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let Scanner { ref source, current, .. } = *self;
        let ret = source.get(current).copied();
        let Scanner { ref mut current, .. } = *self;
        *current += 1;
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.source.len() as isize - self.current as isize;
        let size: usize = if size > 0 { size as usize } else { 0 };
        (size, Some(size))
    }
}

impl ExactSizeIterator for Scanner {}

impl FusedIterator for Scanner {}
