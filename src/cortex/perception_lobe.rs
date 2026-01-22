/* ================================================================================================
   SYNTRA BROWSER — AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/perception_lobe.rs
   Module:      Cortex — Perception Lobe
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Syntra’s perceptual subsystem. Converts raw text or HTML-like content into
                structured perceptions including title, headings, links, and a lightweight
                summary. This lobe provides Syntra with her first “eyes” on the world.

   Overview:
     • Perception       — Structured representation of observed content.
     • perceive()       — Main entry point for perception.
     • extract_title()  — Heuristic title extraction.
     • extract_headings — Markdown-style heading extraction.
     • extract_links    — Simple URL detection.
     • summarize        — Lightweight summarization.

   Notes:
     - Axiom Three keeps perception heuristic-based and dependency-minimal.
     - Future axioms may introduce full DOM parsing and semantic extraction.
   ================================================================================================ */

#![allow(dead_code)]

use crate::utilities::{trace_enter, trace_exit};

#[derive(Debug, Clone)]
pub struct Perception {
    pub title: Option<String>,
    pub headings: Vec<String>,
    pub links: Vec<String>,
    pub summary: String,
}

pub struct PerceptionLobe;

impl PerceptionLobe {
    pub fn perceive(raw: &str) -> Perception {
        trace_enter("PerceptionLobe::perceive");

        let perception = Perception {
            title: Self::extract_title(raw),
            headings: Self::extract_headings(raw),
            links: Self::extract_links(raw),
            summary: Self::summarize(raw),
        };

        trace_exit("PerceptionLobe::perceive");
        perception
    }

    fn extract_title(raw: &str) -> Option<String> {
        if let Some(start) = raw.to_lowercase().find("<title>") {
            if let Some(end) = raw.to_lowercase().find("</title>") {
                if end > start {
                    return Some(raw[start + 7..end].trim().to_string());
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
        raw.lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with("#") {
                    Some(trimmed.trim_start_matches('#').trim().to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    fn extract_links(raw: &str) -> Vec<String> {
        raw.split_whitespace()
            .filter(|t| t.starts_with("http://") || t.starts_with("https://"))
            .map(|t| t.trim_matches(|c: char| c == '"' || c == '\'' || c == ',' || c == '.'))
            .map(|t| t.to_string())
            .collect()
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
