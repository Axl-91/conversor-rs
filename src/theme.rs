//! Shared color palette for the user interface.
//!
//! Keeping the colors here means the whole application can be re-themed by
//! editing a single file. All colors are chosen to read well on both the light
//! and dark egui default themes.

use eframe::egui::Color32;

/// Main accent color, used for the header and the primary button.
pub(crate) const ACCENT: Color32 = Color32::from_rgb(99, 102, 241);

/// Darker shade of the accent, used for buttons placed on the header.
pub(crate) const ACCENT_DARK: Color32 = Color32::from_rgb(67, 56, 202);

/// Lighter shade of the accent, used for text and secondary elements.
pub(crate) const ACCENT_LIGHT: Color32 = Color32::from_rgb(129, 140, 248);

/// Color used for a successful conversion result.
pub(crate) const SUCCESS: Color32 = Color32::from_rgb(34, 160, 90);

/// Color used for errors and unavailable rates.
pub(crate) const ERROR: Color32 = Color32::from_rgb(220, 70, 70);

/// Color used for neutral hints before the first conversion.
pub(crate) const INFO: Color32 = Color32::from_rgb(120, 130, 150);

/// White, used for text drawn on top of the colored header.
pub(crate) const ON_ACCENT: Color32 = Color32::WHITE;

/// Returns a semi-transparent version of `color`.
///
/// This is useful for softly tinted card backgrounds that still let the
/// underlying theme show through.
pub(crate) fn tinted(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}
