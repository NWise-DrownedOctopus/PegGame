pub struct Grid {
    cells: [Cell; 33],
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: i32,
    y: i32,
    has_peg: bool,
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
            if index == 3 {
                cells[index] = Cell { x, y: 3, has_peg: false };
                index += 1;
                continue;
            }
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

        Grid { cells }
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.x == x && cell.y ==y)
    }
}