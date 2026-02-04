// ================================================================================================
//   SYNTRA KERNEL — SYNTRA LANGUAGE FRONTEND
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/syntra_lang/mod.rs
//   Module:      Syntra Language Frontend
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Minimal but coherent front-end for the Syntra Language — a hybrid, safety-first,
//       differentiable, neurosymbolic language designed for AGI-era systems.
//
//       This module currently provides:
//         - tokens      — lexical token definitions
//         - lexer       — a minimal lexer for Syntra source
//         - ast         — core AST structures (modules, functions, classes, rules, layers)
//         - types       — initial Syntra type system skeleton
//         - errors      — front-end error representation
//
//       Future extensions:
//         - parser      — Syntra source → AST
//         - typecheck   — hybrid, linear, and effect typing
//         - lowering    — Syntra AST → Syntra-IR → MLIR dialects
//         - runtime     — capability-aware, sandboxable execution engine
//
//   Philosophy:
//       Readable like Python.
//       Reliable like Rust.
//       Expressive like Julia.
//       Logical like Prolog.
//       Differentiable like modern AI.
//       Future-proof like MLIR.
//       Recognizable by the :: that defines it.
// ================================================================================================

#![allow(dead_code)]

pub mod tokens;
pub mod lexer;
pub mod ast;
pub mod types;
pub mod errors;

// Re-exports for convenience
pub use tokens::{Keyword, Token, TokenKind};
pub use lexer::Lexer;
pub use ast::{Module, Item, Function, Class, TraitDecl, LayerDecl, Expr};
pub use types::SyntraType;
pub use errors::SyntraError;
