// ================================================================================================
//   SYNTRA LANGUAGE — TOKENS
// ------------------------------------------------------------------------------------------------
//   File:        src/syntra_lang/tokens.rs
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Lexical token definitions for the Syntra Language. This is the first stage of the
//       Syntra → Syntra-IR → MLIR pipeline.
// ================================================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Module,
    Import,
    Fn,
    Pure,
    Class,
    Trait,
    Impl,
    Match,
    Of,
    If,
    Else,
    Return,
    Let,
    Mut,
    Own,
    Actor,
    Logic,
    Meta,
    Grad,
    Tensor,
    Layer,
    Fact,
    Rule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    IntLit,
    FloatLit,
    StrLit,
    Newline,
    Indent,
    Dedent,
    Colon,
    Arrow,
    Comma,
    Dot,
    LParen,
    RParen,
    Operator(String),
    Keyword(Keyword),
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            line,
            column,
        }
    }
}

