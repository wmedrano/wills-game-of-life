use std::time::{Duration, Instant};

use grid::Grid;
use palette::Palette;
use piston_window::{
    clear, rectangle_from_to, Event, Loop, PistonWindow, Transformed, WindowSettings,
};

mod grid;
mod palette;

const WINDOW_SIZE: (u32, u32) = (640, 480);
const TILE_SIZE: usize = 10;

struct State {
    // The state of the grid.
    grid: Grid,
    // The last time that the grid was updated.
    last_update: Instant,
}

impl Default for State {
    fn default() -> State {
        State {
            grid: initial_grid(),
            last_update: Instant::now(),
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Will's Game of Life", WINDOW_SIZE)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build window: {}", e));
    let mut state = State::default();
    while let Some(e) = window.next() {
        if let Event::Loop(Loop::Render(_)) = e {
            maybe_update_grid(&mut state.grid, &mut state.last_update);
            render(&mut window, &e, &state.grid);
        }
    }
}

// Update's the grid if more than 200 milliseconds have pass since last_update. If the grid is
// updated, `last_updated` will hold the current time.
fn maybe_update_grid(grid: &mut Grid, last_updated: &mut Instant) {
    let duration_per_update = Duration::from_millis(200);
    if last_updated.elapsed() < duration_per_update {
        return;
    }
    *last_updated = Instant::now();
    *grid = grid.next_step();
}

/// Render's the contents onto window.
fn render(window: &mut piston_window::PistonWindow, e: &piston_window::Event, grid: &Grid) {
    window.draw_2d(e, |c, g, _| {
        clear(Palette.background(), g);
        for (x, y) in grid.iter_alive() {
            rectangle_from_to(
                Palette.foreground(),
                [x as f64, y as f64],
                [x as f64 + 1.0, y as f64 + 1.0],
                c.transform.scale(10.0, 10.0),
                g,
            );
        }
    });
}

fn initial_grid() -> Grid {
    let mut grid = Grid::new(
        WINDOW_SIZE.0 as usize / TILE_SIZE,
        WINDOW_SIZE.1 as usize / TILE_SIZE,
    );
    grid.add_lives(
        [
            (10, 10),
            (11, 10),
            (10, 11),
            (11, 11),
            (20, 20),
            (20, 21),
            (20, 22),
            (3, 2),
            (4, 2),
            (5, 2),
            (5, 3),
            (4, 4),
            (33, 2),
            (34, 2),
            (35, 2),
            (35, 3),
            (34, 4),
            (3, 32),
            (4, 32),
            (5, 32),
            (5, 33),
            (4, 34),
            (13, 32),
            (14, 32),
            (15, 32),
            (15, 33),
            (14, 34),
            (53, 32),
            (54, 32),
            (55, 32),
            (55, 33),
            (54, 34),
            (0, 0),
            (9, 9),
        ]
        .into_iter(),
    );
    grid
}
