// ================================================================================================
//   SYNTRA LANGUAGE — LEXER (V0.1 MINIMAL)
// ------------------------------------------------------------------------------------------------
//   File:        src/syntra_lang/lexer.rs
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       A minimal, intentionally naive lexer for the Syntra Language. It is enough to
//       bootstrap the Syntra → AST pipeline and can be refined as the language spec evolves.
// ================================================================================================

use crate::syntra_lang::tokens::{Keyword, Token, TokenKind};
use crate::syntra_lang::errors::SyntraError;

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current: Option<char>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.chars();
        let current = chars.next();
        Self {
            chars,
            current,
            line: 1,
            column: 1,
        }
    }

    fn bump(&mut self) {
        if let Some(c) = self.current {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.current = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current, Some(c) if c.is_whitespace()) {
            self.bump();
        }
    }

    fn lex_ident_or_keyword(&mut self) -> Token {
        let start_col = self.column;
        let mut s = String::new();
        while matches!(self.current, Some(c) if c.is_alphanumeric() || c == '_' || c == '.') {
            s.push(self.current.unwrap());
            self.bump();
        }

        let kind = match s.as_str() {
            "module" => TokenKind::Keyword(Keyword::Module),
            "import" => TokenKind::Keyword(Keyword::Import),
            "fn" => TokenKind::Keyword(Keyword::Fn),
            "pure" => TokenKind::Keyword(Keyword::Pure),
            "class" => TokenKind::Keyword(Keyword::Class),
            "trait" => TokenKind::Keyword(Keyword::Trait),
            "impl" => TokenKind::Keyword(Keyword::Impl),
            "match" => TokenKind::Keyword(Keyword::Match),
            "of" => TokenKind::Keyword(Keyword::Of),
            "if" => TokenKind::Keyword(Keyword::If),
            "else" => TokenKind::Keyword(Keyword::Else),
            "return" => TokenKind::Keyword(Keyword::Return),
            "let" => TokenKind::Keyword(Keyword::Let),
            "mut" => TokenKind::Keyword(Keyword::Mut),
            "own" => TokenKind::Keyword(Keyword::Own),
            "actor" => TokenKind::Keyword(Keyword::Actor),
            "logic" => TokenKind::Keyword(Keyword::Logic),
            "meta" => TokenKind::Keyword(Keyword::Meta),
            "grad" => TokenKind::Keyword(Keyword::Grad),
            "tensor" => TokenKind::Keyword(Keyword::Tensor),
            "layer" => TokenKind::Keyword(Keyword::Layer),
            "fact" => TokenKind::Keyword(Keyword::Fact),
            "rule" => TokenKind::Keyword(Keyword::Rule),
            _ => TokenKind::Ident,
        };

        Token::new(kind, s, self.line, start_col)
    }

    fn lex_number(&mut self) -> Token {
        let start_col = self.column;
        let mut s = String::new();
        let mut has_dot = false;

        while let Some(c) = self.current {
            if c.is_ascii_digit() {
                s.push(c);
                self.bump();
            } else if c == '.' && !has_dot {
                has_dot = true;
                s.push(c);
                self.bump();
            } else {
                break;
            }
        }

        let kind = if has_dot {
            TokenKind::FloatLit
        } else {
            TokenKind::IntLit
        };

        Token::new(kind, s, self.line, start_col)
    }

    fn lex_string(&mut self) -> Result<Token, SyntraError> {
        let start_col = self.column;
        self.bump(); // opening quote
        let mut s = String::new();

        while let Some(c) = self.current {
            if c == '"' {
                self.bump(); // closing quote
                return Ok(Token::new(TokenKind::StrLit, s, self.line, start_col));
            } else {
                s.push(c);
                self.bump();
            }
        }

        Err(SyntraError::new(
            "unterminated string literal",
            self.line,
            start_col,
        ))
    }

    pub fn next_token(&mut self) -> Result<Token, SyntraError> {
        self.skip_whitespace();

        let line = self.line;
        let col = self.column;

        match self.current {
            None => Ok(Token::new(TokenKind::Eof, "", line, col)),
            Some(c) if c.is_ascii_alphabetic() || c == '_' => Ok(self.lex_ident_or_keyword()),
            Some(c) if c.is_ascii_digit() => Ok(self.lex_number()),
            Some('"') => self.lex_string(),
            Some(':') => {
                self.bump();
                if self.current == Some(':') {
                    self.bump();
                    Ok(Token::new(TokenKind::Operator("::".into()), "::", line, col))
                } else {
                    Ok(Token::new(TokenKind::Colon, ":", line, col))
                }
            }
            Some('-') => {
                self.bump();
                if self.current == Some('>') {
                    self.bump();
                    Ok(Token::new(TokenKind::Arrow, "->", line, col))
                } else {
                    Ok(Token::new(TokenKind::Operator("-".into()), "-", line, col))
                }
            }
            Some('(') => {
                self.bump();
                Ok(Token::new(TokenKind::LParen, "(", line, col))
            }
            Some(')') => {
                self.bump();
                Ok(Token::new(TokenKind::RParen, ")", line, col))
            }
            Some(',') => {
                self.bump();
                Ok(Token::new(TokenKind::Comma, ",", line, col))
            }
            Some('.') => {
                self.bump();
                Ok(Token::new(TokenKind::Dot, ".", line, col))
            }
            Some(other) => {
                let ch = other;
                self.bump();
                Ok(Token::new(
                    TokenKind::Operator(ch.to_string()),
                    ch.to_string(),
                    line,
                    col,
                ))
            }
        }
    }
}

