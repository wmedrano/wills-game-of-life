#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Alive,
    NotAlive,
}

pub struct Board {
    tiles: Vec<Cell>,
    width: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            tiles: vec![Cell::NotAlive; width * height],
            width,
        }
    }

    pub fn add_lives(&mut self, coords: impl Iterator<Item = (usize, usize)>) {
        for (x, y) in coords {
            self.add_life(x, y);
        }
    }

    pub fn add_life(&mut self, x: usize, y: usize) {
        let idx = self.index_for_cell(x, y);
        self.tiles[idx] = Cell::Alive;
    }

    pub fn iter_alive(&self) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, state)| Cell::Alive == **state)
            .map(|(idx, _)| self.cell_for_index(idx))
    }

    pub fn next_step(&mut self) -> Board {
        let mut ret = Board::new(self.width, self.height());
        let neighbors = self.count_live_neighbors();
        // Populate cells that survive to the next generation.  These are cells that are currently
        // alive and surround by 2 or 3 neighbors.
        for pos in self.iter_alive() {
            if let 2 | 3 = neighbors.get(&pos).copied().unwrap_or_default() {
                ret.add_life(pos.0, pos.1);
            }
        }
        // Populate cells that are surrounded by exactly 3 neighbors.
        for (pos, cnt) in neighbors.iter() {
            if *cnt == 3 {
                ret.add_life(pos.0, pos.1);
            }
        }
        ret
    }
}

use std::collections::HashMap;

impl Board {
    fn index_for_cell(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn cell_for_index(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn count_live_neighbors(&mut self) -> HashMap<(usize, usize), usize> {
        let mut neighbors = HashMap::new();
        for (x, y) in self.iter_alive() {
            for (x, y) in self.iter_neighbors(x, y) {
                if !neighbors.contains_key(&(x, y)) {
                    neighbors.insert((x, y), 0);
                }
                let val = neighbors.get_mut(&(x, y)).unwrap();
                *val += 1;
            }
        }
        neighbors
    }

    fn iter_neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let x = x as isize;
        let y = y as isize;
        let width = self.width as isize;
        let height = self.height() as isize;
        NS.iter()
            .copied()
            .map(move |(xx, yy)| (x + width + xx, y + height + yy))
            .map(move |(x, y)| (x % width, y % height))
            .map(|(x, y)| (x as usize, y as usize))
    }
}

/// NS contains the relative coordinates of all the neighbors.
const NS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
