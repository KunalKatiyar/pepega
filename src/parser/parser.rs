use crate::{error_token};
use crate::lexer::token::{LiteralValue, Token, TokenType};
use crate::lexer::token::TokenType::IDENTIFIER;
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
        if self.match_check(vec![TokenType::CLASS]) {
            return self.class_statement();
        }
        if self.match_check(vec![TokenType::FUN]) {
            return self.function("function");
        }
        if self.match_check(vec![TokenType::VAR]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn statement(&mut self) -> Stmt {
        if self.match_check(vec![TokenType::IF]) {
            return self.if_statement();
        }
        if self.match_check(vec![TokenType::PRINT]) {
            return self.print_statement();
        }
        if self.match_check(vec![TokenType::RETURN]) {
            return self.return_statement();
        }
        if self.match_check(vec![TokenType::WHILE]) {
            return self.while_statement();
        }
        if self.match_check(vec![TokenType::FOR]) {
            return self.for_statement();
        }
        if self.match_check(vec![TokenType::LEFT_BRACE]) {
            return Stmt::Block { statements: self.block() };
        }
        self.expression_statement()
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.");
        statements
    }

    fn class_statement(&mut self) -> Stmt {
        let name = self.consume(TokenType::IDENTIFIER, "Expect class name.");
        self.consume(TokenType::LEFT_BRACE, "Expect '{' before class body.");
        let mut methods: Vec<Stmt> = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            methods.push(self.function("method"));
        }
        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after class body.");
        Stmt::Class { name, methods }
    }

    fn return_statement(&mut self) -> Stmt {
        let keyword = self.previous();
        let value = if !self.check(TokenType::SEMICOLON) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after return value.");
        Stmt::Return { keyword, value }
    }

    fn for_statement(&mut self) -> Stmt {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.");
        let initializer = if self.match_check(vec![TokenType::SEMICOLON]) {
            None
        } else if self.match_check(vec![TokenType::VAR]) {
            Some(self.var_declaration())
        } else {
            Some(self.expression_statement())
        };
        let condition = if !self.check(TokenType::SEMICOLON) {
            self.expression()
        } else {
            Expr::new_literal(LiteralValue::BooleanVal(true))
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.");
        let increment = if !self.check(TokenType::RIGHT_PAREN) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.");
        let mut body = Box::new(self.statement());
        if let Some(increment) = increment {
            body = Box::new(Stmt::Block { statements: vec![*body, Stmt::Expression { expression: increment }] });
        }
        body = Box::new(Stmt::While { condition, body });
        if let Some(initializer) = initializer {
            body = Box::new(Stmt::Block { statements: vec![initializer, *body] });
        }
        *body
    }
    fn while_statement(&mut self) -> Stmt {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.");
        let condition = self.expression();
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after condition.");
        let body = Box::new(self.statement());
        Stmt::While { condition, body }
    }

    fn if_statement(&mut self) -> Stmt {
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.");
        let condition = self.expression();
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after if condition.");
        let then_branch = Box::new(self.statement());
        let else_branch = if self.match_check(vec![TokenType::ELSE]) {
            Some(Box::new(self.statement()))
        } else {
            None
        };
        Stmt::If { condition, then_branch, else_branch }
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

    fn function(&mut self, kind: &str) -> Stmt {
        let name = self.consume(TokenType::IDENTIFIER, &format!("Expect {} name.", kind));
        self.consume(TokenType::LEFT_PAREN, &format!("Expect '(' after {} name.", kind));
        let mut params: Vec<Token> = Vec::new();
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                if params.len() >= 255 {
                    self.error(self.peek(), "Cannot have more than 255 parameters.");
                }
                params.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name."));
                if !self.match_check(vec![TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after parameters.");
        self.consume(TokenType::LEFT_BRACE, &format!("Expect '{{' before {} body.", kind));
        let body = self.block();
        Stmt::Function { name, params, body }
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
        let expr = self.or();
        if self.match_check(vec![TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment();
            if let Expr::Variable { name } = expr {
                return Expr::new_assign(name, value);
            } else if let Expr::Get { object, name } = expr {
                return Expr::new_set(*object, name, value);
            }
            self.error(equals, "Invalid assignment target.");
        }
        expr
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();
        while self.match_check(vec![TokenType::OR]) {
            let operator = self.previous();
            let right = self.and();
            expr = Expr::new_logical(expr, operator, right);
        }
        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();
        while self.match_check(vec![TokenType::AND]) {
            let operator = self.previous();
            let right = self.equality();
            expr = Expr::new_logical(expr, operator, right);
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
        self.call()
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        loop {
            if self.match_check(vec![TokenType::LEFT_PAREN]) {
                expr = self.finish_call(expr);
            } else if self.match_check(vec![TokenType::DOT]) {
                let tok = self.consume(IDENTIFIER, "Expect property name after '.'.");
                expr = Expr::new_get(expr, tok);
            } else {
                break;
            }
        }
        expr
    }

    fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Cannot have more than 255 arguments.");
                }
                arguments.push(self.expression());
                if !self.match_check(vec![TokenType::COMMA]) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RIGHT_PAREN, "Expect ')' after arguments.");
        Expr::new_call(callee, paren, arguments)
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
        if self.match_check(vec![TokenType::THIS]) {
            return Expr::new_this(self.previous());
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