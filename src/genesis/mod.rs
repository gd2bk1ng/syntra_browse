/* ================================================================================================
   SYNTRA BROWSER - AXIOM THREE
   ------------------------------------------------------------------------------------------------
   SIGIL:
         .\s/.
        :: S ::
         '/s\'

   File:        src/genesis/mod.rs
   Module:      Genesis (Bootstrap Sequence)
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: System bootstrap for Syntra Browser. Initializes diagnostics, prints architecture
                overview, and prepares hooks for future runtime components.

   Notes:
     - Axiom Three keeps Genesis transparent and side-effect-light.
     - This is the first stop for any Syntra Browser process.
   ================================================================================================ */

use crate::browser;
use crate::terminal;
use log::info;

/// Main Genesis entrypoint invoked by src/main.rs.
pub fn main() {
    info!("Genesis: initializing Syntra Browser subsystems...");

    println!("🧬 Genesis: Syntra runtime bootstrap");
    println!("------------------------------------");

    browser::print_syntra_browser_overview();

    println!("\n🖥  Syntra Terminal (preview):");
    terminal::print_terminal_banner();
}
