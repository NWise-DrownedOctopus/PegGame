pub struct Grid {
    pub cells: [Cell; 33],
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub has_peg: bool,
}

impl Grid {
    pub fn new() -> Grid {
        let mut cells = [Cell { x: 0, y: 0 , has_peg: true}; 33]; // temporary default values
        let mut index = 0;

        // Row 0
        for x in 2..=4 {
            cells[index] = Cell { x, y: 0, has_peg: true };
            index += 1;
        }

        // Row 1
        for x in 2..=4 {
            cells[index] = Cell { x, y: 1, has_peg: true };
            index += 1;
        }

        // Row 2
        for x in 0..=6 {
            cells[index] = Cell { x, y: 2, has_peg: true };
            index += 1;
        }

        // Row 3
        for x in 0..=6 {
            cells[index] = Cell { x, y: 3, has_peg: true };
            index += 1;
        }

        // Row 4
        for x in 0..=6 {
            cells[index] = Cell { x, y: 4, has_peg: true };
            index += 1;
        }

        // Row 5
        for x in 2..=4 {
            cells[index] = Cell { x, y: 5, has_peg: true };
            index += 1;
        }

        // Row 6
        for x in 2..=4 {
            cells[index] = Cell { x, y: 6, has_peg: true };
            index += 1;
        }

        cells[16] = Cell { x: 3, y: 3, has_peg: false};

        Grid { cells }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.x == x && cell.y ==y)
    }

    pub fn get_cell_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        self.cells.iter_mut().find(|cell| cell.x == x && cell.y == y)
    }

    pub fn check_move(&self, start_cell: &Cell, destination_cell: &Cell) -> bool {
        let dx = (start_cell.x - destination_cell.x).abs();
        let dy = (start_cell.y - destination_cell.y).abs();

        // First check for basic issues
        if !start_cell.has_peg || destination_cell.has_peg {
            println!("An invalid move has been selected, selected invalid cells");
            println!(
                "start peg: {}, destination peg: {}",
                start_cell.has_peg,
                destination_cell.has_peg
            );
            return false
        }

        // Lets make sure the cell is the right distance away
        if !((dx == 2 && dy == 0) || (dx == 0 && dy == 2)) {
            println!("An invalid move has been selected, distance incorrect");
            return false
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
                    return true
                }
                else {
                    println!("An invalid move has been selected, mid cell has no peg");
                    return false
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_horizontal_jump() {
        let grid = Grid::new();

        let start = grid.get_cell(1, 3).unwrap();
        let dest = grid.get_cell(3, 3).unwrap(); // This is the center cell

        assert!(grid.check_move(start, dest));
    }

    #[test]
    fn valid_vertical_jump() {
        let grid = Grid::new();

        let start = grid.get_cell(3, 1).unwrap();
        let dest = grid.get_cell(3, 3).unwrap(); // This is the center cell

        assert!(grid.check_move(start, dest));
    }

    #[test]
    fn invalid_move_no_start_peg() {
        let grid = Grid::new();

        let start = grid.get_cell(3, 3).unwrap(); // This is the center cell
        let dest = grid.get_cell(3, 1).unwrap();

        assert!(!grid.check_move(start, dest));        
    }

    #[test]
    fn invalid_move_destination_occupied() {
        let grid = Grid::new();

        let start = grid.get_cell(3, 0).unwrap();
        let dest = grid.get_cell(3, 2).unwrap(); // This cell has a peg

        assert!(!grid.check_move(start, dest));
    }

    #[test]
    fn invalid_move_wrong_distance() {
        let grid = Grid::new();

        let start = grid.get_cell(0, 2).unwrap();
        let dest = grid.get_cell(3, 2).unwrap(); // This cell is too far

        assert!(!grid.check_move(start, dest));
    }

    #[test]
    fn make_move_updates_cells() {
        let mut grid = Grid::new();

        let start = (3, 1);
        let dest = (3, 3);

        grid.make_move(start, dest);

        assert!(!grid.get_cell(3, 1).unwrap().has_peg); // start cleared
        assert!(!grid.get_cell(3, 2).unwrap().has_peg); // middle cleared
        assert!(grid.get_cell(3, 3).unwrap().has_peg);  // destination filled
    }

    #[test]
    fn new_game_has_valid_moves() {
        let grid = Grid::new();

        assert!(grid.has_any_valid_move());
    }

    #[test]
    fn board_with_no_moves_is_game_over() {
        let mut grid = Grid::new();

        // Clear the board
        for cell in grid.cells.iter_mut() {
            cell.has_peg = false;
        }

        // Adding isolated pegs with no valid moves possible
        grid.get_cell_mut(0,2).unwrap().has_peg = true;
        grid.get_cell_mut(6,2).unwrap().has_peg = true;
        grid.get_cell_mut(3,6).unwrap().has_peg = true;

        assert!(!grid.has_any_valid_move());
    }

    #[test]
    fn board_with_single_valid_move() {
        let mut grid = Grid::new();

        // Clear board
        for cell in grid.cells.iter_mut() {
            cell.has_peg = false;
        }

        // Create one valid jump
        grid.get_cell_mut(3,1).unwrap().has_peg = true;
        grid.get_cell_mut(3,2).unwrap().has_peg = true;

        assert!(grid.has_any_valid_move());
    }
}
