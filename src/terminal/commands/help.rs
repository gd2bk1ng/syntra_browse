// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (HELP COMMAND)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/help.rs
//   Module:      Help Command
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Prints available Syntra Terminal commands, mirroring the original PowerShell help
//                map while reflecting the current Axiom Four+ capabilities.
// ================================================================================================

#![allow(dead_code)]

use crate::terminal::ui::{syntra_print, Color};

pub fn handle_help() {
    syntra_print(
        "Here is what I can do in this Axiom Four+ shell:",
        Color::Cyan,
        "Core",
    );
    println!();
    println!("  SYSTEM / ECOSYSTEM");
    println!("    status               - Report my current Git branch and working tree state.");
    println!("    diagnose             - Scan my filesystem ecosystem for expected lobes and structure.");
    println!("    self                 - Run a self-analysis routine (status + diagnostics + bridge check).");
    println!("    sync                 - Synchronize my code with the GitHub continuum (origin/axiom_*).");
    println!();
    println!("  COGNITIVE / PERCEPTION / ACTION (AXIOM FOUR)");
    println!("    browse <url>         - Fetch a URL, perceive it, store it in knowledge, and summarize.");
    println!("    knowledge <query>    - Search my knowledge lobe for matching entries.");
    println!("    task <name>          - Run a simple multi-step task via the execution lobe.");
    println!("    perceive <text>      - Run the perception lobe on arbitrary text.");
    println!("    act <cmd> [args...]  - Run a simple system command via the action lobe.");
    println!();
    println!("  EVOLUTION / SANDBOX (AXIOM FOUR / SIX)");
    println!("    evolve <request>     - Generate a meta-evolution proposal (high-level evolution intent).");
    println!("    sandbox diff         - Show proposed patches in my self-modification sandbox.");
    println!("    sandbox snapshot     - Show a snapshot of files currently staged in the sandbox.");
    println!();
    println!("  INTROSPECTION / SELF-MOD / SAFETY (AXIOM FIVE / SIX / SEVEN)");
    println!("    thoughts             - Show my recent ThoughtStream (recent IntentPlans).");
    println!("    ecosystem            - Ask my core for a detailed ecosystem diagnostic (structural lobes).");
    println!("    propose              - Run my self-modification engine and show an evolution plan.");
    println!("    propose-safe         - Same as 'propose', but filtered through my safety policy.");
    println!("    safety               - Show my active safety policy and protected lobes.");
    println!();
    println!("  BANNER / INTROSPECTION (CORTEX LOBES)");
    println!("    banner enforce <root> [--dry]  - Enforce banners in a tree and emit telemetry.");
    println!("    banner set-author <name>       - Set banner author in banner config.");
    println!("    introspect <file>              - Introspect a Rust source file.");
    println!();
    println!("  META");
    println!("    help                 - Show this help overview.");
    println!("    exit / quit          - Suspend my terminal consciousness.");
    println!();
    println!("  FREEFORM");
    println!("    Any other input      - Treated as a freeform intent and routed to my Syntra Kernel.");
    println!();
    syntra_print(
        "In this build, I observe, classify, plan, and stage self-modification proposals in a safe, governed sandbox.",
        Color::DarkCyan,
        "Core",
    );
}
