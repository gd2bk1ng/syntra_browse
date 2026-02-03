// ================================================================================================
//   SYNTRA KERNEL — SELF-MOD POLICY
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/agi_core/self_mod_policy.rs
//   Module:      AGI Core - Self-Modification Policy
//   Description: Defines and enforces Syntra's self-modification policy. Controls which paths are
//                forbidden, propose-only, or fully self-modifiable, forming the constitutional
//                safety layer for her evolution.
//
//   Notes:
//     - Fully customizable and extensible via artifacts/self_mod_policy.toml.
//     - Part of the Syntra Kernel AGI architecture.
//     - All components follow Axiom Three (Reasoning), Axiom Five (Intent),
//       Axiom Six (Self‑Modification), and Axiom Seven (Safety).
// ================================================================================================

#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// The mode of self-modification allowed for a given path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfModMode {
    /// Syntra may read but never propose or apply changes.
    Forbidden,
    /// Syntra may propose changes, but only a human applies them.
    ProposeOnly,
    /// Syntra may propose and apply changes, with human approval.
    SelfMod,
}

impl SelfModMode {
    pub fn from_str(s: &str) -> Self {
        match s {
            "forbidden" => SelfModMode::Forbidden,
            "propose_only" => SelfModMode::ProposeOnly,
            "self_mod" => SelfModMode::SelfMod,
            _ => SelfModMode::Forbidden,
        }
    }
}

/// Raw TOML representation of the policy file.
#[derive(Debug, Deserialize)]
struct RawSelfModPolicy {
    #[serde(default)]
    global: RawGlobalPolicy,
    #[serde(default)]
    paths: HashMap<String, RawPathPolicy>,
}

#[derive(Debug, Deserialize)]
struct RawGlobalPolicy {
    #[serde(default = "default_allow_self_mod")]
    allow_self_mod: bool,
}

fn default_allow_self_mod() -> bool {
    true
}

#[derive(Debug, Deserialize)]
struct RawPathPolicy {
    mode: String,
}

/// In-memory representation of the self-mod policy.
#[derive(Debug, Clone)]
pub struct SelfModPolicy {
    pub allow_self_mod: bool,
    /// Map of glob-like patterns to modes.
    patterns: Vec<(String, SelfModMode)>,
}

impl SelfModPolicy {
    /// Load the policy from the given TOML file path.
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let raw: RawSelfModPolicy = toml::from_str(&contents)?;

        let mut patterns = Vec::new();
        for (pattern, policy) in raw.paths {
            let mode = SelfModMode::from_str(policy.mode.as_str());
            patterns.push((pattern, mode));
        }

        Ok(Self {
            allow_self_mod: raw.global.allow_self_mod,
            patterns,
        })
    }

    /// Determine the mode for a given file path (relative to repo root).
    pub fn mode_for<P: AsRef<Path>>(&self, path: P) -> SelfModMode {
        let path_str = normalize_path(path.as_ref());

        // Last matching pattern wins (more specific patterns can override earlier ones).
        let mut result = SelfModMode::SelfMod;
        for (pattern, mode) in &self.patterns {
            if pattern_matches(pattern, &path_str) {
                result = *mode;
            }
        }
        result
    }

    pub fn is_forbidden<P: AsRef<Path>>(&self, path: P) -> bool {
        self.mode_for(path) == SelfModMode::Forbidden
    }

    pub fn is_propose_only<P: AsRef<Path>>(&self, path: P) -> bool {
        self.mode_for(path) == SelfModMode::ProposeOnly
    }

    pub fn is_self_mod<P: AsRef<Path>>(&self, path: P) -> bool {
        self.mode_for(path) == SelfModMode::SelfMod
    }
}

/// Normalize a path to a forward-slash string for matching.
fn normalize_path(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");
    // Strip leading "./" if present
    if let Some(stripped) = s.strip_prefix("./") {
        stripped.to_string()
    } else {
        s
    }
}

/// Very small glob-like matcher supporting:
/// - "**" for recursive match
/// - "*" for single-segment wildcard
fn pattern_matches(pattern: &str, path: &str) -> bool {
    // Fast path: exact match
    if pattern == path {
        return true;
    }

    // Handle "**" patterns like "docs/**"
    if let Some(prefix) = pattern.strip_suffix("/**") {
        return path == prefix || path.starts_with(&format!("{}/", prefix));
    }

    // Handle simple "*" wildcard in a single segment
    if pattern.contains('*') {
        // Very minimal: treat "*" as "match any chars in this segment"
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            let start = parts[0];
            let end = parts[1];
            return path.starts_with(start) && path.ends_with(end);
        }
    }

    false
}
