// ================================================================================================
//   SYNTRA KERNEL — TERMINAL (BANNER COMMANDS)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/terminal/commands/banner.rs
//   Module:      Banner Commands
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: CLI commands for banner enforcement and author configuration.
// ================================================================================================

#![allow(dead_code)]

use std::path::PathBuf;

use crate::cortex::Cortex;
use crate::agi_core::feedback::{FeedbackProcessor, FeedbackStore, RuleUpdater, ModelTrainer};
use std::sync::Arc;

pub fn handle_banner_command<R>(cortex: &mut Cortex<R>, args: &[String])
where
    R: crate::agi_core::Reasoner,
{
    if args.is_empty() {
        eprintln!("Usage: banner <enforce|set-author> ...");
        return;
    }

    match args[0].as_str() {
        "enforce" => {
            if args.len() < 2 {
                eprintln!("Usage: banner enforce <root> [--dry]");
                return;
            }
            let root = PathBuf::from(&args[1]);
            let dry_run = args.get(2).map(|s| s == "--dry").unwrap_or(false);

            let config_path = PathBuf::from(".syntra/banner.toml");

            let (summary, feedback_events) = match cortex.enforce_banners_with_telemetry(
                &root,
                dry_run,
                &config_path,
            ) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Banner enforcement failed: {:?}", e);
                    return;
                }
            };

            println!(
                "Scanned: {} | Updated: {} | Skipped: {}",
                summary.scanned_files, summary.updated_files, summary.skipped_files
            );

            // Wire into feedback pipeline
            let store = Arc::new(FeedbackStore::new(64));
            let mut processor = FeedbackProcessor::new(store.clone());
            processor.register_strategy(Arc::new(RuleUpdater::new()));
            processor.register_strategy(Arc::new(ModelTrainer::new()));
            processor.process_feedback_batch(feedback_events);

            for r in summary.results {
                if r.updated {
                    println!(
                        "[UPDATED] {} (module='{}')",
                        r.path.display(),
                        r.module_name
                    );
                }
            }
        }
        "set-author" => {
            if args.len() < 2 {
                eprintln!("Usage: banner set-author <name>");
                return;
            }
            let name = args[1..].join(" ");
            let config_path = PathBuf::from(".syntra/banner.toml");
            if let Err(e) = cortex.set_banner_author(&config_path, &name) {
                eprintln!("Failed to set author: {:?}", e);
            } else {
                println!("Banner author set to '{}'", name);
            }
        }
        _ => {
            eprintln!("Unknown banner subcommand.");
        }
    }
}
