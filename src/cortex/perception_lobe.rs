/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/perception_lobe.rs
   Module:      Cortex - Perception Lobe
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Transforms raw text or HTML-like content into structured perceptions. This is
                Syntra's "eyes" on the world, used for summarization and structural awareness.

   Notes:
     - Axiom Three keeps parsing lightweight and heuristic-based.
     - Future axioms may integrate full HTML/DOM parsing and semantic models.
   ================================================================================================ */

#![allow(dead_code)]

use crate::utilities::{info, trace_enter, trace_exit};

/// A coarse-grained perception of some content.
#[derive(Debug, Clone)]
pub struct Perception {
    pub title: Option<String>,
    pub headings: Vec<String>,
    pub links: Vec<String>,
    pub summary: String,
}

pub struct PerceptionLobe;

impl PerceptionLobe {
    /// Create a perception from raw text or HTML-like content.
    pub fn perceive(raw: &str) -> Perception {
        trace_enter("PerceptionLobe::perceive");

        let title = Self::extract_title(raw);
        let headings = Self::extract_headings(raw);
        let links = Self::extract_links(raw);
        let summary = Self::summarize(raw);

        let perception = Perception {
            title,
            headings,
            links,
            summary,
        };

        trace_exit("PerceptionLobe::perceive");
        perception
    }

    fn extract_title(raw: &str) -> Option<String> {
        // Very lightweight heuristic: first non-empty line, or <title>...</title>
        if let Some(start) = raw.to_lowercase().find("<title>") {
            if let Some(end) = raw.to_lowercase().find("</title>") {
                if end > start {
                    let inner = &raw[start + 7..end];
                    return Some(inner.trim().to_string());
                }
            }
        }

        for line in raw.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }

        None
    }

    fn extract_headings(raw: &str) -> Vec<String> {
        let mut out = Vec::new();

        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") || trimmed.starts_with("## ") || trimmed.starts_with("### ") {
                out.push(trimmed.trim_start_matches('#').trim().to_string());
            }
        }

        out
    }

    fn extract_links(raw: &str) -> Vec<String> {
        let mut out = Vec::new();

        // Very simple heuristic: look for "http" tokens.
        for token in raw.split_whitespace() {
            if token.starts_with("http://") || token.starts_with("https://") {
                out.push(token.trim_matches(|c: char| c == '"' || c == '\'' || c == ',' || c == '.').to_string());
            }
        }

        out
    }

    fn summarize(raw: &str) -> String {
        let mut lines = Vec::new();
        for line in raw.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_string());
            }
            if lines.len() >= 5 {
                break;
            }
        }

        if lines.is_empty() {
            "No meaningful content detected.".to_string()
        } else {
            format!("Summary (first {} lines):\n{}", lines.len(), lines.join("\n"))
        }
    }
}
