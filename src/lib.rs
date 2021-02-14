mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        //let width = 64;
        //let height = 64;
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                let column = i %width;
                let row = i / width;

                if row > 4 && row <= 16 && column > width - row - 3*width/4+1 && column < width/4 {
                    Cell::Alive
                } else if row > 4 && row <= 16 && column > width/4-1 && column < width/4 + row - 1 {
                    Cell::Alive
                } else if row > 4 && row <= 16 && column > width/2-1 && column > width - row - width/4+1 && column < 3*width/4 {
                    Cell::Alive
                } else if row > 4 && row <= 16 && column > 3*width/4-1 && column < 3*width/4 + row - 1 {
                    Cell::Alive
                } else if row > 16 && row <= 24 && ((column > 1 && column < width/4) || ( column > 3*width/4 && column < width - 1)) {
                    Cell::Alive
                } else if row > 24 && column > row-24 && column < (width - row + 24) {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn add_glider(&mut self, row: u32, column: u32) {
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                if delta_row == 1 && delta_col == self.width - 1 {
                    self.cells[idx] = Cell::Dead;
                    continue
                }
                if delta_row == 1 && delta_col == 0 {
                    self.cells[idx] = Cell::Alive;
                    continue
                }
                if delta_row == 1 && delta_col == 1 {
                    self.cells[idx] = Cell::Dead;
                    continue
                }
                if delta_row == 0 && delta_col == self.width - 1 {
                    self.cells[idx] = Cell::Dead;
                    continue
                }
                if delta_row == 0 && delta_col == 0 {
                    self.cells[idx] = Cell::Dead;
                    continue
                }
                if delta_row == 0 && delta_col == 1 {
                    self.cells[idx] = Cell::Alive;
                    continue
                }
                if delta_row == self.height - 1 {
                    self.cells[idx] = Cell::Alive;
                    continue
                }
            }
        }
    }
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}
