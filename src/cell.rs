#[derive(Clone, Default)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    state: bool,
}

impl Cell {
    pub fn new(alive: bool, x: usize, y: usize) -> Self {
        Self { state: alive, x, y }
    }
    pub fn is_alive(&self) -> bool {
        self.state
    }
    pub fn set_state(&mut self, state: bool) {
        self.state = state
    }

    pub fn next_state(&self, nb_neighbours: u8) -> bool {
        if nb_neighbours == 2 || nb_neighbours == 3 {
            return true;
        }
        if self.is_alive() && nb_neighbours == 3 {
            return true;
        }

        return false;
    }
}