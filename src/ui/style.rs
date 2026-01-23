/* ================================================================================================
   SYNTRA BROWSER — AXIOM ZERO
   ------------------------------------------------------------------------------------------------
   File:        src/ui/style.rs
   Module:      UI Style Configuration
   Author:      Alexandr Roussinov (gd2bk1ng)
   Description: Defines UI style configuration structures and APIs for dynamic
                runtime control of colors, shapes, and fonts in Syntra Browser.

   Notes:
     - Supports serde serialization for persistence.
     - Provides setters to update style properties programmatically.
   ================================================================================================ */

use serde::{Deserialize, Serialize};

/// Defines color themes for the browser UI.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorTheme {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub highlight: String,
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self {
            background: "#1E1E2F".to_string(),
            foreground: "#D9E0EE".to_string(),
            accent: "#96CDFB".to_string(),
            highlight: "#F5C2E7".to_string(),
        }
    }
}

/// Defines shape styles for UI elements.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShapeStyle {
    pub border_radius: f32,
    pub shadow_strength: f32,
}

impl Default for ShapeStyle {
    fn default() -> Self {
        Self {
            border_radius: 6.0,
            shadow_strength: 0.25,
        }
    }
}

/// The overall UI style configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleConfig {
    pub colors: ColorTheme,
    pub shapes: ShapeStyle,
    pub font_family: String,
    pub font_size: u32,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            colors: ColorTheme::default(),
            shapes: ShapeStyle::default(),
            font_family: "Inter".to_string(),
            font_size: 14,
        }
    }
}

impl StyleConfig {
    /// Set a color property by key.
    pub fn set_color(&mut self, key: &str, value: String) -> Result<(), String> {
        match key {
            "background" => self.colors.background = value,
            "foreground" => self.colors.foreground = value,
            "accent" => self.colors.accent = value,
            "highlight" => self.colors.highlight = value,
            _ => return Err(format!("Unknown color key: {}", key)),
        }
        Ok(())
    }

    /// Set a shape property by key.
    pub fn set_shape(&mut self, key: &str, value: f32) -> Result<(), String> {
        match key {
            "border_radius" => self.shapes.border_radius = value,
            "shadow_strength" => self.shapes.shadow_strength = value,
            _ => return Err(format!("Unknown shape key: {}", key)),
        }
        Ok(())
    }

    /// Set the font family.
    pub fn set_font_family(&mut self, family: String) {
        self.font_family = family;
    }

    /// Set the font size.
    pub fn set_font_size(&mut self, size: u32) {
        self.font_size = size;
    }
}
