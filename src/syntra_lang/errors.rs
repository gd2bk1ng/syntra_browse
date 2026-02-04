// ================================================================================================
//   SYNTRA LANGUAGE — ERRORS
// ------------------------------------------------------------------------------------------------
//   File:        src/syntra_lang/errors.rs
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Front-end error representation for the Syntra Language. Used by lexer, parser, and
//       later by typechecker and effect system.
// ================================================================================================

use crate::syntra_lang::tokens::Token;

#[derive(Debug, Clone)]
pub struct SyntraError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl SyntraError {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }

    pub fn from_token(message: impl Into<String>, token: &Token) -> Self {
        Self {
            message: message.into(),
            line: token.line,
            column: token.column,
        }
    }
}
