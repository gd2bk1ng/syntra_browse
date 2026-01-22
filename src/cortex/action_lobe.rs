/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/cortex/action_lobe.rs
   Module:      Cortex - Action Lobe
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Provides a thin abstraction over external actions Syntra can take, such as
                fetching URLs or invoking system commands. This is Syntra's "hands".

   Notes:
     - Axiom Three keeps actions conservative and observable.
     - Future axioms may introduce richer browser automation.
   ================================================================================================ */

#![allow(dead_code)]

use std::process::Command;

use crate::utilities::{info, warn, trace_enter, trace_exit};

pub struct ActionResult {
    pub success: bool,
    pub output: String,
}

pub struct ActionLobe;

impl ActionLobe {
    /// Fetch a URL using an external tool (e.g., `curl`), returning raw text.
    pub fn fetch_url(url: &str) -> ActionResult {
        trace_enter("ActionLobe::fetch_url");

        let output = Command::new("curl")
            .arg("-L")
            .arg("-s")
            .arg(url)
            .output();

        match output {
            Ok(out) => {
                let text = String::from_utf8_lossy(&out.stdout).to_string();
                trace_exit("ActionLobe::fetch_url");
                ActionResult {
                    success: out.status.success(),
                    output: text,
                }
            }
            Err(e) => {
                warn(&format!("Failed to invoke curl: {}", e));
                trace_exit("ActionLobe::fetch_url");
                ActionResult {
                    success: false,
                    output: String::new(),
                }
            }
        }
    }

    /// Run a simple system command and capture its output.
    pub fn run_command(cmd: &str, args: &[&str]) -> ActionResult {
        trace_enter("ActionLobe::run_command");

        let output = Command::new(cmd).args(args).output();

        match output {
            Ok(out) => {
                let text = String::from_utf8_lossy(&out.stdout).to_string();
                trace_exit("ActionLobe::run_command");
                ActionResult {
                    success: out.status.success(),
                    output: text,
                }
            }
            Err(e) => {
                warn(&format!("Failed to run command '{}': {}", cmd, e));
                trace_exit("ActionLobe::run_command");
                ActionResult {
                    success: false,
                    output: String::new(),
                }
            }
        }
    }
}
