// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let mut board: [bool; 49] = [
        false, false, true,  true,  true,  false, false,
        false, false, true,  true,  true,  false, false,
        true,  true,  true,  true,  true,  true,  true,
        true,  true,  true,  false, true,  true,  true,
        true,  true,  true,  true,  true,  true,  true,
        false, false, true,  true,  true,  false, false,
        false, false, true,  true,  true,  false, false,
    ];

    // Store selected cell
    let selected_start = std::cell::RefCell::new(None::<usize>);

    // ui.on_try_move(move |name| {
    //     let index: usize = name.strip_prefix("cell-").unwrap().parse().unwrap();

    //     if let Some(start_index) = selected_start.borrow_mut().take() {
    //         // Try to perform move from start_index -> index
    //         let (sr, sc) = (start_index / 7, start_index % 7);
    //         let (er, ec) = (index / 7, index % 7);

    //         if is_valid_move(sr, sc, er, ec, &board) {
    //             execute_move(sr, sc, er, ec, &mut board);
    //             app.set_board_state(board.to_vec());
    //         } else {
    //             println!("Invalid move from {} to {}", start_index, index);
    //         }
    //     } else {
    //         // First click
    //         *selected_start.borrow_mut() = Some(index);
    //         app.set_selected_start(name.into());
    //     }
    // });

    ui.run()?;

    Ok(())
}
