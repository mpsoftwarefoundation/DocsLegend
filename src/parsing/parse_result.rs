use crate::{errors::standard_error::StandardError, parsing::page::Page};

#[derive(Clone)]
pub struct ParseResult {
    pub error: Option<StandardError>,
    pub page: Option<Page>,
    pub last_registered_advance_count: usize,
    pub advance_count: usize,
    pub to_reverse_count: usize,
}

impl ParseResult {
    pub fn new() -> Self {
        Self {
            error: None,
            page: None,
            last_registered_advance_count: 0,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    pub fn register_advancement(&mut self) {
        self.last_registered_advance_count = 1;
        self.advance_count += 1;
    }

    pub fn register(&mut self, parse_result: ParseResult) -> Option<Page> {
        self.last_registered_advance_count = parse_result.advance_count;
        self.advance_count += parse_result.advance_count;

        if parse_result.error.is_some() {
            self.error = parse_result.error
        }

        parse_result.page
    }

    pub fn try_register(&mut self, parse_result: ParseResult) -> Option<Page> {
        if parse_result.error.is_some() {
            self.to_reverse_count = parse_result.advance_count;

            return None;
        }

        self.register(parse_result)
    }

    pub fn success(&mut self, page: Option<Page>) -> ParseResult {
        self.page = page;

        self.clone()
    }

    pub fn failure(&mut self, error: Option<StandardError>) -> ParseResult {
        if self.error.is_none() || self.last_registered_advance_count == 0 {
            self.error = error
        }

        self.clone()
    }
}
