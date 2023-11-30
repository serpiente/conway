use crate::cell::Cell;
use crate::point::Point;
use rayon::prelude::*;
use rand::Rng;

pub struct Grid {
    width: usize,
    height: usize,
    pub(crate) cells: Vec<Cell>,
}


impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Cell> = vec![];
        let mut rng = rand::thread_rng();

        for x in 0..width {
            for y in 0..height {
                cells.push(
                    Cell::new(
                        rng.gen::<bool>(),
                        x,
                        y,
                    )
                )
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    fn within_coords(&self, point: &Point) -> bool {
        return point.x >= 0 && point.x <= self.width && point.y >= 0 && point.y <= self.height;
    }


    pub fn point_from_idx(&self, idx: usize) -> Option<Point> {
        if idx > self.cells.len() {
            None
        } else {
            Some(Point { x: idx % self.width, y: idx / self.height })
        }
    }


    pub fn get_cell(&self, point: &Point) -> Option<&Cell> {
        if let Some(idx) = self.idx_from_point(point) {
            Some(&self.cells[idx])
        } else {
            None
        }
    }

    pub fn idx_from_point(&self, point: &Point) -> Option<usize> {
        if point.x * self.width + point.y < self.cells.len() {
            Some(point.x * self.width + point.y)
        } else {
            None
        }
    }

    fn get_neighbours(&self, cell: &Cell) -> Vec<&Cell> {
        let mut neighbours: Vec<&Cell> = vec![];
        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                if *dx == 0 && *dy == 0 { //dont care about itself
                    continue;
                } else {
                    let tuple = (cell.x as isize + dx, cell.y as isize + dy);

                    if let Ok(neighbour_point) = &tuple.try_into() {
                        if let Some(cell) = self.get_cell(neighbour_point) {
                            neighbours.push(cell)
                        }
                    }
                }
            }
        }
        neighbours
    }

    pub fn nb_alive_neighbors(&self, cell: &Cell) -> u8 {
        let mut alive_neighbors = 0;
        for neighbour in self.get_neighbours(cell) {
            if neighbour.is_alive() {
                alive_neighbors += 1
            }
        }
        alive_neighbors
    }

    pub fn update(&mut self) {
        let next_states = (0..self.cells.len())
            .into_par_iter()
            .map(|idx| {
                let cell = &self.cells[idx];
                let nb_alive_neighbors = self.nb_alive_neighbors(cell);
                let next_state = cell.next_state(nb_alive_neighbors);
                next_state
            }).collect::<Vec<bool>>();

        for idx in 0..self.cells.len() {
            let cell = &self.cells[idx];
            self.cells[idx] = Cell::new(next_states[idx], cell.x, cell.y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100); // width * height
    }
}