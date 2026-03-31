use rand::seq::SliceRandom;
use crate::grid::Grid;

pub trait Game {
    fn new(size: i32) -> Self where Self: Sized;
    fn get_grid(&self) -> &Grid;
    fn get_grid_mut(&mut self) -> &mut Grid;
    fn make_move(&mut self, start: (i32, i32), dest: (i32, i32));
    fn is_game_over(&self) -> bool;
    fn randomize(&mut self);
    fn get_board_size(&self) -> i32;
    fn reset(&mut self);
}

pub struct ManualGame {
    pub grid: Grid,
    pub board_size: i32,
    pub selected_start: Option<(i32, i32)>,
    pub selected_end: Option<(i32, i32)>,
}

pub struct AutomatedGame {
    pub grid: Grid,
    pub board_size: i32,
}

impl Game for ManualGame {
    fn new(size: i32) -> Self {
        ManualGame {
            grid: Grid::new(size),
            board_size: size,
            selected_start: None,
            selected_end: None,
        }
    }

    fn get_grid(&self) -> &Grid { &self.grid }
    fn get_grid_mut(&mut self) -> &mut Grid { &mut self.grid }
    fn get_board_size(&self) -> i32 { self.board_size }

    fn make_move(&mut self, start: (i32, i32), dest: (i32, i32)) {
        self.grid.make_move(start, dest);
        self.selected_start = None;
        self.selected_end = None;
    }

    fn is_game_over(&self) -> bool {
        !self.grid.has_any_valid_move()
    }

    fn randomize(&mut self) {
        self.grid.randomize_pegs();
    }

    fn reset(&mut self) {
        self.grid = Grid::new(self.board_size);
        self.selected_start = None;
        self.selected_end = None;
    }
}

impl Game for AutomatedGame {
    fn new(size: i32) -> Self {
        AutomatedGame {
            grid: Grid::new(size),
            board_size: size,
        }
    }

    fn get_grid(&self) -> &Grid { &self.grid }
    fn get_grid_mut(&mut self) -> &mut Grid { &mut self.grid }
    fn get_board_size(&self) -> i32 { self.board_size }

    fn make_move(&mut self, start: (i32, i32), dest: (i32, i32)) {
        self.grid.make_move(start, dest);
    }

    fn is_game_over(&self) -> bool {
        !self.grid.has_any_valid_move()
    }

    fn randomize(&mut self) {
        self.grid.randomize_pegs();
    }

    fn reset(&mut self) {
        self.grid = Grid::new(self.board_size);
    }
}

impl AutomatedGame {
    pub fn make_auto_move(&mut self) -> bool {
        let valid_moves = self.grid.get_all_valid_moves();
        if let Some(&(start, dest)) = valid_moves.choose(&mut rand::thread_rng()) {
            self.grid.make_move(start, dest);
            true
        } else {
            false
        }
    }
}

pub enum GameMode {
    Manual(ManualGame),
    Automated(AutomatedGame),
}

pub struct GameManager {
    pub mode: GameMode,
}

impl GameManager {
    pub fn new_manual(size: i32) -> Self {
        GameManager { mode: GameMode::Manual(ManualGame::new(size)) }
    }

    pub fn new_automated(size: i32) -> Self {
        GameManager { mode: GameMode::Automated(AutomatedGame::new(size)) }
    }

    pub fn as_game(&self) -> &dyn Game {
        match &self.mode {
            GameMode::Manual(g) => g,
            GameMode::Automated(g) => g,
        }
    }

    pub fn as_game_mut(&mut self) -> &mut dyn Game {
        match &mut self.mode {
            GameMode::Manual(g) => g,
            GameMode::Automated(g) => g,
        }
    }

    pub fn board_size(&self) -> i32 {
        self.as_game().get_board_size()
    }

    pub fn set_board_size(&mut self, size: i32) {
        match &mut self.mode {
            GameMode::Manual(g) => g.board_size = size,
            GameMode::Automated(g) => g.board_size = size,
        }
    }
}