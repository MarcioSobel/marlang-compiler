use super::{
    lexer::{Token, TokenKind},
    Ast, AstBinaryOperator, AstBinaryOperatorKind, AstExpression, AstStatement,
};

pub struct Parser {
    tokens: Vec<Token>,
    current_position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens
            .iter()
            .filter(|token| token.kind != TokenKind::Whitespace)
            .map(|token| token.clone())
            .collect();

        Self {
            tokens,
            current_position: 0,
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut statements: Vec<AstStatement> = Vec::new();
        while let Some(statement) = self.parse_statement() {
            statements.push(statement);
        }

        Ast::new(statements)
    }

    fn current_token(&mut self) -> Option<&Token> {
        self.tokens.get(self.current_position)
    }

    fn peek(&mut self, offset: isize) -> Option<&Token> {
        let index = offset + self.current_position as isize;
        self.tokens.get(index as usize)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current_position += 1;
        self.peek(-1)
    }

    fn parse_statement(&mut self) -> Option<AstStatement> {
        let expression = self.parse_expression();
        if expression.is_none() {
            return None;
        }
        println!("{:?}", expression);
        Some(AstStatement::from_expression(expression.unwrap()))
    }

    fn parse_expression(&mut self) -> Option<AstExpression> {
        self.parse_binary_expression(0)
    }

    fn parse_primary_expression(&mut self) -> Option<AstExpression> {
        let token = self.consume();
        if token.is_none() {
            return None;
        }
        let token = token.unwrap();

        match token.kind {
            TokenKind::Number(value) => Some(AstExpression::from_number(value)),
            TokenKind::OpenParen => {
                let expression = self.parse_expression();
                let token = self.consume().unwrap();
                if token.kind != TokenKind::CloseParen {
                    panic!(
                        "Invalid expression: Expected ')' at char {}",
                        token.span.start
                    );
                }
                expression
            }
            _ => None,
        }
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<AstExpression> {
        let left = self.parse_primary_expression();
        if left.is_none() {
            return None;
        }
        let mut left = left.unwrap();

        while let Some(operator) = self.parse_operator_kind() {
            self.consume();

            let operator_precedence = operator.get_precedence();
            if operator_precedence < precedence {
                break;
            }

            let right = self.parse_binary_expression(operator_precedence);
            if right.is_none() {
                break;
            }

            let right = right.unwrap();
            left = AstExpression::from_binary_expression(left, right, operator);
        }

        Some(left)
    }

    fn parse_operator_kind(&mut self) -> Option<AstBinaryOperator> {
        let token = self.current_token();
        if token.is_none() {
            return None;
        }

        let token = token.unwrap();
        let kind = match token.kind {
            TokenKind::Plus => Some(AstBinaryOperatorKind::Plus),
            TokenKind::Minus => Some(AstBinaryOperatorKind::Minus),
            TokenKind::Asterisk => Some(AstBinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(AstBinaryOperatorKind::Divide),
            _ => None,
        };
        if kind.is_none() {
            return None;
        }

        let kind = kind.unwrap();
        Some(AstBinaryOperator::new(kind, token.clone()))
    }
}
