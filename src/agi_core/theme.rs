// ================================================================================================
//   SYNTRA KERNEL — AGI CORE (THEME PACK & THEME REGISTRY)
// ------------------------------------------------------------------------------------------------
//   File:        src/agi_core/theme.rs
//   Module:      ThemePack & ThemeRegistry
//   Author:      Alexandr Roussinov (gd2bk1ng)
//   Description:
//       Theme system for SyntraOS. Provides a structured, extensible way to define visual and
//       identity themes for the Control Center, desktop shell, and robot UIs. Supports inheritance
//       (extends), discovery, and runtime switching.
// ================================================================================================

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// Core color representation in RGB.
///
/// We keep it simple and portable so it can be mapped to CSS, GPU shaders,
/// terminal palettes, or robot LED controllers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// High-level color roles used across SyntraOS.
///
/// These are semantic roles, not raw palette indices. This makes it easy
/// to adapt themes across desktop, control center, and robot UIs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeColors {
    pub background_primary: Option<RgbColor>,
    pub background_secondary: Option<RgbColor>,
    pub accent_primary: Option<RgbColor>,
    pub accent_secondary: Option<RgbColor>,
    pub accent_live: Option<RgbColor>,
    pub text_primary: Option<RgbColor>,
    pub text_secondary: Option<RgbColor>,
    pub text_muted: Option<RgbColor>,
    pub warning: Option<RgbColor>,
    pub error: Option<RgbColor>,
}

/// Typography configuration for SyntraOS.
///
/// This is intentionally abstract; concrete frontends (web, desktop, robot HUD)
/// can map these to platform-specific font stacks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeTypography {
    pub font_family: Option<String>,
    pub font_mono: Option<String>,
    pub heading_scale: Option<f32>,
    pub line_height: Option<f32>,
}

/// Layout configuration for panels, spacing, and density.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeLayout {
    pub border_radius: Option<u8>,
    pub spacing_unit: Option<u8>,
    pub panel_padding: Option<u8>,
    pub grid_density: Option<String>, // e.g., "compact", "comfortable", "spacious"
}

/// Component-specific skinning.
///
/// These are high-level knobs; concrete UIs can interpret them as needed.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeComponents {
    pub panel_background: Option<String>, // references a color role key
    pub panel_border_color: Option<String>,
    pub panel_border_width: Option<u8>,

    pub button_primary_background: Option<String>,
    pub button_primary_text: Option<String>,
    pub button_hover_brightness: Option<f32>,

    pub terminal_background: Option<RgbColor>,
    pub terminal_text: Option<RgbColor>,
}

/// Animation tuning for SyntraOS.
///
/// Used by boot loader, control center transitions, robot HUDs, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeAnimations {
    pub boot_sequence_speed: Option<f32>,
    pub hover_transition_ms: Option<u64>,
    pub panel_slide_ms: Option<u64>,
}

/// Identity layer for branding and institutional customization.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeIdentity {
    pub logo_path: Option<String>,
    pub watermark: Option<String>,
    pub institution: Option<String>,
}

/// A single theme pack definition.
///
/// Theme packs can extend other themes via `extends`, allowing institutions
/// to define child themes that override only a subset of properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePack {
    pub name: String,
    pub version: Option<String>,
    pub extends: Option<String>,

    #[serde(default)]
    pub colors: ThemeColors,

    #[serde(default)]
    pub typography: ThemeTypography,

    #[serde(default)]
    pub layout: ThemeLayout,

    #[serde(default)]
    pub components: ThemeComponents,

    #[serde(default)]
    pub animations: ThemeAnimations,

    #[serde(default)]
    pub identity: ThemeIdentity,
}

