use std::collections::HashMap;

// Grid represents a grid of life.
pub struct Grid {
    tiles: Vec<bool>,
    width: usize,
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

impl Grid {
    /// Create a new grid without any life.
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            tiles: vec![false; width * height],
            width,
        }
    }

    /// Add a life to the grid. If the cell is already alive, then nothing happens.
    pub fn add_life(&mut self, x: usize, y: usize) {
        self.tiles[x + y * self.width] = true;
    }

    /// Add several lives to the grid.
    pub fn add_lives(&mut self, coords: impl Iterator<Item = (usize, usize)>) {
        for (x, y) in coords {
            self.add_life(x, y);
        }
    }

    /// Iterate through all the alive cells. The coordinates are returned as a pair of (x, y).
    pub fn iter_alive(&self) -> impl '_ + Iterator<Item = (usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, v)| **v)
            .map(|(idx, _)| (idx % self.width, idx / self.width))
    }

    /// Produce the next step of the simulation. This returns a copy of the grid.
    pub fn next_step(&mut self) -> Grid {
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
        let mut ret = Grid::new(self.width, self.height());
        // Lives to next generation.
        for (x, y) in self.iter_alive() {
            let cnt = neighbors.get(&(x, y)).copied().unwrap_or_default();
            if cnt == 2 || cnt == 3 {
                ret.add_life(x, y);
            }
        }
        // Reproduction.
        for (pos, cnt) in neighbors.iter() {
            if *cnt == 3 {
                ret.add_life(pos.0, pos.1);
            }
        }
        ret
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

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }
}
