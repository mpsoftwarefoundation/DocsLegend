#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    TT_STR,
    TT_IDENTIFIER,
    TT_KEYWORD,
    TT_LBRACKET,
    TT_RBRACKET,
    TT_LT,
    TT_GT,
    TT_COLON,
    TT_EOF,
}
