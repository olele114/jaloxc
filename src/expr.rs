/// Defines the abstract syntax tree (AST) for the Lox language expressions.
///
/// This module implements the Visitor pattern for traversing expression trees.
/// Expressions include literals, unary operations, binary operations, and grouping.
///
/// # Examples
/// ```
/// use jaloxc::ast::{Expr, LiteralValue};
/// use jaloxc::token::Token;
/// use jaloxc::token::TokenType::*;
///
/// // Create a literal expression: 42
/// let literal = Expr::Literal(LiteralValue::Number(42.0));
///
/// // Create a unary expression: -42
/// let unary = Expr::Unary {
///     operator: Token::new(Minus, "-".to_string(), None, 1),
///     right: Box::new(literal),
/// };
///
/// // Create a grouping expression: (-42)
/// let grouping = Expr::Grouping(Box::new(unary));
/// ```
pub mod expr {
    use crate::token::{Token, TokenType
    };

    /// Represents any expression in the Lox language.
    ///
    /// Expressions can be literals, unary operations, binary operations, or groupings.
    /// This enum implements the Visitor pattern through the `accept` method.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Expr {
        /// Binary operation expression (e.g., 1 + 2)
        Binary {
            /// Left operand expression
            left: Box<Expr>,

            /// Operator token (e.g., Plus, Minus, Star, etc.)
            operator: Token,

            /// Right operand expression
            right: Box<Expr>,
        },

        /// Grouping expression (e.g., (1 + 2))
        Grouping {
            /// The expression inside the parentheses
            expression: Box<Expr>,
        },

        /// Literal value expression (e.g., 42, "hello", true, nil)
        Literal {
            /// The literal value
            value: LiteralValue
        },

        /// Unary operation expression (e.g., -42, !false)
        Unary {
            /// Operator token (e.g., Minus, Bang)
            operator: Token,
            
            /// Right operand expression
            right: Box<Expr>,
        }
    }

    /// Represents possible literal values in expressions
    #[derive(Debug, Clone, PartialEq)]
    pub enum LiteralValue {
        /// Floating-point number (e.g., 123, 123.45)
        Number(f64),
        
        /// String value (e.g., "hello")
        String(String),

        /// Boolean value (true or false)
        Bool(bool),

        /// Nil value
        Nil,
    }

    /// Defines the Visitor trait for expression traversal
    ///
    /// Implement this trait to process different expression types.
    /// Each visit method corresponds to a specific expression variant.
    pub trait Visitor<T> {
        /// Processes a Binary expression
        fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
        /// Processes a Grouping expression
        fn visit_grouping(&mut self, expression: &Expr) -> T;

        /// Processes a Literal expression 
        fn visit_literal(&mut self, value: &LiteralValue) -> T;

        /// Processes a Unary expression
        fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
    }

    impl Expr {
        /// Accepts a visitor to traverse the expression tree
        ///
        /// This method implements the Visitor pattern, dispatching to the
        /// appropriate visitor method based on the expression type.
        ///
        /// # Arguments
        /// * `visitor` - The visitor instance to process the expression
        ///
        /// # Returns
        /// The result of the visitor operation 
        pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
            match self {
                Expr::Binary { left, operator, right } => {
                    visitor.visit_binary(left, operator, right)
                }
                Expr::Grouping { expression } => {
                    visitor.visit_grouping(expression)
                }
                Expr::Literal { value } => {
                    visitor.visit_literal(value)
                }
                Expr::Unary { operator, right } => {
                    visitor.visit_unary(operator, right)
                }
            }
        }

        /// Creates a new Binary expression
        ///
        /// # Arguments
        /// * `left` - Left operand expression
        /// * `operator` - Operator token
        /// * `right` - Right operand expression
        ///
        /// # Returns
        /// Binary expression instance
        pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
            Expr::Binary { 
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }

        /// Creates a new Grouping expression
        ///
        /// # Arguments
        /// * `expression` - Expression to group
        ///
        /// # Returns
        /// Grouping expression instance
        pub fn grouping(expression: Expr) -> Self {
            Expr::Grouping { 
                expression: Box::new(expression),
            }
        } 

        /// Creates a new Literal expression
        ///
        /// # Arguments
        /// * `value` - Literal value
        ///
        /// # Returns
        /// Literal expression instance
        pub fn literal(value: LiteralValue) -> Self {
            Expr::Literal { value }
        }

        /// Creates a new Unary expression
        ///
        /// # Arguments
        /// * `operator` - Operator token
        /// * `right` - Right operand expression
        ///
        /// # Returns
        /// Unary expression instance
        pub fn unary(operator: Token, right: Expr) -> Self {
            Expr::Unary { 
                operator,
                right: Box::new(right),
            }
        }
    }

    impl std::fmt::Display for Expr {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Expr::Binary { left, operator, right } => {
                    write!(f, "({} {} {})", operator.lexeme, left, right)
                }
                Expr::Grouping { expression } => {
                    write!(f, "(group {})", expression)
                }
                Expr::Literal { value } => match value {
                    LiteralValue::Number(n) => write!(f, "{}", n),
                    LiteralValue::String(s) => write!(f, "\"{}\"", s),
                    LiteralValue::Bool(b)=> write!(f, "{}", b),
                    LiteralValue::Nil => write!(f, "nil"),
                },
                Expr::Unary { operator, right } => {
                    write!(f, "{} {}", operator.lexeme, right)
                }
            }
        }
    }
}

