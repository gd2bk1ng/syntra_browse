// ================================================================================================
//   SYNTRA LANGUAGE — AST (ABSTRACT SYNTAX TREE)
// ------------------------------------------------------------------------------------------------
//   File:        src/syntra_lang/ast.rs
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Core AST structures for the Syntra Language. This is the high-level representation
//       that will be typechecked, effect-checked, and lowered into Syntra-IR and MLIR.
// ================================================================================================

use crate::syntra_lang::types::SyntraType;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    Class(Class),
    Trait(TraitDecl),
    Layer(LayerDecl),
    Fact(Fact),
    Rule(Rule),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: Option<SyntraType>,
    pub body: Expr,
    pub is_pure: bool,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Option<SyntraType>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Function>,
}

#[derive(Debug, Clone)]
pub struct TraitDecl {
    pub name: String,
    pub methods: Vec<FunctionSig>,
}

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: Option<SyntraType>,
}

#[derive(Debug, Clone)]
pub struct LayerDecl {
    pub name: String,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Fact {
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub head: Fact,
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Binary {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    Block(Vec<Expr>),
    Unit,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Int(i64),
    Ident(String),
}
