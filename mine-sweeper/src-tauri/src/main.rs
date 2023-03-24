use std::sync::Mutex;

use crate::game::GameState;
use tauri::Manager;

mod command;
mod game;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command::foo, command::init_dots])
        .setup(|app| {
            let game = GameState {
                value: Mutex::new(String::from("start")),
            };
            app.manage(game);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
