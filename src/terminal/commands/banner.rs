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
//   Description: CLI commands for banner enforcement and author configuration. Integrates banner
//                violations into the feedback telemetry pipeline as high-anomaly events.
// ================================================================================================

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use crate::agi_core::feedback::{FeedbackProcessor, FeedbackStore, ModelTrainer, RuleUpdater};
use crate::agi_core::Reasoner;
use crate::cortex::Cortex;
use crate::terminal::ui::{syntra_print, Color};

pub fn handle_banner_command<R: Reasoner>(cortex: &mut Cortex<R>, args: &[String]) {
    if args.is_empty() {
        syntra_print(
            "Usage: banner <enforce|set-author> ...",
            Color::Yellow,
            "Core",
        );
        return;
    }

    match args[0].as_str() {
        "enforce" => {
            if args.len() < 2 {
                syntra_print(
                    "Usage: banner enforce <root> [--dry]",
                    Color::Yellow,
                    "Core",
                );
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
                    syntra_print(
                        &format!("Banner enforcement failed: {e:?}"),
                        Color::Red,
                        "Core",
                    );
                    return;
                }
            };

            syntra_print(
                &format!(
                    "Scanned: {} | Updated: {} | Skipped: {}",
                    summary.scanned_files, summary.updated_files, summary.skipped_files
                ),
                Color::DarkGray,
                "Core",
            );

            let store = Arc::new(FeedbackStore::new(64));
            let mut processor = FeedbackProcessor::new(store.clone());
            processor.register_strategy(Arc::new(RuleUpdater::new()));
            processor.register_strategy(Arc::new(ModelTrainer::new()));
            processor.process_feedback_batch(feedback_events);

            for r in summary.results {
                if r.updated {
                    syntra_print(
                        &format!(
                            "[UPDATED] {} (module='{}')",
                            r.path.display(),
                            r.module_name
                        ),
                        Color::Cyan,
                        "Core",
                    );
                }
            }
        }
        "set-author" => {
            if args.len() < 2 {
                syntra_print(
                    "Usage: banner set-author <name>",
                    Color::Yellow,
                    "Core",
                );
                return;
            }

            let name = args[1..].join(" ");
            let config_path = PathBuf::from(".syntra/banner.toml");

            match cortex.set_banner_author(&config_path, &name) {
                Ok(_) => syntra_print(
                    &format!("Banner author set to '{name}'"),
                    Color::DarkGray,
                    "Core",
                ),
                Err(e) => syntra_print(
                    &format!("Failed to set author: {e:?}"),
                    Color::Red,
                    "Core",
                ),
            }
        }
        _ => {
            syntra_print("Unknown banner subcommand.", Color::Yellow, "Core");
        }
    }
}
