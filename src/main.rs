use piston_window::{types::Color, *};

const WINDOW_SIZE: (u32, u32) = (640, 480);

struct Palette;

impl Palette {
    fn background() -> Color {
        // Payne's gray: #5E6472
        rgb_to_color(94, 100, 114)
    }

    fn foreground() -> Color {
        // Celeste: #B8F2E6
        rgb_to_color(184, 242, 230)
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Game1", WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build window: {}", e));
    while let Some(e) = window.next() {
        let window_transform = math::scale(1.0 / WINDOW_SIZE.0 as f64, 1.0 / WINDOW_SIZE.1 as f64);
        window.draw_2d(&e, |_c, g, _d| {
            clear(Palette::background(), g);
            ellipse_from_to(
                Palette::foreground(),
                [16.0, 16.0],
                [48.0, 48.0],
                window_transform,
                g,
            );
        });
    }
}

fn rgb_to_color(r: u8, g: u8, b: u8) -> Color {
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]
}
