use board::Board;
use palette::Palette;
use piston_window::{
    clear, rectangle_from_to, Event, Loop, PistonWindow, Transformed, WindowSettings,
};
use std::time::{Duration, Instant};

mod board;
mod palette;
mod settings;

fn initial_board_impl() -> Board {
    let mut board = Board::new(
        settings::WINDOW_SIZE.0 as usize / settings::TILE_SIZE,
        settings::WINDOW_SIZE.1 as usize / settings::TILE_SIZE,
    );
    board.add_lives(
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
    board
}

struct State {
    board: Board,
    last_update: Instant,
}

impl Default for State {
    fn default() -> State {
        State {
            board: initial_board(),
            last_update: Instant::now(),
        }
    }
}

fn initial_board() -> Board {
    initial_board_impl()
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Will's Game of Life", settings::WINDOW_SIZE)
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build window: {}", e));
    let mut state = State::default();
    while let Some(e) = window.next() {
        if let Event::Loop(Loop::Render(_)) = e {
            maybe_update_board(&mut state.board, &mut state.last_update);
            render(&mut window, &e, &state.board);
        }
    }
}

fn maybe_update_board(board: &mut Board, last_updated: &mut Instant) {
    let duration_per_update = Duration::from_millis(50);
    if last_updated.elapsed() < duration_per_update {
        return;
    }
    *last_updated = Instant::now();
    *board = board.next_step();
}

fn render(window: &mut piston_window::PistonWindow, e: &piston_window::Event, board: &Board) {
    window.draw_2d(e, |c, g, _| {
        clear(Palette.background(), g);
        for (x, y) in board.iter_alive() {
            rectangle_from_to(
                Palette.foreground(),
                [x as f64, y as f64],
                [x as f64 + 1.0, y as f64 + 1.0],
                c.transform
                    .scale(settings::TILE_SIZE as f64, settings::TILE_SIZE as f64),
                g,
            );
        }
    });
}
