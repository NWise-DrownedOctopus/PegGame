// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cell::RefCell;
use slint::{VecModel, ModelRc, Model};
use std::error::Error;
use std::rc::Rc;

mod grid;
use crate::grid::Grid;

slint::include_modules!();

struct GameState {
    grid: Grid,
    selected_start: Option<(i32, i32)>,
    selected_end: Option<(i32, i32)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = Rc::new(RefCell::new(AppWindow::new()?));
    let state = Rc::new(RefCell::new(GameState {
        grid: Grid::new(),
        selected_start: None,
        selected_end: None,
    }));

    // Build initial model
    let cells: Vec<CellData> = state.borrow().grid.cells.iter().map(|c| CellData {
        x_pos: c.x,
        y_pos: c.y,
        has_peg: c.has_peg,
    }).collect();

    let model = ModelRc::new(VecModel::from(cells));
    ui.borrow().set_cells(model.clone());
    // let model_for_click = model.clone();
    let model_for_reset = model.clone();

    // Handle hovered events
    let state_for_hover = state.clone();
    ui.borrow().on_peg_cell_hovered(move |x_pos, y_pos| {
        let state = state_for_hover.borrow();

        if let Some(cell) = state.grid.get_cell(x_pos, y_pos) {
            // println!("{:?}", cell);
        }
    });

    // Handle clicked events
    let state_for_click = state.clone();
    let ui_for_hover = ui.clone();

    ui.borrow().on_peg_cell_clicked(move |x_pos, y_pos| {
        let mut state = state_for_click.borrow_mut();

        if let Some(cell) = state.grid.get_cell(x_pos, y_pos) {
            // println!("{:?}", cell);
            if state.selected_start.is_none() {
                state.selected_start = Some((x_pos, y_pos));
                println!("We selected a peg");
            } else {
                if state.selected_end.is_none() {
                    state.selected_end = Some((x_pos, y_pos));
                    println!("We selected a Move destination");

                    if let (Some((start_x, start_y)), Some((destination_x, destination_y))) =
                        (state.selected_start, state.selected_end)
                    {
                        if let (Some(start_cell), Some(destination_cell)) = (
                            state.grid.get_cell(start_x, start_y),
                            state.grid.get_cell(destination_x, destination_y),
                        ) {
                            let valid_move = state.grid.check_move(start_cell, destination_cell);

                            if !valid_move {
                                state.selected_start = None;
                                state.selected_end = None;
                            } else {
                                state.grid.make_move((start_x, start_y), (destination_x, destination_y));
                                update_ui(&model, &state.grid);
                                state.selected_start = None;
                                state.selected_end = None;

                                state.grid.make_move((start_x, start_y), (destination_x, destination_y));
                                update_ui(&model, &state.grid);

                                if !state.grid.has_any_valid_move() {
                                    println!("Game Over!");
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    let state_for_reset = state.clone();
    let ui_for_reset = ui.clone();

    ui.borrow().on_new_game_clicked(move || {
        let mut state = state_for_reset.borrow_mut();

        // Reset backend state
        state.grid = Grid::new();
        state.selected_start = None;
        state.selected_end = None;

        // Reset UI
        update_ui(&model_for_reset, &state.grid);

        // Reset displayed labels
        let ui = ui_for_reset.borrow();
        ui.set_hovered_cell("".into());
        ui.set_selected_cell("".into());
    });

    ui.borrow().run()?;

    Ok(())
}

fn update_ui(model: &ModelRc<CellData>, grid: &Grid) {
    for (i, cell) in grid.cells.iter().enumerate() {
        model.set_row_data(i, CellData {
            x_pos: cell.x,
            y_pos: cell.y,
            has_peg: cell.has_peg,
        });
    }
}


