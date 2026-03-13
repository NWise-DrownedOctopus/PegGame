// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

mod grid;

use crate::grid::Grid;

slint::include_modules!();

struct GameState {
    grid: Grid,
    selected_start: Option<(i32, i32)>,
    selected_end: Option<(i32, i32)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let state = Rc::new(RefCell::new(GameState {
        grid: Grid::new(),
        selected_start: None,
        selected_end: None,
    }));

    // Handle hovered events
    let state_for_hover = state.clone();
    ui.on_peg_cell_hovered(move |x_pos, y_pos| {
        let state = state_for_hover.borrow();

        if let Some(cell) = state.grid.get_cell(x_pos, y_pos) {
           // println!("{:?}", cell);
        }
    });

    // Handle clicked events
    let state_for_click = state.clone();
    ui.on_peg_cell_clicked(move |x_pos, y_pos| {
        let mut state = state_for_click.borrow_mut();

        if let Some(cell) = state.grid.get_cell(x_pos, y_pos) {
            // println!("{:?}", cell);
            if state.selected_start.is_none() {
                state.selected_start = Some((x_pos, y_pos));
                println!("We selected a peg");
            }
            else {
                if state.selected_end.is_none() {
                    state.selected_end = Some((x_pos, y_pos));
                    println!("We selected a Move destination");
                }                
            }
        }        
    });

    ui.run()?;

    Ok(())
}
