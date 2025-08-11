use crate::{
    errors::standard_error::StandardError,
    lexing::{position::Position, token::Token, token_type::TokenType},
    parsing::page::Page,
    parsing::parse_result::ParseResult,
};
use std::sync::Arc;

pub struct Parser {
    pub tokens: Arc<[Token]>,
    pub token_index: isize,
    pub current_token: Option<Token>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        let mut parser = Self {
            tokens: Arc::from(tokens),
            token_index: -1,
            current_token: None,
        };
        parser.advance();

        parser
    }

    fn advance(&mut self) -> Option<Token> {
        self.token_index += 1;
        self.update_current_token();

        self.current_token.clone()
    }

    fn reverse(&mut self, amount: usize) -> Option<Token> {
        self.token_index -= amount as isize;
        self.update_current_token();

        self.current_token.clone()
    }

    fn update_current_token(&mut self) {
        if self.token_index >= 0 && self.token_index < self.tokens.len() as isize {
            self.current_token = Some(self.tokens[self.token_index as usize].clone());
        }
    }

    fn current_token_copy(&mut self) -> Token {
        self.current_token.as_ref().unwrap().clone()
    }

    fn current_token_ref(&mut self) -> &Token {
        self.current_token.as_ref().unwrap()
    }

    fn current_pos_start(&self) -> Position {
        self.current_token
            .as_ref()
            .unwrap()
            .pos_start
            .as_ref()
            .unwrap()
            .clone()
    }

    fn current_pos_end(&self) -> Position {
        self.current_token
            .as_ref()
            .unwrap()
            .pos_end
            .as_ref()
            .unwrap()
            .clone()
    }

    fn current_pos_range(&self) -> (Position, Position) {
        (
            self.current_token
                .as_ref()
                .unwrap()
                .pos_start
                .as_ref()
                .unwrap()
                .clone(),
            self.current_token
                .as_ref()
                .unwrap()
                .pos_end
                .as_ref()
                .unwrap()
                .clone(),
        )
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut parse_result = self.statements();

        if parse_result.error.is_some() && self.current_token_copy().token_type != TokenType::TT_EOF
        {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword",
                self.current_pos_start(),
                self.current_pos_end(),
                None,
            )));
        }

        parse_result
    }

    fn expr(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let pos_start = self.current_pos_start();

        if self
            .current_token_ref()
            .matches(TokenType::TT_KEYWORD, "page")
        {
            parse_result.register_advancement();
            self.advance();

            let pos_end = self.current_pos_end();

            if self.current_token_copy().token_type != TokenType::TT_LBRACKET {
                return parse_result.failure(Some(StandardError::new(
                    "expected '{'",
                    pos_start,
                    pos_end,
                    Some("add a '{' to define the fields of the page"),
                )));
            }

            parse_result.register_advancement();
            self.advance();

            let allowed_fields = ["name", "path", "contents"];
            let mut name_val: Option<String> = None;
            let mut path_val: Option<String> = None;
            let mut contents_val: Option<String> = None;
            let mut subpages: Vec<Page> = Vec::new();

            while self.current_token_ref().token_type != TokenType::TT_RBRACKET {
                let field_token = self.current_token_copy();

                if field_token.token_type != TokenType::TT_IDENTIFIER {
                    return parse_result.failure(Some(StandardError::new(
                        "expected field 'name', 'path', or 'contents'",
                        pos_start,
                        pos_end,
                        None,
                    )));
                }

                let field_name = field_token.value.clone().unwrap();

                if !allowed_fields.contains(&field_name.as_str()) {
                    return parse_result.failure(Some(StandardError::new(
                        "invalid page field",
                        pos_start,
                        pos_end,
                        Some("fields are 'name', 'path', and 'contents'"),
                    )));
                }

                parse_result.register_advancement();
                self.advance();

                if self.current_token_ref().token_type != TokenType::TT_COLON {
                    return parse_result.failure(Some(StandardError::new(
                        "missing ':'",
                        self.current_pos_start(),
                        self.current_pos_end(),
                        Some("add a colon to specify the value of the field"),
                    )));
                }

                parse_result.register_advancement();
                self.advance();

                if self.current_token_ref().token_type != TokenType::TT_STR {
                    return parse_result.failure(Some(StandardError::new(
                        "expected string value",
                        self.current_pos_start(),
                        self.current_pos_end(),
                        Some("the value of a field must be a string"),
                    )));
                }

                let value = self.current_token_copy().value.unwrap();

                parse_result.register_advancement();
                self.advance();

                match field_name.as_str() {
                    "name" => name_val = Some(value),
                    "path" => path_val = Some(value),
                    "contents" => contents_val = Some(value),
                    _ => {}
                }

                if self
                    .current_token_ref()
                    .matches(TokenType::TT_KEYWORD, "page")
                {
                    let subpage = parse_result.register(self.expr());

                    if parse_result.error.is_some() {
                        return parse_result;
                    }

                    subpages.push(subpage.unwrap());

                    continue;
                }
            }

            if self.current_token_ref().token_type != TokenType::TT_RBRACKET {
                return parse_result.failure(Some(StandardError::new(
                    "expected '}' at end of page definition",
                    self.current_pos_start(),
                    self.current_pos_end(),
                    None,
                )));
            }

            parse_result.register_advancement();
            self.advance();

            if name_val.is_none() || path_val.is_none() || contents_val.is_none() {
                return parse_result.failure(Some(StandardError::new(
                    "missing one or more required fields",
                    pos_start,
                    pos_end,
                    Some("add the following required fields 'name', 'path', and 'contents'"),
                )));
            }

            return parse_result.success(Some(Page::new(
                &name_val.unwrap(),
                &path_val.unwrap(),
                &contents_val.unwrap(),
                subpages,
            )));
        }

        parse_result.failure(Some(StandardError::new(
            "unkown keyword",
            pos_start,
            self.current_pos_end(),
            None,
        )))
    }

    fn statement(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let pos_start = self.current_pos_start();

        let expr = parse_result.register(self.expr());

        if parse_result.error.is_some() {
            return parse_result.failure(Some(StandardError::new(
                "expected keyword, object, function, expression",
                pos_start,
                self.current_pos_end(),
                None,
            )));
        }

        parse_result.success(expr)
    }

    fn statements(&mut self) -> ParseResult {
        let mut parse_result = ParseResult::new();
        let mut statements: Vec<Page> = Vec::new();
        let pos_start = self.current_pos_start();

        if self.current_token_ref().token_type == TokenType::TT_EOF {
            return parse_result.success(None);
        }

        let statement = parse_result.register(self.statement());

        if parse_result.error.is_some() {
            return parse_result;
        }

        statements.push(statement.unwrap());

        loop {
            match self.current_token_ref().token_type {
                TokenType::TT_EOF | TokenType::TT_RBRACKET => break,
                _ => {}
            }

            let statement = parse_result.register(self.statement());

            if parse_result.error.is_some() {
                return parse_result;
            }

            statements.push(statement.unwrap());
        }

        parse_result.success(Some(Page::new("", "", "", statements)))
    }
}
