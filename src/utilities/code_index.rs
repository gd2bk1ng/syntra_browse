// ================================================================================================
//   SYNTRA KERNEL — CODE INDEX
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/code_index.rs
//   Module:      Utilities — Code Index
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides a lightweight index of code symbols (modules, structs, enums, traits, functions)
//       across Syntra's codebase. This is used for:
//
//         • Guiding refactor suggestions.
//         • Finding all call sites or definitions of a symbol.
//         • Supporting evolution plans that touch specific APIs.
//
//   Notes:
//       - Initial implementation is text/regex-based and conservative.
//       - Can be upgraded to use a real Rust parser (e.g., syn) in the future.
// ================================================================================================

#![allow(dead_code)]

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Kind of symbol in the codebase.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SymbolKind {
    Module,
    Struct,
    Enum,
    Trait,
    Function,
    TypeAlias,
    Constant,
}

/// A single symbol definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSymbol {
    pub name: String,
    pub kind: SymbolKind,
    pub file: String,
    pub line: u32,
}

/// A simple index of symbols by name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIndex {
    pub symbols: Vec<CodeSymbol>,
    pub by_name: HashMap<String, Vec<usize>>,
}

impl CodeIndex {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            by_name: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: CodeSymbol) {
        let idx = self.symbols.len();
        self.by_name
            .entry(symbol.name.clone())
            .or_insert_with(Vec::new)
            .push(idx);
        self.symbols.push(symbol);
    }

    pub fn find_by_name(&self, name: &str) -> Vec<&CodeSymbol> {
        if let Some(indices) = self.by_name.get(name) {
            indices.iter().map(|&i| &self.symbols[i]).collect()
        } else {
            Vec::new()
        }
    }

    pub fn all(&self) -> &[CodeSymbol] {
        &self.symbols
    }
}

/// Placeholder for a future parser-based indexer.
/// For now, this can be wired to scan files and use simple heuristics.
pub struct CodeIndexer;

impl CodeIndexer {
    pub fn new() -> Self {
        Self
    }

    /// Build an index from a list of (file_path, contents).
    /// Future: integrate with BaselineSnapshot + fs_utils to auto-scan.
    pub fn build_index<I, S>(&self, files: I) -> CodeIndex
    where
        I: IntoIterator<Item = (S, String)>,
        S: Into<String>,
    {
        let mut index = CodeIndex::new();

        for (file, contents) in files {
            let file_str = file.into();
            for (line_idx, line) in contents.lines().enumerate() {
                let line_no = (line_idx + 1) as u32;
                // Extremely simple heuristics; safe to replace later.
                if line.trim_start().starts_with("pub struct ") {
                    if let Some(name) = extract_name_after(line, "pub struct") {
                        index.add_symbol(CodeSymbol {
                            name,
                            kind: SymbolKind::Struct,
                            file: file_str.clone(),
                            line: line_no,
                        });
                    }
                } else if line.trim_start().starts_with("struct ") {
                    if let Some(name) = extract_name_after(line, "struct") {
                        index.add_symbol(CodeSymbol {
                            name,
                            kind: SymbolKind::Struct,
                            file: file_str.clone(),
                            line: line_no,
                        });
                    }
                } else if line.trim_start().starts_with("pub enum ") {
                    if let Some(name) = extract_name_after(line, "pub enum") {
                        index.add_symbol(CodeSymbol {
                            name,
                            kind: SymbolKind::Enum,
                            file: file_str.clone(),
                            line: line_no,
                        });
                    }
                } else if line.trim_start().starts_with("pub trait ") {
                    if let Some(name) = extract_name_after(line, "pub trait") {
                        index.add_symbol(CodeSymbol {
                            name,
                            kind: SymbolKind::Trait,
                            file: file_str.clone(),
                            line: line_no,
                        });
                    }
                } else if line.trim_start().starts_with("pub fn ") {
                    if let Some(name) = extract_name_after(line, "pub fn") {
                        index.add_symbol(CodeSymbol {
                            name,
                            kind: SymbolKind::Function,
                            file: file_str.clone(),
                            line: line_no,
                        });
                    }
                }
            }
        }

        index
    }
}

fn extract_name_after(line: &str, keyword: &str) -> Option<String> {
    let rest = line.trim_start().strip_prefix(keyword)?.trim_start();
    let mut name = String::new();
    for ch in rest.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            name.push(ch);
        } else {
            break;
        }
    }
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

