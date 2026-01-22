/* ================================================================================================
   SYNTRA BROWSER — AXIOM FIVE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/evaluation_lobe.rs
   Module:      Cortex — Evaluation Lobe (Self-Comparison & Scoring)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides self-evaluation primitives for Syntra. Given two versions of a module
                (for example: previous vs new), this lobe produces a structured comparison:
                strengths, weaknesses, risk factors, and a qualitative verdict.

   Overview:
     • EvaluationSummary   — Structured comparison result.
     • EvaluationLobe      — Static methods for comparing code snapshots and plans.

   Notes:
     - This lobe is text- and heuristic-based; it does not execute code.
     - Future axioms may integrate tests, benchmarks, and telemetry for quantitative scoring.
   ================================================================================================ */

#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct EvaluationSummary {
    pub subject: String,
    pub previous_label: String,
    pub new_label: String,
    pub strengths_previous: Vec<String>,
    pub strengths_new: Vec<String>,
    pub risks: Vec<String>,
    pub verdict: String,
}

pub struct EvaluationLobe;

impl EvaluationLobe {
    /// High-level helper: compare two textual descriptions or code snapshots.
    pub fn compare_versions(subject: &str, previous: &str, new: &str) -> EvaluationSummary {
        // Extremely simple heuristics for now: length, keywords, and structure hints.
        let prev_len = previous.lines().count();
        let new_len = new.lines().count();

        let mut strengths_previous = Vec::new();
        let mut strengths_new = Vec::new();
        let mut risks = Vec::new();

        if prev_len > 0 && new_len > 0 {
            if new_len > prev_len {
                strengths_new.push("New version appears more extensive (more lines / structure).".to_string());
            } else if new_len < prev_len {
                strengths_previous.push("Previous version is more compact and potentially simpler.".to_string());
            }
        }

        if previous.contains("unsafe") && !new.contains("unsafe") {
            strengths_new.push("New version avoids 'unsafe' usage present in the previous version.".to_string());
        }
        if new.contains("unsafe") && !previous.contains("unsafe") {
            risks.push("New version introduces 'unsafe' where the previous version did not.".to_string());
        }

        if new.contains("TODO") {
            risks.push("New version contains TODO markers indicating incomplete work.".to_string());
        }

        if new.contains("///") && !previous.contains("///") {
            strengths_new.push("New version adds Rust doc comments, improving readability and tooling support.".to_string());
        }

        if previous.contains("///") && !new.contains("///") {
            strengths_previous.push("Previous version had more inline documentation.".to_string());
        }

        let verdict = Self::render_verdict(subject, &strengths_previous, &strengths_new, &risks);

        EvaluationSummary {
            subject: subject.to_string(),
            previous_label: "previous".to_string(),
            new_label: "new".to_string(),
            strengths_previous,
            strengths_new,
            risks,
            verdict,
        }
    }

    fn render_verdict(
        subject: &str,
        strengths_previous: &[String],
        strengths_new: &[String],
        risks: &[String],
    ) -> String {
        let mut verdict = String::new();
        verdict.push_str(&format!("Evaluation for '{}':\n", subject));

        if strengths_previous.is_empty() && strengths_new.is_empty() && risks.is_empty() {
            verdict.push_str("  No clear differences detected between previous and new versions.\n");
            verdict.push_str("  Verdict: neutral — both versions appear similar at a structural level.\n");
            return verdict;
        }

        if !strengths_previous.is_empty() {
            verdict.push_str("  Strengths of previous version:\n");
            for s in strengths_previous {
                verdict.push_str(&format!("    - {}\n", s));
            }
        }

        if !strengths_new.is_empty() {
            verdict.push_str("  Strengths of new version:\n");
            for s in strengths_new {
                verdict.push_str(&format!("    - {}\n", s));
            }
        }

        if !risks.is_empty() {
            verdict.push_str("  Risks / concerns:\n");
            for r in risks {
                verdict.push_str(&format!("    - {}\n", r));
            }
        }

        verdict.push_str("\n  High-level verdict:\n");

        if strengths_new.len() > strengths_previous.len() && risks.is_empty() {
            verdict.push_str("    The new version appears strictly better based on structure and documentation.\n");
        } else if strengths_previous.len() > strengths_new.len() && risks.is_empty() {
            verdict.push_str("    The previous version appears more robust or simpler; consider keeping it.\n");
        } else if !risks.is_empty() && strengths_new.len() <= strengths_previous.len() {
            verdict.push_str("    The new version introduces risks without clear compensating strengths; prefer the previous version.\n");
        } else {
            verdict.push_str("    Both versions have trade-offs; consider combining strengths and mitigating risks.\n");
        }

        verdict
    }

    /// Render a human-readable report from an EvaluationSummary.
    pub fn render_report(summary: &EvaluationSummary) -> String {
        let mut out = String::new();
        out.push_str(&summary.verdict);
        out
    }
}
