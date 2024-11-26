//! Module for maze generation logic.

use rand::seq::SliceRandom;
use rand::thread_rng;

/// Represents a cell in the maze grid.
#[derive(Clone, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub walls: [bool; 4], // [Top, Right, Bottom, Left]
    pub visited: bool,
}

impl Cell {
    /// Create a new cell with all walls intact.
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            walls: [true; 4],
            visited: false,
        }
    }
}

/// Represents a maze grid.
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Cell>,
}

impl Maze {
    /// Create a new maze with uninitialized cells.
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                grid.push(Cell::new(x, y));
            }
        }
        Self {
            width,
            height,
            grid,
        }
    }

    /// Generate a maze using recursive backtracking.
    pub fn generate(&mut self) {
        let mut rng = thread_rng();
        let mut stack = Vec::new();

        let current = &mut self.grid[0];
        current.visited = true;
        stack.push(current.clone());

        while let Some(cell) = stack.pop() {
            let unvisited_neighbors = self.get_unvisited_neighbors(cell.x, cell.y);

            if !unvisited_neighbors.is_empty() {
                stack.push(cell.clone());

                let next = unvisited_neighbors.choose(&mut rng).unwrap();
                self.remove_wall_between(&cell, next);

                let next_ref = self
                    .grid
                    .iter_mut()
                    .find(|c| c.x == next.x && c.y == next.y)
                    .unwrap();
                next_ref.visited = true;

                stack.push(next_ref.clone());
            }
        }
    }

    /// Get unvisited neighbors of a cell.
    fn get_unvisited_neighbors(&self, x: usize, y: usize) -> Vec<Cell> {
        let mut neighbors = Vec::new();

        if y > 0 {
            let top = &self.grid[(y - 1) * self.width + x];
            if !top.visited {
                neighbors.push(top.clone());
            }
        }
        if x < self.width - 1 {
            let right = &self.grid[y * self.width + (x + 1)];
            if !right.visited {
                neighbors.push(right.clone());
            }
        }
        if y < self.height - 1 {
            let bottom = &self.grid[(y + 1) * self.width + x];
            if !bottom.visited {
                neighbors.push(bottom.clone());
            }
        }
        if x > 0 {
            let left = &self.grid[y * self.width + (x - 1)];
            if !left.visited {
                neighbors.push(left.clone());
            }
        }

        neighbors
    }

    /// Removes the wall between two adjacent cells.
    fn remove_wall_between(&mut self, current: &Cell, next: &Cell) {
        let dx = next.x as isize - current.x as isize;
        let dy = next.y as isize - current.y as isize;

        
        {
            let current_cell = self
                .grid
                .iter_mut()
                .find(|c| c.x == current.x && c.y == current.y)
                .expect("Current cell not found in grid");

            if dx == 1 {
                current_cell.walls[1] = false; 
            } else if dx == -1 {
                current_cell.walls[3] = false; 
            } else if dy == 1 {
                current_cell.walls[2] = false; 
            } else if dy == -1 {
                current_cell.walls[0] = false; 
            }
        }

        {
            let next_cell = self
                .grid
                .iter_mut()
                .find(|c| c.x == next.x && c.y == next.y)
                .expect("Next cell not found in grid");

            if dx == 1 {
                next_cell.walls[3] = false; 
            } else if dx == -1 {
                next_cell.walls[1] = false;
            } else if dy == 1 {
                next_cell.walls[0] = false;
            } else if dy == -1 {
                next_cell.walls[2] = false; 
            }
        }
    }
}
