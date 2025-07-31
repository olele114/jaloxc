/// Defines token types and structures for the Lox language interpreter.
/// 
/// This module contains the `TokenType` enum representing all valid token types,
/// the `Token` struct representing a scanned token, and the `Literal` enum for
/// representing different literal value types.
use std::fmt;

/// All possible token types in the Lox language.
/// 
/// Categorized into single-character tokens, multi-character tokens,
/// literals, keywords, and the special EOF marker.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    
    /// One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    /// Literal value tokens
    Identifier, String, Number,
    
    /// Keyword tokens
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    
    /// End-of-file marker
    Eof,
}

/// Represents a token scanned from source code.
/// 
/// Contains information about the token's type, the original lexeme,
/// any literal value it represents, and its line location in source.
#[derive(Debug, Clone)]
pub struct Token {
    /// The type of token
    pub token_type: TokenType,
    
    /// The original text as it appeared in source code
    pub lexeme: String,
    
    /// The interpreted value for literals (numbers, strings, etc.)
    pub literal: Option<Literal>,
    
    /// The source line number where this token was found
    pub line: usize,
}

/// Represents literal values in Lox source code.
/// 
/// Can be a number, string, boolean, or nil value.
#[derive(Debug, Clone)]
pub enum Literal {
    /// Floating-point number literal
    Number(f64),
    
    /// String literal
    Str(String),
    
    /// Boolean literal (true or false)
    Bool(bool),
    
    /// Nil literal
    Nil,
}

impl Token {
    /// Creates a new token instance.
    ///
    /// # Arguments
    /// * `token_type` - The type of token
    /// * `lexeme` - Original source text
    /// * `literal` - Optional literal value
    /// * `line` - Source line number
    ///
    /// # Returns
    /// New Token instance
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self { token_type, lexeme, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

