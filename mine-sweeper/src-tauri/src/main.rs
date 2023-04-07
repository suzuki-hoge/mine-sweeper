extern crate core;

use tauri::Manager;

use crate::command::GameState;

mod command;
mod game;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::new,
            command::configure,
            command::sweep,
            command::flag
        ])
        .setup(|app| {
            let state = GameState::new();
            app.manage(state);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
