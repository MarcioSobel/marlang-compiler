use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // literals
    Number(isize),

    // operators
    Plus,
    Minus,
    Asterisk,
    Slash,

    // separators
    Semicolon,
    OpenParen,
    CloseParen,

    // keywords
    Return,

    // other
    Invalid,
    Unknown,
    Identifier,
    Whitespace,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

#[derive(Debug)]
pub struct TextSpan {
    pub start: usize,
    pub end: usize,
    pub literal: String,
}

pub struct Lexer<'a> {
    input: &'a String,
    current_position: usize,
    chars: Peekable<Chars<'a>>,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Self {
            input,
            current_position: 0,
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(token) = self.next_token() {
            println!("{:?}", token);
            tokens.push(token);
        }

        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let start = self.current_position;
        while let Some(char) = self.peek() {
            let mut kind = TokenKind::Unknown;

            if char.is_digit(10) {
                let value = self.consume_number();
                kind = TokenKind::Number(value);
            }

            if char.is_whitespace() {
                kind = self.consume_whitespace();
            }

            if self.is_identifier_start() {
                let identifier = self.consume_identifier();
                kind = Self::get_identifier_kind(&identifier);
            }

            if kind == TokenKind::Unknown {
                kind = self.consume_symbol();
            }

            let end = self.current_position;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            return Some(Token::new(kind, span));
        }

        None
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn consume(&mut self) -> Option<char> {
        self.current_position += 1;
        self.chars.next()
    }

    fn consume_number(&mut self) -> isize {
        let mut number: isize = 0;
        while let Some(char) = self.peek() {
            if !char.is_digit(10) {
                break;
            }
            number = (number * 10) + char.to_digit(10).unwrap() as isize;
            self.consume();
        }
        number
    }

    fn consume_identifier(&mut self) -> String {
        let mut buffer = String::new();
        while let Some(char) = self.peek() {
            if char.is_whitespace() {
                break;
            }

            let char = self.consume().unwrap();
            buffer.push(char);
        }
        buffer
    }

    fn consume_whitespace(&mut self) -> TokenKind {
        while let Some(char) = self.peek() {
            if !char.is_whitespace() {
                break;
            }

            self.consume();

            if let None = self.peek() {
                return TokenKind::Eof;
            }
        }

        TokenKind::Whitespace
    }

    fn consume_symbol(&mut self) -> TokenKind {
        let char = self.consume().unwrap();
        match char {
            ';' => TokenKind::Semicolon,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            _ => TokenKind::Invalid,
        }
    }

    fn get_identifier_kind(identifier: &String) -> TokenKind {
        match identifier.as_str() {
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier,
        }
    }

    fn is_identifier_start(&mut self) -> bool {
        if let Some(char) = self.peek() {
            return char.is_alphabetic() || char == '_';
        }
        false
    }
}
