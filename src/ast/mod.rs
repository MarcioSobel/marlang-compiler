use lexer::Token;

pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<AstStatement>,
}

#[derive(Debug)]
pub struct AstStatement {
    pub kind: AstStatementKind,
}

#[derive(Debug)]
pub enum AstStatementKind {
    Expression(AstExpression),
}

#[derive(Debug)]
pub struct AstExpression {
    pub kind: AstExpressionKind,
}

#[derive(Debug)]
pub enum AstExpressionKind {
    Number(isize),
    Binary(AstBinaryExpression),
}

#[derive(Debug)]
pub struct AstBinaryExpression {
    pub left: Box<AstExpression>,
    pub right: Box<AstExpression>,
    pub operator: AstBinaryOperator,
}

#[derive(Debug)]
pub struct AstBinaryOperator {
    pub kind: AstBinaryOperatorKind,
    pub token: Token,
}

#[derive(Debug)]
pub enum AstBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Ast {
    pub fn new(statements: Vec<AstStatement>) -> Self {
        Self { statements }
    }
}

impl AstStatement {
    pub fn from_expression(expression: AstExpression) -> Self {
        Self {
            kind: AstStatementKind::Expression(expression),
        }
    }
}

impl AstExpression {
    pub fn from_number(value: isize) -> Self {
        Self {
            kind: AstExpressionKind::Number(value),
        }
    }

    pub fn from_binary_expression(
        left: AstExpression,
        right: AstExpression,
        operator: AstBinaryOperator,
    ) -> Self {
        let binary_expression = AstBinaryExpression {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        };
        Self {
            kind: AstExpressionKind::Binary(binary_expression),
        }
    }
}

impl AstBinaryOperator {
    pub fn new(kind: AstBinaryOperatorKind, token: Token) -> Self {
        Self { kind, token }
    }

    pub fn get_precedence(&self) -> u8 {
        match self.kind {
            AstBinaryOperatorKind::Plus => 1,
            AstBinaryOperatorKind::Minus => 1,
            AstBinaryOperatorKind::Multiply => 2,
            AstBinaryOperatorKind::Divide => 2,
        }
    }
}
