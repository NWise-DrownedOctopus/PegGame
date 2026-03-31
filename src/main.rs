#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cell::RefCell;
use slint::{VecModel, ModelRc, Model};
use std::error::Error;
use std::rc::Rc;

mod grid;
mod game;

use crate::game::{GameManager, GameMode, ManualGame, AutomatedGame, Game};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = Rc::new(RefCell::new(AppWindow::new()?));
    ui.borrow().set_board_size(7);

    let manager = Rc::new(RefCell::new(GameManager::new_manual(7)));

    let cells: Vec<CellData> = manager.borrow().as_game().get_grid().cells.iter().map(|c| CellData {
        x_pos: c.x,
        y_pos: c.y,
        has_peg: c.has_peg,
    }).collect();

    let model = Rc::new(RefCell::new(ModelRc::new(VecModel::from(cells))));
    ui.borrow().set_cells(model.borrow().clone());

    // Mode switching
    let manager_for_mode = manager.clone();
    let model_for_mode = model.clone();
    let ui_for_mode = ui.clone();

    ui.borrow().on_game_mode_changed(move |is_automated| {
        let mut mgr = manager_for_mode.borrow_mut();
        let size = mgr.board_size();
        *mgr = if is_automated {
            GameManager::new_automated(size)
        } else {
            GameManager::new_manual(size)
        };
        update_ui(&model_for_mode, mgr.as_game().get_grid());
        let ui = ui_for_mode.borrow();
        ui.set_board_size(size);
        ui.set_is_automated(is_automated); // <-- add this line
    });

    // Board size
    let manager_for_size = manager.clone();
    let model_for_size = model.clone();
    let ui_for_size = ui.clone();

    ui.borrow().on_board_size_changed(move |new_size| {
        let mut mgr = manager_for_size.borrow_mut();
        mgr.set_board_size(new_size);
        mgr.as_game_mut().reset();

        let new_cells: Vec<CellData> = mgr.as_game().get_grid().cells.iter().map(|c| CellData {
            x_pos: c.x,
            y_pos: c.y,
            has_peg: c.has_peg,
        }).collect();

        let new_model = ModelRc::new(VecModel::from(new_cells));
        *model_for_size.borrow_mut() = new_model.clone();

        let ui = ui_for_size.borrow();
        ui.set_cells(new_model);
        ui.set_board_size(new_size);
        ui.set_hovered_cell("".into());
        ui.set_selected_cell("".into());
    });

    // Manual cell clicks
    let manager_for_click = manager.clone();
    let model_for_click = model.clone();

    ui.borrow().on_peg_cell_clicked(move |x_pos, y_pos| {
        let mut mgr = manager_for_click.borrow_mut();

        if let GameMode::Manual(ref mut game) = mgr.mode {
            if let Some(cell) = game.grid.get_cell(x_pos, y_pos) {
                if game.selected_start.is_none() {
                    if cell.has_peg {
                        game.selected_start = Some((x_pos, y_pos));
                    }
                } else if game.selected_end.is_none() {
                    game.selected_end = Some((x_pos, y_pos));

                    if let (Some(start), Some(dest)) = (game.selected_start, game.selected_end) {
                        if let (Some(start_cell), Some(dest_cell)) = (
                            game.grid.get_cell(start.0, start.1),
                            game.grid.get_cell(dest.0, dest.1),
                        ) {
                            if game.grid.check_move(start_cell, dest_cell) {
                                game.make_move(start, dest);
                                update_ui(&model_for_click, &game.grid);

                                if game.is_game_over() {
                                    println!("Game Over!");
                                }
                            } else {
                                game.selected_start = None;
                                game.selected_end = None;
                            }
                        }
                    }
                }
            }
        }
    });

    // Auto move
    let manager_for_auto = manager.clone();
    let model_for_auto = model.clone();

    ui.borrow().on_auto_move_clicked(move || {
        let mut mgr = manager_for_auto.borrow_mut();

        if let GameMode::Automated(ref mut game) = mgr.mode {
            if game.make_auto_move() {
                update_ui(&model_for_auto, &game.grid);
                if game.is_game_over() {
                    println!("Game Over!");
                }
            } else {
                println!("No valid moves available.");
            }
        }
    });

    // Randomize
    let manager_for_rand = manager.clone();
    let model_for_rand = model.clone();

    ui.borrow().on_randomize_clicked(move || {
        let mut mgr = manager_for_rand.borrow_mut();
        mgr.as_game_mut().randomize();
        update_ui(&model_for_rand, mgr.as_game().get_grid());
    });

    // New game
    let manager_for_reset = manager.clone();
    let model_for_reset = model.clone();
    let ui_for_reset = ui.clone();

    ui.borrow().on_new_game_clicked(move || {
        let mut mgr = manager_for_reset.borrow_mut();
        mgr.as_game_mut().reset();
        update_ui(&model_for_reset, mgr.as_game().get_grid());

        let ui = ui_for_reset.borrow();
        ui.set_hovered_cell("".into());
        ui.set_selected_cell("".into());
        ui.set_board_size(mgr.board_size());
    });

    // Hover (unchanged in behavior)
    let manager_for_hover = manager.clone();

    ui.borrow().on_peg_cell_hovered(move |x_pos, y_pos| {
        let mgr = manager_for_hover.borrow();
        let _ = mgr.as_game().get_grid().get_cell(x_pos, y_pos);
    });

    ui.borrow().run()?;
    Ok(())
}

fn update_ui(model: &Rc<RefCell<ModelRc<CellData>>>, grid: &crate::grid::Grid) {
    let model_ref = model.borrow();
    for (i, cell) in grid.cells.iter().enumerate() {
        model_ref.set_row_data(i, CellData {
            x_pos: cell.x,
            y_pos: cell.y,
            has_peg: cell.has_peg,
        });
    }
}