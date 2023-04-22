use piston_window::types::Color;

// Contains a color palette.
// https://coolors.co/palette/ef476f-ffd166-06d6a0-118ab2-073b4c
pub struct Palette;

impl Palette {
    /// Returns the background color.
    pub fn background(&self) -> Color {
        rgb_to_color(7, 59, 76)
    }

    /// Returns the foreground color.
    pub fn foreground(&self) -> Color {
        rgb_to_color(255, 209, 102)
    }
}

/// Converts rgb into a piston color.
fn rgb_to_color(r: u8, g: u8, b: u8) -> Color {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]
}
