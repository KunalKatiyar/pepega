use crate::{error_token};
use crate::lexer::token::{LiteralValue, Token, TokenType};
use crate::parser::expr::Expr;
use crate::parser::stmt::Stmt;
use crate::parser::stmt::Stmt::Print;

pub struct ParseError {
    pub message: String,
    pub token: Token
}

impl ParseError {
    pub fn new(message: String, token: Token) -> ParseError {
        ParseError {
            message,
            token
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    parse_errors: Vec<ParseError>
}

impl Parser {
    pub fn new (tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            parse_errors: Vec::new()
        }
    }

    pub(crate) fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        if self.parse_errors.len() > 0 {
            panic!("Cannot parse with errors.");
        }
        statements
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_check(vec![TokenType::VAR]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn statement(&mut self) -> Stmt {
        if self.match_check(vec![TokenType::PRINT]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.");
        let mut initializer = Expr::new_literal(LiteralValue::NullVal);
        if self.match_check(vec![TokenType::EQUAL]) {
            initializer = self.expression();
        }
        self.consume(TokenType::SEMICOLON, "Expect ';' after variable declaration.");
        Stmt::Var { name, initializer }
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        Print { expression: value }
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.consume(TokenType::SEMICOLON, "Expect ';' after expression.");
        Stmt::Expression { expression: expr }
    }

    fn match_check(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn expression(&mut self) -> Expr{
        self.assignment()
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.equality();
        if self.match_check(vec![TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment();
            if let Expr::Variable { name } = expr {
                return Expr::new_assign(name, value);
            }
            self.error(equals, "Invalid assignment target.");
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_check(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::new_binary(expr, operator, right);
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_check(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::new_binary(expr, operator, right);
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_check(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::new_binary(expr, operator, right);
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_check(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::new_binary(expr, operator, right);
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_check(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::new_unary(operator, right);
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_check(vec![TokenType::FALSE]) {
            return Expr::new_literal(LiteralValue::BooleanVal(false));
        }
        if self.match_check(vec![TokenType::TRUE]) {
            return Expr::new_literal(LiteralValue::BooleanVal(true));
        }
        if self.match_check(vec![TokenType::NIL]) {
            return Expr::new_literal(LiteralValue::NullVal);
        }
        if self.match_check(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Expr::new_literal(self.previous().literal.unwrap());
        }
        if self.match_check(vec![TokenType::IDENTIFIER]) {
            return Expr::new_variable(self.previous());
        }
        if self.match_check(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::new_grouping(expr);
        }
        let _error_token = self.error(self.peek(), "Expect expression.");
        Expr::new_literal(LiteralValue::NullVal)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance()
        }
        self.error(self.peek(), message)
    }

    fn add_parse_error(&mut self, error: ParseError) {
        self.parse_errors.push(error);
    }

    fn error (&mut self, token: Token, message: &str) -> Token {
        self.add_parse_error(ParseError::new(message.to_string(), token.clone()));
        let return_token = token.clone();
        error_token(token, message);
        return_token
    }


}