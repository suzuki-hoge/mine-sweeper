use tauri::State;

use crate::game::{Dots, GameState};

#[tauri::command]
pub fn foo(game: State<'_, GameState>, x: usize, y: usize) -> String {
    println!("{:?}", (game.value));
    let mut value = game.value.lock().unwrap();
    *value = "end".to_string();
    format!("foooo: {x}, {y}")
}

#[tauri::command]
pub fn init_dots(w: usize, h: usize) -> Dots {
    Dots::init(w, h)
}