impl ThemePack {
    /// Merge `other` into `self`, with `other` taking precedence where it has values.
    ///
    /// This is used to implement inheritance (`extends`).
    pub fn merge(&mut self, other: &ThemePack) {
        // Colors
        if other.colors.background_primary.is_some() {
            self.colors.background_primary = other.colors.background_primary;
        }
        if other.colors.background_secondary.is_some() {
            self.colors.background_secondary = other.colors.background_secondary;
        }
        if other.colors.accent_primary.is_some() {
            self.colors.accent_primary = other.colors.accent_primary;
        }
        if other.colors.accent_secondary.is_some() {
            self.colors.accent_secondary = other.colors.accent_secondary;
        }
        if other.colors.accent_live.is_some() {
            self.colors.accent_live = other.colors.accent_live;
        }
        if other.colors.text_primary.is_some() {
            self.colors.text_primary = other.colors.text_primary;
        }
        if other.colors.text_secondary.is_some() {
            self.colors.text_secondary = other.colors.text_secondary;
        }
        if other.colors.text_muted.is_some() {
            self.colors.text_muted = other.colors.text_muted;
        }
        if other.colors.warning.is_some() {
            self.colors.warning = other.colors.warning;
        }
        if other.colors.error.is_some() {
            self.colors.error = other.colors.error;
        }

        // Typography
        if other.typography.font_family.is_some() {
            self.typography.font_family = other.typography.font_family.clone();
        }
        if other.typography.font_mono.is_some() {
            self.typography.font_mono = other.typography.font_mono.clone();
        }
        if other.typography.heading_scale.is_some() {
            self.typography.heading_scale = other.typography.heading_scale;
        }
        if other.typography.line_height.is_some() {
            self.typography.line_height = other.typography.line_height;
        }

        // Layout
        if other.layout.border_radius.is_some() {
            self.layout.border_radius = other.layout.border_radius;
        }
        if other.layout.spacing_unit.is_some() {
            self.layout.spacing_unit = other.layout.spacing_unit;
        }
        if other.layout.panel_padding.is_some() {
            self.layout.panel_padding = other.layout.panel_padding;
        }
        if other.layout.grid_density.is_some() {
            self.layout.grid_density = other.layout.grid_density.clone();
        }

        // Components
        if other.components.panel_background.is_some() {
            self.components.panel_background = other.components.panel_background.clone();
        }
        if other.components.panel_border_color.is_some() {
            self.components.panel_border_color = other.components.panel_border_color.clone();
        }
        if other.components.panel_border_width.is_some() {
            self.components.panel_border_width = other.components.panel_border_width;
        }
        if other.components.button_primary_background.is_some() {
            self.components.button_primary_background =
                other.components.button_primary_background.clone();
        }
        if other.components.button_primary_text.is_some() {
            self.components.button_primary_text =
                other.components.button_primary_text.clone();
        }
        if other.components.button_hover_brightness.is_some() {
            self.components.button_hover_brightness =
                other.components.button_hover_brightness;
        }
        if other.components.terminal_background.is_some() {
            self.components.terminal_background = other.components.terminal_background;
        }
        if other.components.terminal_text.is_some() {
            self.components.terminal_text = other.components.terminal_text;
        }

        // Animations
        if other.animations.boot_sequence_speed.is_some() {
            self.animations.boot_sequence_speed = other.animations.boot_sequence_speed;
        }
        if other.animations.hover_transition_ms.is_some() {
            self.animations.hover_transition_ms = other.animations.hover_transition_ms;
        }
        if other.animations.panel_slide_ms.is_some() {
            self.animations.panel_slide_ms = other.animations.panel_slide_ms;
        }

        // Identity
        if other.identity.logo_path.is_some() {
            self.identity.logo_path = other.identity.logo_path.clone();
        }
        if other.identity.watermark.is_some() {
            self.identity.watermark = other.identity.watermark.clone();
        }
        if other.identity.institution.is_some() {
            self.identity.institution = other.identity.institution.clone();
        }
    }
}

/// Registry of all discovered themes.
///
/// This is the backbone for:
///   - Control Center theme selection
///   - Robot UI theming
///   - Institution branding
pub struct ThemeRegistry {
    themes: HashMap<String, ThemePack>,
    themes_dir: PathBuf,
}

