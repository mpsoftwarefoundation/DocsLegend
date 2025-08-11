use crate::errors::standard_error::StandardError;
use crate::lexing::position::Position;
use crate::lexing::token::Token;
use crate::lexing::token_type::TokenType;
use crate::syntax::attributes::*;
use std::sync::Arc;

pub struct Lexer {
    pub filename: String,
    pub text: String,
    pub chars: Arc<[char]>,
    pub position: Position,
    pub current_char: Option<char>,
}

impl Lexer {
    pub fn new(filename: &str, text: String) -> Self {
        let contents = text.replace("\r\n", "\n");

        let mut lexer = Self {
            filename: filename.to_string(),
            text: contents.to_string(),
            chars: contents.chars().collect::<Vec<_>>().into(),
            position: Position::new(-1, 0, -1, filename, &contents.clone()),
            current_char: None,
        };
        lexer.advance();

        lexer
    }

    pub fn advance(&mut self) {
        self.position.advance(self.current_char);

        if self.position.index >= 0 && (self.position.index as usize) < self.chars.len() {
            self.current_char = Some(self.chars[self.position.index as usize]);
        } else {
            self.current_char = None;
        }
    }

    pub fn make_tokens(&mut self) -> Result<Vec<Token>, StandardError> {
        let mut tokens = Vec::new();

        while let Some(current_char) = self.current_char {
            let token = match current_char {
                ' ' | '\t' | '\n' => {
                    self.advance();

                    continue;
                }
                '#' => {
                    self.skip_comment();

                    continue;
                }
                c if LETTERS.contains(c) => Some(self.make_identifier()),
                '"' => match self.make_string() {
                    Ok(token) => Some(token),
                    Err(error) => return Err(error),
                },
                '{' => {
                    let token = Token::new(
                        TokenType::TT_LBRACKET,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                '}' => {
                    let token = Token::new(
                        TokenType::TT_RBRACKET,
                        None,
                        Some(self.position.clone()),
                        None,
                    );

                    self.advance();

                    Some(token)
                }
                ':' => {
                    let token =
                        Token::new(TokenType::TT_COLON, None, Some(self.position.clone()), None);
                    self.advance();
                    Some(token)
                }
                unknown_char => {
                    let pos_start = self.position.clone();

                    self.advance();

                    return Err(StandardError::new(
                        &format!("unkown character '{unknown_char}'"),
                        pos_start,
                        self.position.clone(),
                        None,
                    ));
                }
            };

            if let Some(t) = token {
                tokens.push(t);
            }
        }

        tokens.push(Token::new(
            TokenType::TT_EOF,
            None,
            Some(self.position.clone()),
            None,
        ));

        Ok(tokens)
    }

    pub fn make_identifier(&mut self) -> Token {
        let mut id_string = String::new();
        let pos_start = self.position.clone();

        while let Some(character) = self.current_char {
            if LETTERS_DIGITS.contains(character) {
                id_string.push(character);

                self.advance();
            } else {
                break;
            }
        }

        let pos_end = self.position.clone();

        let token_type = if KEYWORDS.contains(&id_string.as_str()) {
            TokenType::TT_KEYWORD
        } else {
            TokenType::TT_IDENTIFIER
        };

        Token::new(token_type, Some(id_string), Some(pos_start), Some(pos_end))
    }

    pub fn make_string(&mut self) -> Result<Token, StandardError> {
        let mut string = String::new();
        let pos_start = self.position.clone();

        self.advance();

        while let Some(character) = self.current_char {
            if character == '"' {
                break;
            }

            string.push(character);

            self.advance();
        }

        if self.current_char != Some('"') {
            return Err(StandardError::new(
                "unfinished string",
                pos_start,
                self.position.clone(),
                Some("add a '\"' at the end of the string to close it"),
            ));
        }

        self.advance();

        let pos_end = self.position.clone();

        Ok(Token::new(
            TokenType::TT_STR,
            Some(string),
            Some(pos_start),
            Some(pos_end),
        ))
    }

    pub fn skip_comment(&mut self) {
        self.advance();

        while let Some(character) = self.current_char {
            if character != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
}
