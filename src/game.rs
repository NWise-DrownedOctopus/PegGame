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

// --- Recording ---

pub struct MoveRecord {
    pub move_number: u32,
    pub start: (i32, i32),
    pub end: (i32, i32),
}

pub struct GameRecorder {
    pub enabled: bool,
    pub moves: Vec<MoveRecord>,
}

impl GameRecorder {
    pub fn new() -> Self {
        GameRecorder { enabled: false, moves: Vec::new() }
    }

    pub fn record(&mut self, start: (i32, i32), end: (i32, i32)) {
        if self.enabled {
            let move_number = self.moves.len() as u32 + 1;
            self.moves.push(MoveRecord { move_number, start, end });
        }
    }

    pub fn clear(&mut self) {
        self.moves.clear();
    }

    pub fn export_to_string(&self) -> String {
        self.moves.iter()
            .map(|m| format!(
                "({}, ({}, {}), ({}, {}))",
                m.move_number, m.start.0, m.start.1, m.end.0, m.end.1
            ))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// --- Replay ---

#[derive(Clone)]
pub struct ReplayMove {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

/// Parse a recording file into a list of ReplayMoves.
/// Each line must match the format: (N, (x, y), (x, y))
/// Returns an error string if the file is missing, empty, or any line is malformed.
pub fn parse_recording(path: &std::path::PathBuf) -> Result<Vec<ReplayMove>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Could not read file: {}", e))?;

    if content.trim().is_empty() {
        return Err("Recording file is empty.".to_string());
    }

    let mut moves = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match parse_move_line(line) {
            Some(m) => moves.push(m),
            None => return Err(format!(
                "Invalid format on line {}: \"{}\"", i + 1, line
            )),
        }
    }

    if moves.is_empty() {
        return Err("Recording file contains no valid moves.".to_string());
    }

    Ok(moves)
}

/// Parse a single line of the format: (N, (sx, sy), (ex, ey))
fn parse_move_line(line: &str) -> Option<ReplayMove> {
    let line = line.strip_prefix('(')?.strip_suffix(')')?;

    // Split into 3 parts on ", (" — gives ["N", "sx, sy)", "ex, ey)"]
    let mut parts = line.splitn(3, ", (");
    let _move_num = parts.next()?.trim().parse::<u32>().ok()?;

    let start_str = parts.next()?.strip_suffix(')')?;
    let end_str   = parts.next()?.strip_suffix(')')?;

    let start = parse_coord(start_str)?;
    let end   = parse_coord(end_str)?;

    Some(ReplayMove { start, end })
}

fn parse_coord(s: &str) -> Option<(i32, i32)> {
    let mut parts = s.splitn(2, ',');
    let x = parts.next()?.trim().parse::<i32>().ok()?;
    let y = parts.next()?.trim().parse::<i32>().ok()?;
    Some((x, y))
}

// --- Games ---

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

/// Holds a fresh board and the parsed move list for replay.
/// Moves are applied externally by the replay thread in main.rs.
pub struct ReplayGame {
    pub grid: Grid,
    pub board_size: i32,
    pub moves: Vec<ReplayMove>,
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

    fn is_game_over(&self) -> bool { !self.grid.has_any_valid_move() }
    fn randomize(&mut self) { self.grid.randomize_pegs(); }

    fn reset(&mut self) {
        self.grid = Grid::new(self.board_size);
        self.selected_start = None;
        self.selected_end = None;
    }
}

impl Game for AutomatedGame {
    fn new(size: i32) -> Self {
        AutomatedGame { grid: Grid::new(size), board_size: size }
    }

    fn get_grid(&self) -> &Grid { &self.grid }
    fn get_grid_mut(&mut self) -> &mut Grid { &mut self.grid }
    fn get_board_size(&self) -> i32 { self.board_size }
    fn make_move(&mut self, start: (i32, i32), dest: (i32, i32)) { self.grid.make_move(start, dest); }
    fn is_game_over(&self) -> bool { !self.grid.has_any_valid_move() }
    fn randomize(&mut self) { self.grid.randomize_pegs(); }
    fn reset(&mut self) { self.grid = Grid::new(self.board_size); }
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

impl Game for ReplayGame {
    fn new(size: i32) -> Self {
        ReplayGame { grid: Grid::new(size), board_size: size, moves: Vec::new() }
    }

