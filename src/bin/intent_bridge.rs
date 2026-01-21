/* ================================================================================================
   SYNTRA BROWSER - AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/bin/intent_bridge.rs
   Module:      Intent Bridge Binary
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Command-line interface for Syntra's intent semantics. Accepts freeform text
                from external environments and returns a structured, ASCII-safe JSON line
                describing the interpreted intent. This binary acts as the primary gateway
                between shell tools and Syntra's AGI Core.

   Overview:
     • Reads freeform text from CLI arguments or stdin.
     • Routes the text through the Conduit Intent Bridge.
     • Outputs a single-line JSON object suitable for scripting and automation.

   Notes:
     - This binary is intentionally minimal and dependency-free for long-term stability.
     - Designed for use by PowerShell, Bash, automation tools, and external systems.
     - All output is ASCII-safe to ensure compatibility across terminals and platforms.
     - Future expansions may include richer metadata, multi-step plans, or confidence scores.
   ================================================================================================ */

use std::env;
use std::io::{self, Read};

use syntra_browse::conduit::bridge::process_intent;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Determine whether the intent was passed as CLI args or via stdin.
    let intent_text = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buffer) {
            eprintln!("{{\"error\":\"failed_to_read_stdin\",\"message\":\"{}\"}}", e);
            std::process::exit(1);
        }
        buffer.trim().to_string()
    };

    // Reject empty input early.
    if intent_text.is_empty() {
        println!("{{\"error\":\"empty_intent\",\"message\":\"No intent text provided.\"}}");
        return;
    }

    // Process the intent through Syntra's AGI Core.
    let response = process_intent(&intent_text);

    // Emit a single-line ASCII-safe JSON object.
    println!("{}", response.to_json_line());
}
