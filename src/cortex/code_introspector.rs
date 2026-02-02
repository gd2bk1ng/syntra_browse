// ================================================================================================
//   SYNTRA KERNEL — CORTEX (CODE INTROSPECTOR LOBE)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/cortex/code_introspector.rs
//   Module:      Code Introspector Lobe
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Lightweight structural analysis of Rust source files. Extracts module semantics,
//                key items, and generates human-readable descriptions and notes for banners.
// ================================================================================================

#![allow(dead_code)]

use std::fs;
use std::path::Path;

/// High-level classification of a file’s role.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum FileRole {
    AgiCore,
    Cortex,
    Runtime,
    Renderer,
    Browser,
    Terminal,
    Utilities,
    Tests,
    Unknown,
}

/// Summary of a Rust source file for banner generation and documentation.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct FileAnalysis {
    pub module_name: String,
    pub description: String,
    pub notes: Vec<String>,
    pub role: FileRole,
    pub structs: Vec<String>,
    pub enums: Vec<String>,
    pub traits: Vec<String>,
    pub functions: Vec<String>,
}

impl FileAnalysis {
    pub fn short_description(&self) -> String {
        self.description.clone()
    }
}

/// Introspector that uses simple heuristics to understand a Rust file.
pub struct CodeIntrospector;

impl CodeIntrospector {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze_file(path: &Path) -> std::io::Result<FileAnalysis> {
        let content = fs::read_to_string(path)?;
        Ok(Self::analyze_content(path, &content))
    }

    pub fn analyze_content(path: &Path, content: &str) -> FileAnalysis {
        let role = infer_role(path);
        let (structs, enums, traits, functions) = extract_items(content);
        let module_name = infer_module_name(path, &role);
        let description = infer_description(&module_name, &role, &structs, &traits, &functions);
        let notes = infer_notes(&role, &structs, &traits, &functions);

        FileAnalysis {
            module_name,
            description,
            notes,
            role,
            structs,
            enums,
            traits,
            functions,
        }
    }
}

fn infer_role(path: &Path) -> FileRole {
    let path_str = path.to_string_lossy().to_lowercase();

    if path_str.contains("agi_core") {
        FileRole::AgiCore
    } else if path_str.contains("cortex") {
        FileRole::Cortex
    } else if path_str.contains("runtime") {
        FileRole::Runtime
    } else if path_str.contains("renderer") {
        FileRole::Renderer
    } else if path_str.contains("browser") {
        FileRole::Browser
    } else if path_str.contains("terminal") {
        FileRole::Terminal
    } else if path_str.contains("tests") || path_str.ends_with("_test.rs") {
        FileRole::Tests
    } else if path_str.contains("util") {
        FileRole::Utilities
    } else {
        FileRole::Unknown
    }
}

fn extract_items(content: &str) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let mut structs = Vec::new();
    let mut enums = Vec::new();
    let mut traits = Vec::new();
    let mut functions = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ") {
            if let Some(name) = trimmed.split_whitespace().nth(2) {
                structs.push(name.trim_end_matches('{').to_string());
            }
        } else if trimmed.starts_with("pub enum ") || trimmed.starts_with("enum ") {
            if let Some(name) = trimmed.split_whitespace().nth(2) {
                enums.push(name.trim_end_matches('{').to_string());
            }
        } else if trimmed.starts_with("pub trait ") || trimmed.starts_with("trait ") {
            if let Some(name) = trimmed.split_whitespace().nth(2) {
                traits.push(name.trim_end_matches('{').to_string());
            }
        } else if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            if let Some(name) = trimmed.split_whitespace().nth(1) {
                functions.push(name.split('(').next().unwrap_or(name).to_string());
            }
        }
    }

    (structs, enums, traits, functions)
}

fn infer_module_name(path: &Path, role: &FileRole) -> String {
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown.rs");

    let base = file_name.trim_end_matches(".rs");

    match role {
        FileRole::AgiCore => format!("AGI Core — {}", to_title_case(base)),
        FileRole::Cortex => format!("Cortex — {}", to_title_case(base)),
        FileRole::Runtime => format!("Runtime — {}", to_title_case(base)),
        FileRole::Renderer => format!("Renderer — {}", to_title_case(base)),
        FileRole::Browser => format!("Browser — {}", to_title_case(base)),
        FileRole::Terminal => format!("Terminal — {}", to_title_case(base)),
        FileRole::Utilities => format!("Utilities — {}", to_title_case(base)),
        FileRole::Tests => format!("Tests — {}", to_title_case(base)),
        FileRole::Unknown => to_title_case(base),
    }
}

fn infer_description(
    module_name: &str,
    role: &FileRole,
    structs: &[String],
    traits: &[String],
    functions: &[String],
) -> String {
    let mut desc = String::new();

    match role {
        FileRole::AgiCore => {
            desc.push_str("Core AGI module for Syntra Kernel. ");
        }
        FileRole::Cortex => {
            desc.push_str("Cortex lobe for introspection, diagnostics, or meta-operations. ");
        }
        FileRole::Runtime => {
            desc.push_str("Runtime subsystem for scheduling, actors, or execution. ");
        }
        FileRole::Renderer => {
            desc.push_str("Rendering or visualization subsystem. ");
        }
        FileRole::Browser => {
            desc.push_str("Browsing, navigation, or session management subsystem. ");
        }
        FileRole::Terminal => {
            desc.push_str("Terminal or CLI interaction subsystem. ");
        }
        FileRole::Utilities => {
            desc.push_str("Utility helpers shared across the kernel. ");
        }
        FileRole::Tests => {
            desc.push_str("Test harness or validation module. ");
        }
        FileRole::Unknown => {}
    }

    if !structs.is_empty() {
        desc.push_str(&format!("Defines structs: {}. ", structs.join(", ")));
    }
    if !traits.is_empty() {
        desc.push_str(&format!("Defines traits: {}. ", traits.join(", ")));
    }
    if !functions.is_empty() {
        desc.push_str(&format!(
            "Provides functions such as: {}.",
            functions
                .iter()
                .take(4)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    if desc.is_empty() {
        format!("Module: {}.", module_name)
    } else {
        desc
    }
}

fn infer_notes(
    role: &FileRole,
    structs: &[String],
    traits: &[String],
    functions: &[String],
) -> Vec<String> {
    let mut notes = Vec::new();

    match role {
        FileRole::AgiCore => notes.push("Part of the AGI Core semantic or control stack.".into()),
        FileRole::Cortex => notes.push("Participates in introspection, diagnostics, or meta-control.".into()),
        FileRole::Runtime => notes.push("Influences runtime behavior or execution semantics.".into()),
        FileRole::Renderer => notes.push("Affects visual output or scene composition.".into()),
        FileRole::Browser => notes.push("Impacts browsing, navigation, or session state.".into()),
        FileRole::Terminal => notes.push("Shapes terminal or CLI interaction patterns.".into()),
        FileRole::Utilities => notes.push("Shared utility; changes may ripple across modules.".into()),
        FileRole::Tests => notes.push("Test module; used for validation and regression detection.".into()),
        FileRole::Unknown => {}
    }

    if !traits.is_empty() {
        notes.push("Traits defined here may be implemented across multiple lobes.".into());
    }
    if functions
        .iter()
        .any(|f| f.contains("scan") || f.contains("diagnostic"))
    {
        notes.push("Contains diagnostic or scanning logic.".into());
    }

    notes
}

fn to_title_case(s: &str) -> String {
    s.split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
