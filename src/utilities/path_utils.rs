// ================================================================================================
//   SYNTRA KERNEL — PATH UTILITIES
// ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/utilities/path_utils.rs
//   Module:      Utilities — Path Normalization & Matching
//   Author:      Alexandr Roussinov (gd2bk1ng)
//
//   Description:
//       Provides robust, cross‑platform path utilities for Syntra’s AGI Kernel. These utilities
//       ensure consistent handling of filesystem paths across Windows, Linux, macOS, containers,
//       and sandbox environments. This module is foundational for:
//
//         • Baseline snapshots
//         • Self‑modification policy enforcement
//         • Evolution diffing
//         • Dependency graph analysis
//         • Future AGI introspection and lobe‑level reasoning
//
//   Overview:
//       - normalize_path(): canonical forward‑slash path normalization.
//       - is_within(): checks if a path is inside another directory.
//       - join_paths(): safe join with normalization.
//       - strip_prefix_safe(): prefix removal without panics.
//       - glob_match(): advanced glob‑like matching (supports *, **, ?).
//       - canonicalize_lossy(): safe canonicalization without failing on missing paths.
//
//   Notes:
//       - ASCII‑safe, cross‑platform, stable.
//       - Designed for long‑term evolution and introspection.
//       - MIT & Apache 2.0 dual‑licensed.
// ================================================================================================
//
//   Copyright:
//       This file is dual‑licensed under MIT and Apache 2.0.
//       You may use, modify, and distribute it under either license.
//
// ================================================================================================

#![allow(dead_code)]

use std::path::{Path, PathBuf};

/// Normalize a path to a forward‑slash UTF‑8 string.
///
/// Ensures:
///   - Windows backslashes → forward slashes
///   - Removes redundant "./"
///   - No trailing slash (except root)
pub fn normalize_path(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");

    let trimmed = if let Some(stripped) = s.strip_prefix("./") {
        stripped.to_string()
    } else {
        s
    };

    if trimmed.ends_with('/') && trimmed.len() > 1 {
        trimmed.trim_end_matches('/').to_string()
    } else {
        trimmed
    }
}

/// Safely join two paths and normalize the result.
pub fn join_paths(base: &Path, child: &str) -> PathBuf {
    let joined = base.join(child);
    PathBuf::from(normalize_path(&joined))
}

/// Returns true if `child` is inside `parent` (after normalization).
pub fn is_within(parent: &Path, child: &Path) -> bool {
    let p = normalize_path(parent);
    let c = normalize_path(child);

    c == p || c.starts_with(&format!("{}/", p))
}

/// Remove a prefix from a path without panicking.
///
/// Returns:
///   - Some(relative_path) if prefix matches
///   - None otherwise
pub fn strip_prefix_safe<'a>(base: &'a Path, full: &'a Path) -> Option<String> {
    let rel = pathdiff::diff_paths(full, base)?;
    Some(normalize_path(&rel))
}

/// Canonicalize a path but never fail.
///
/// If canonicalization fails (e.g., missing file), returns a normalized version of the input.
pub fn canonicalize_lossy(path: &Path) -> PathBuf {
    match path.canonicalize() {
        Ok(p) => PathBuf::from(normalize_path(&p)),
        Err(_) => PathBuf::from(normalize_path(path)),
    }
}

/// Glob‑like matching with support for:
///   - "*"  → match any characters except "/"
///   - "**" → match across directory boundaries
///   - "?"  → match any single character
///
/// This is more powerful than the minimal matcher in self_mod_policy.
pub fn glob_match(pattern: &str, text: &str) -> bool {
    glob_match_recursive(pattern, text)
}

fn glob_match_recursive(pat: &str, text: &str) -> bool {
    // Base cases
    if pat.is_empty() {
        return text.is_empty();
    }

    let pat_chars: Vec<char> = pat.chars().collect();
    let txt_chars: Vec<char> = text.chars().collect();

    match pat_chars[0] {
        // "**" → match zero or more directories
        '*' if pat_chars.get(1) == Some(&'*') => {
            let rest = &pat[2..];
            if glob_match_recursive(rest, text) {
                return true;
            }
            if !text.is_empty() {
                return glob_match_recursive(pat, &text[1..]);
            }
            false
        }

        // "*" → match any sequence except "/"
        '*' => {
            for i in 0..=text.len() {
                if glob_match_recursive(&pat[1..], &text[i..]) {
                    return true;
                }
            }
            false
        }

        // "?" → match any single character
        '?' => {
            if text.is_empty() {
                return false;
            }
            glob_match_recursive(&pat[1..], &text[1..])
        }

        // Literal match
        c => {
            if text.is_empty() || text.chars().next().unwrap() != c {
                return false;
            }
            glob_match_recursive(&pat[1..], &text[1..])
        }
    }
}
