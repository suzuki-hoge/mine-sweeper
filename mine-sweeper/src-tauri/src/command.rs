// use tauri::State;

// use crate::game::Dot;

#[tauri::command]
// pub fn foo(game: State<'_, GameState>, x: usize, y: usize) -> String {
pub fn foo() -> String {
    // let mut value = game.value.lock().unwrap();
    // *value = "end".to_string();
    panic!()
}

#[tauri::command]
pub fn init_dots() -> Vec<Vec<u64>> {
    // Dots::init(w, h)
    panic!()
}
