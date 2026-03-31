use rand::seq::SliceRandom;

pub struct Grid {
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub has_peg: bool,
}

impl Grid {
    pub fn new(size: i32) -> Grid {
        let mut cells = Vec::new();

        let mid = size / 2;
        let arm_half = 1; // since center is always width 3

        for y in 0..size {
            for x in 0..size {
                let in_vertical_band = x >= mid - arm_half && x <= mid + arm_half;
                let in_horizontal_band = y >= mid - arm_half && y <= mid + arm_half;

                if in_vertical_band || in_horizontal_band {
                    cells.push(Cell {
                        x,
                        y,
                        has_peg: true,
                    });
                }
            }
        }

        // Remove center peg
        if let Some(center) = cells.iter_mut().find(|c| c.x == mid && c.y == mid) {
            center.has_peg = false;
        }

        Grid { cells }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.x == x && cell.y == y)
    }

    pub fn get_cell_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        self.cells
            .iter_mut()
            .find(|cell| cell.x == x && cell.y == y)
    }

    pub fn check_move(&self, start_cell: &Cell, destination_cell: &Cell) -> bool {
        println!("We called check_move");
        let dx = (start_cell.x - destination_cell.x).abs();
        let dy = (start_cell.y - destination_cell.y).abs();

        // First check for basic issues
        if !start_cell.has_peg || destination_cell.has_peg {
            println!("An invalid move has been selected, selected invalid cells");
            println!(
                "start peg: {}, destination peg: {}",
                start_cell.has_peg, destination_cell.has_peg
            );
            return false;
        }

        // Lets make sure the cell is the right distance away
        if !((dx == 2 && dy == 0) || (dx == 0 && dy == 2)) {
            println!("An invalid move has been selected, distance incorrect");
            return false;
        }

        // Now lets check the middle cell
        let mid_x = (start_cell.x + destination_cell.x) / 2;
        let mid_y = (start_cell.y + destination_cell.y) / 2;

        let mid_cell = self.get_cell(mid_x, mid_y);

        match mid_cell {
            None => return false,
            Some(mid_cell) => {
                if mid_cell.has_peg == true {
                    println!("A valid move has been selected");
                    return true;
                } else {
                    println!("An invalid move has been selected, mid cell has no peg");
                    return false;
                }
            }
        }
    }

    pub fn has_any_valid_move(&self) -> bool {
        for start_cell in self.cells.iter() {
            if !start_cell.has_peg {
                continue;
            }

            let possible_moves = [
                (start_cell.x + 2, start_cell.y),
                (start_cell.x - 2, start_cell.y),
                (start_cell.x, start_cell.y + 2),
                (start_cell.x, start_cell.y - 2),
            ];

            for (dest_x, dest_y) in possible_moves {
                if let Some(dest_cell) = self.get_cell(dest_x, dest_y) {
                    if self.check_move(start_cell, dest_cell) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_all_valid_moves(&self) -> Vec<((i32, i32), (i32, i32))> {
        let mut valid_moves = Vec::new();

        for start_cell in self.cells.iter() {
            if !start_cell.has_peg {
                continue;
            }

            let possible_destinations = [
                (start_cell.x + 2, start_cell.y),
                (start_cell.x - 2, start_cell.y),
                (start_cell.x, start_cell.y + 2),
                (start_cell.x, start_cell.y - 2),
            ];

            for (dest_x, dest_y) in possible_destinations {
                if let Some(dest_cell) = self.get_cell(dest_x, dest_y) {
                    if self.check_move(start_cell, dest_cell) {
                        valid_moves.push(((start_cell.x, start_cell.y), (dest_x, dest_y)));
                    }
                }
            }
        }

        valid_moves
    }

    pub fn make_move(&mut self, start: (i32, i32), dest: (i32, i32)) {
        let mid_x = (start.0 + dest.0) / 2;
        let mid_y = (start.1 + dest.1) / 2;

        if let Some(start_cell) = self.get_cell_mut(start.0, start.1) {
            start_cell.has_peg = false;
        }
        if let Some(mid_cell) = self.get_cell_mut(mid_x, mid_y) {
            mid_cell.has_peg = false;
        }
        if let Some(dest_cell) = self.get_cell_mut(dest.0, dest.1) {
            dest_cell.has_peg = true;
        }
    }

    pub fn randomize_pegs(&mut self) {
        let peg_count = self.cells.iter().filter(|c| c.has_peg).count();
        
        let mut rng = rand::thread_rng();
        self.cells.shuffle(&mut rng);
        
        for (i, cell) in self.cells.iter_mut().enumerate() {
            cell.has_peg = i < peg_count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_horizontal_jump() {
        let grid = Grid::new(7);

        let start = grid.get_cell(1, 3).unwrap();
        let dest = grid.get_cell(3, 3).unwrap();

        assert!(grid.check_move(start, dest));
    }

    #[test]
    fn valid_vertical_jump() {
        let grid = Grid::new(7);

        let start = grid.get_cell(3, 1).unwrap();
        let dest = grid.get_cell(3, 3).unwrap();

        assert!(grid.check_move(start, dest));
    }
}
