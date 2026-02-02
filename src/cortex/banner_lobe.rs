// ================================================================================================
//   SYNTRA KERNEL — CORTEX (BANNER LOBE)
//   ------------------------------------------------------------------------------------------------
//        .\s/.
//       :: S ::
//        '/s\'
//
//   File:        src/cortex/banner_lobe.rs
//   Module:      Banner Lobe
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description: Canonical banner generator for Syntra Kernel source files. Owns the sigil,
//                formatting rules, and author resolution. All banners must be produced here.
// ================================================================================================

#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;

// Serde is optional — only active when the "agi" feature is enabled.
#[cfg(feature = "agi")]
use serde::{Deserialize, Serialize};

/// Persistent configuration for banner generation.
#[cfg_attr(feature = "agi", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct BannerConfig {
    /// Default author name to use when generating banners.
    pub author: Option<String>,
    /// Optional project name or kernel variant.
    pub kernel_name: String,
}

impl Default for BannerConfig {
    fn default() -> Self {
        Self {
            author: None,
            kernel_name: "SYNTRA KERNEL".to_string(),
        }
    }
}

impl BannerConfig {
    pub fn load_from(path: &Path) -> Self {
        if let Ok(bytes) = fs::read(path) {
            // toml 0.9 uses `from_slice`, unchanged
            #[cfg(feature = "agi")]
            if let Ok(cfg) = toml::from_slice::<BannerConfig>(&bytes) {
                return cfg;
            }
        }
        Self::default()
    }

    pub fn save_to(&self, path: &Path) -> std::io::Result<()> {
        // toml 0.9 removed `to_vec_pretty` — use `to_string_pretty` + bytes
        #[cfg(feature = "agi")]
        let data = toml::to_string_pretty(self)
            .expect("Failed to serialize BannerConfig")
            .into_bytes();

        #[cfg(not(feature = "agi"))]
        let data = Vec::new(); // no-op when serde is disabled

        fs::create_dir_all(path.parent().unwrap_or_else(|| Path::new(".")))?;
        fs::write(path, data)
    }
}

/// Central banner generator. This is the single source of truth for the sigil and layout.
pub struct BannerLobe {
    config: BannerConfig,
    config_path: PathBuf,
}

impl BannerLobe {
    pub fn new(config_path: impl AsRef<Path>) -> Self {
        let config_path = config_path.as_ref().to_path_buf();
        let config = BannerConfig::load_from(&config_path);
        Self { config, config_path }
    }

    /// Returns the effective author, preferring config, then environment, then fallback.
    pub fn effective_author(&self) -> String {
        if let Some(a) = &self.config.author {
            return a.clone();
        }

        if let Ok(user) = env::var("SYNTRA_AUTHOR") {
            return user;
        }
        if let Ok(user) = env::var("USER") {
            return user;
        }
        if let Ok(user) = env::var("USERNAME") {
            return user;
        }

        "Unknown Author".to_string()
    }

    /// Allows external tools (UI, CLI) to set the author once the user provides it.
    pub fn set_author(&mut self, author: String) -> std::io::Result<()> {
        self.config.author = Some(author);
        self.config.save_to(&self.config_path)
    }

    /// Render a canonical banner for a given file/module/description.
    pub fn render_banner(
        &self,
        file: &str,
        module: &str,
        description: &str,
    ) -> String {
        let author = self.effective_author();
        let kernel = &self.config.kernel_name;

        format!(
"// ================================================================================================
//   {kernel} — {module}
//   ------------------------------------------------------------------------------------------------
//        .\\s/.
//       :: S ::
//        '/s\\'
//
//   File:        {file}
//   Module:      {module}
//   Author:      {author}
//   Description: {description}
//   
// ================================================================================================",
        )
    }

    /// Render a banner with an auto-generated timestamp note.
    pub fn render_banner_with_timestamp(
        &self,
        file: &str,
        module: &str,
        description: &str,
    ) -> String {
        let mut banner = self.render_banner(file, module, description);
        let ts = Utc::now().to_rfc3339();
        banner.push_str(&format!(
            "\n//   Generated:   {ts}\n// ================================================================================================"
        ));
        banner
    }

    /// Check if a given text already starts with a Syntra Kernel banner.
    pub fn has_banner(&self, content: &str) -> bool {
        content.contains("SYNTRA KERNEL")
            && content.contains(".\\s/.")
            && content.contains("'/s\\'")
    }

    /// Replace or insert a banner at the top of a file.
    pub fn apply_banner_to_content(
        &self,
        file: &str,
        module: &str,
        description: &str,
        original: &str,
    ) -> String {
        let new_banner = self.render_banner(file, module, description);

        if self.has_banner(original) {
            // Replace existing banner (assume it's the first block of comment lines).
            let mut lines = original.lines();
            let mut after_banner = Vec::new();
            let mut in_banner = true;

            while let Some(line) = lines.next() {
                if in_banner && line.trim_start().starts_with("//") {
                    continue; // skip old banner lines
                } else {
                    in_banner = false;
                    after_banner.push(line);
                }
            }

            format!("{}\n\n{}", new_banner, after_banner.join("\n"))
        } else {
            format!("{}\n\n{}", new_banner, original)
        }
    }
}
