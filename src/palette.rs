pub struct Palette;

impl Palette {
    pub fn background(&self) -> Color {
        // Midnight green.
        rgb_to_color([7, 59, 76])
    }

    pub fn foreground(&self) -> Color {
        // Sunglow.
        rgb_to_color([255, 209, 102])
    }
}

use piston_window::types::Color;

/// Converts rgb into a piston color.
fn rgb_to_color(rgb: [u8; 3]) -> Color {
    [
        rgb[0] as f32 / 255.0,
        rgb[1] as f32 / 255.0,
        rgb[2] as f32 / 255.0,
        1.0,
    ]
}