    fn get_grid(&self) -> &Grid { &self.grid }
    fn get_grid_mut(&mut self) -> &mut Grid { &mut self.grid }
    fn get_board_size(&self) -> i32 { self.board_size }
    fn make_move(&mut self, start: (i32, i32), dest: (i32, i32)) { self.grid.make_move(start, dest); }
    fn is_game_over(&self) -> bool { !self.grid.has_any_valid_move() }
    fn randomize(&mut self) {} // no-op during replay
    fn reset(&mut self) { self.grid = Grid::new(self.board_size); }
}

// --- Game Manager ---

pub enum GameMode {
    Manual(ManualGame),
    Automated(AutomatedGame),
    Replay(ReplayGame),
}

pub struct GameManager {
    pub mode: GameMode,
    pub recorder: GameRecorder,
}

impl GameManager {
    pub fn new_manual(size: i32) -> Self {
        GameManager {
            mode: GameMode::Manual(ManualGame::new(size)),
            recorder: GameRecorder::new(),
        }
    }

    pub fn new_automated(size: i32) -> Self {
        GameManager {
            mode: GameMode::Automated(AutomatedGame::new(size)),
            recorder: GameRecorder::new(),
        }
    }

    pub fn new_replay(size: i32, moves: Vec<ReplayMove>) -> Self {
        GameManager {
            mode: GameMode::Replay(ReplayGame {
                grid: Grid::new(size),
                board_size: size,
                moves,
            }),
            recorder: GameRecorder::new(),
        }
    }

    pub fn as_game(&self) -> &dyn Game {
        match &self.mode {
            GameMode::Manual(g) => g,
            GameMode::Automated(g) => g,
            GameMode::Replay(g) => g,
        }
    }

    pub fn as_game_mut(&mut self) -> &mut dyn Game {
        match &mut self.mode {
            GameMode::Manual(g) => g,
            GameMode::Automated(g) => g,
            GameMode::Replay(g) => g,
        }
    }

    pub fn board_size(&self) -> i32 {
        self.as_game().get_board_size()
    }

    pub fn set_board_size(&mut self, size: i32) {
        match &mut self.mode {
            GameMode::Manual(g) => g.board_size = size,
            GameMode::Automated(g) => g.board_size = size,
            GameMode::Replay(g) => g.board_size = size,
        }
    }

    pub fn is_replaying(&self) -> bool {
        matches!(self.mode, GameMode::Replay(_))
    }

    /// Make a move and record it if recording is enabled.
    pub fn make_move(&mut self, start: (i32, i32), dest: (i32, i32)) {
        self.as_game_mut().make_move(start, dest);
        self.recorder.record(start, dest);
    }

    /// Make an auto move (automated mode only) and record it if enabled.
    pub fn make_auto_move(&mut self) -> bool {
        if let GameMode::Automated(ref mut game) = self.mode {
            let valid_moves = game.grid.get_all_valid_moves();
            if let Some(&(start, dest)) = valid_moves.choose(&mut rand::thread_rng()) {
                game.grid.make_move(start, dest);
                self.recorder.record(start, dest);
                return true;
            }
        }
        false
    }

    /// Reset the game and clear the move history.
    pub fn reset(&mut self) {
        self.as_game_mut().reset();
        self.recorder.clear();
    }

    /// Save the recorded moves to a text file.
    pub fn save_recording(&self, path: &std::path::PathBuf) -> std::io::Result<()> {
        std::fs::write(path, self.recorder.export_to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_creates_fresh_game_state() {
        let grid_size = 7;
        let mut state = ManualGame {
            grid: Grid::new(grid_size),
            board_size: grid_size,
            selected_start: Some((3, 1)),
            selected_end: Some((3, 3)),
        };

        state.grid.make_move((3, 1), (3, 3));
        state.grid = Grid::new(grid_size);
        state.selected_start = None;
        state.selected_end = None;

        assert!(state.selected_start.is_none());
        assert!(state.selected_end.is_none());

        let center = state.grid.get_cell(3, 3).unwrap();
        assert!(!center.has_peg);

        let peg_count = state.grid.cells.iter().filter(|c| c.has_peg).count();
        assert_eq!(peg_count, 32);
    }

    #[test]
    fn parse_move_line_valid() {
        let m = parse_move_line("(1, (1, 3), (3, 3))").unwrap();
        assert_eq!(m.start, (1, 3));
        assert_eq!(m.end, (3, 3));
    }

    #[test]
    fn parse_move_line_invalid() {
        assert!(parse_move_line("not a move").is_none());
        assert!(parse_move_line("(1, (a, b), (3, 3))").is_none());
        assert!(parse_move_line("").is_none());
    }
}