impl ThemeRegistry {
    /// Create a new registry pointing at a themes directory.
    pub fn new(themes_dir: impl AsRef<Path>) -> Self {
        ThemeRegistry {
            themes: HashMap::new(),
            themes_dir: themes_dir.as_ref().to_path_buf(),
        }
    }

    /// Load all themes from the themes directory.
    ///
    /// Expected layout:
    ///   themes/
    ///     syntra_default/theme.json
    ///     custom_theme/theme.json
    pub fn load_all(&mut self) {
        self.themes.clear();

        let dir = &self.themes_dir;
        if !dir.exists() {
            warn!(
                "ThemeRegistry: themes directory does not exist: {}",
                dir.display()
            );
            return;
        }

        let Ok(entries) = fs::read_dir(dir) else {
            warn!(
                "ThemeRegistry: failed to read themes directory: {}",
                dir.display()
            );
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let theme_file = path.join("theme.json");
            if !theme_file.exists() {
                continue;
            }

            match fs::read_to_string(&theme_file) {
                Ok(contents) => match serde_json::from_str::<ThemePack>(&contents) {
                    Ok(theme) => {
                        info!(
                            "ThemeRegistry: loaded theme '{}' from {}",
                            theme.name,
                            theme_file.display()
                        );
                        self.themes.insert(theme.name.clone(), theme);
                    }
                    Err(e) => {
                        warn!(
                            "ThemeRegistry: failed to parse theme.json at {}: {:?}",
                            theme_file.display(),
                            e
                        );
                    }
                },
                Err(e) => {
                    warn!(
                        "ThemeRegistry: failed to read theme.json at {}: {:?}",
                        theme_file.display(),
                        e
                    );
                }
            }
        }

        // Resolve inheritance after all themes are loaded.
        self.resolve_inheritance();
    }

    /// Resolve `extends` relationships between themes.
    ///
    /// This merges parent themes into child themes, with child values taking precedence.
    fn resolve_inheritance(&mut self) {
        // We clone keys to avoid borrowing issues while mutating.
        let theme_names: Vec<String> = self.themes.keys().cloned().collect();

        for name in theme_names {
            // We use a small stack to walk up the inheritance chain.
            let mut chain: Vec<String> = Vec::new();
            let mut current = name.clone();

            while let Some(theme) = self.themes.get(&current) {
                if let Some(parent_name) = &theme.extends {
                    if chain.contains(parent_name) {
                        warn!(
                            "ThemeRegistry: detected inheritance cycle involving '{}'",
                            parent_name
                        );
                        break;
                    }
                    chain.push(parent_name.clone());
                    current = parent_name.clone();
                } else {
                    break;
                }
            }

            // Apply inheritance from root parent down to the child.
            // We rebuild the merged theme by starting from the root-most parent.
            if let Some(child_theme) = self.themes.get(&name).cloned() {
                let mut merged = child_theme.clone();

                for parent_name in chain.into_iter().rev() {
                    if let Some(parent_theme) = self.themes.get(&parent_name) {
                        let mut parent_clone = parent_theme.clone();
                        parent_clone.merge(&merged);
                        merged = parent_clone;
                    } else {
                        warn!(
                            "ThemeRegistry: theme '{}' extends missing parent '{}'",
                            name, parent_name
                        );
                    }
                }

                self.themes.insert(name.clone(), merged);
            }
        }
    }

    /// Get a theme by name.
    pub fn get(&self, name: &str) -> Option<&ThemePack> {
        self.themes.get(name)
    }

    /// List all available theme names.
    pub fn list(&self) -> Vec<String> {
        let mut names: Vec<String> = self.themes.keys().cloned().collect();
        names.sort();
        names
    }

    /// Reload themes from disk.
    ///
    /// This can be wired to a "Scan for new themes" button in the Control Center.
    pub fn reload(&mut self) {
        info!("ThemeRegistry: reloading themes from {}", self.themes_dir.display());
        self.load_all();
    }
}
