/// Lexical scanner implementation for Lox.
/// 
/// Transforms source code into a sequence of tokens by scanning characters
/// and recognizing language patterns (keywords, literals, operators, etc.).
use std::collections::HashMap;
use crate::token::{Token, TokenType, Literal};

/// The lexical scanner that processes source code into tokens.
pub struct Scanner {
    /// Source code as character vector for easier indexing
    source: Vec<char>,
    
    /// List of tokens generated during scanning
    tokens: Vec<Token>,
    
    /// Start position of current lexeme being scanned
    start: usize,
    
    /// Current scanning position in source
    current: usize,
    
    /// Current line number in source
    line: usize
}

impl Scanner {
    /// Creates a new scanner for the given source string.
    ///
    /// # Arguments
    /// * `source` - The Lox source code to scan
    ///
    /// # Returns
    /// New Scanner instance initialized to start scanning
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

     /// Scans all tokens from the source code.
    ///
    /// Processes the entire source string, generating tokens until EOF is reached.
    ///
    /// # Returns
    /// Reference to the vector of scanned tokens
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        
        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            None,
            self.line
        ));
        &self.tokens
    }

    /// Processes a single token based on current scanner state.
    ///
    /// Examines the current character and dispatches to appropriate
    /// token handling methods based on character type.
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok);
            }
            '=' => {
                let tok = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tok);
            }
            '<' => {
                let tok = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok);
            }
            '>' => {
                let tok = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.block_comment();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {/* Ignore whitespace */ }
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            _ => self.error("Unexpected character"),
        }
    }
        
    /// Processes block comments, including nested comments.
    ///
    /// Handles both single-line (`//`) and multi-line (`/* */`) comments.
    /// Supports arbitrary nesting depth for multi-line comments.
    fn block_comment(&mut self) {
        let mut nesting = 1;
        while nesting > 0 && !self.is_at_end() {
            if self.peek() == '/' && self.peek_next() == '*' {
                self.advance();
                self.advance();
                nesting += 1;
            } else if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                nesting -= 1;
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.advance();
            }
        }

        if nesting > 0 {
            self.error("Unterminated block comment");
        }
    }

    /// Processes string literals.
    ///
    /// Collects characters between double quotes, handling escape sequences
    /// and tracking newlines within strings.
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string");
            return;
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        self.add_token_with_literal(
            TokenType::String,
            Some(Literal::Str(value))
        );
    }

    /// Processes numeric literals.
    ///
    /// Handles both integers and floating-point numbers with decimal points.
    /// Validates number format and converts to f64 representation.
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let num_str: String = self.source[self.start..self.current].iter().collect();
        let value = num_str.parse::<f64>().unwrap_or_else(|_|{
            self.error(&format!("Invalid number: {}", num_str));
            0.0
        });

        self.add_token_with_literal(
            TokenType::Number, 
            Some(Literal::Number(value))
        );
    }

    /// Processes identifiers and keywords.
    ///
    /// Collects alphanumeric sequences and checks against keyword table.
    /// Handles special literal values (true, false, nil) appropriately.
    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();

        let token_type = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        let literal = match token_type {
            TokenType::False => Some(Literal::Bool(false)),
            TokenType::True => Some(Literal::Bool(true)),
            TokenType::Nil => Some(Literal::Nil),
            _ => None,
        };

        self.add_token_with_literal(token_type, literal);
    }

    /// Advances the scanner by one character.
    ///
    /// # Returns
    /// The character at the current position before advancing
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    /// Checks if the next character matches expected value.
    ///
    /// If it matches, advances the scanner and returns true.
    /// Otherwise, returns false without advancing.
    ///
    /// # Arguments
    /// * `expected` - The character to match
    ///
    /// # Returns
    /// True if matched and advanced, false otherwise
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    /// Peeks at the current character without consuming it.
    ///
    /// # Returns
    /// Current character if available, null character otherwise
    fn peek(&self) -> char{
        if self.is_at_end() {'\0'} else {self.source[self.current]}
    }

    /// Peeks at the next character without consuming it.
    ///
    /// # Returns
    /// Next character if available, null character otherwise
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {'\0'} else {self.source[self.current + 1]}
    }

    /// Checks if scanner has reached end of source.
    ///
    /// # Returns
    /// True if all characters have been processed, false otherwise
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Adds a token without a literal value.
    ///
    /// # Arguments
    /// * `token_type` - The type of token to add
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    /// Adds a token with an associated literal value.
    ///
    /// # Arguments
    /// * `token_type` - The type of token to add
    /// * `literal` - Optional literal value for the token
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    /// Reports an error during scanning.
    ///
    /// Prints error message to stderr with line number context.
    ///
    /// # Arguments
    /// * `message` - Error description
    fn error(&mut self, message: &str) {
        eprintln!("[line {}] Error: {}", self.line, message);
    }
}
