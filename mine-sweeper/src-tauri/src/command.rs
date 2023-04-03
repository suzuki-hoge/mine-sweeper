use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::game::Density::{High, Low, Middle};
use crate::game::Swept::{Bomb, Clear, Safe, Stay};
use crate::game::{Game, Swept};

pub struct GameState {
    game: Mutex<Game>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            game: Mutex::new(Game::new()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DotsJson {
    dots: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct DotsAndSweptJson {
    swept: String,
    dots: Vec<Vec<String>>,
}

#[tauri::command]
pub fn init_game(state: State<'_, GameState>, w: usize, h: usize, density: String) -> DotsJson {
    let mut game = state.game.lock().unwrap();
    let density = match density.as_str() {
        "low" => Low,
        "middle" => Middle,
        "high" => High,
        _ => panic!(),
    };
    game.init(w, h, density);
    DotsJson { dots: game.show() }
}

#[tauri::command]
pub fn sweep(state: State<'_, GameState>, x: usize, y: usize) -> DotsAndSweptJson {
    let mut game = state.game.lock().unwrap();
    let swept = game.sweep(x, y);
    DotsAndSweptJson {
        swept: swept_label(swept),
        dots: game.show(),
    }
}

#[tauri::command]
pub fn flag(state: State<'_, GameState>, x: usize, y: usize) -> DotsAndSweptJson {
    let mut game = state.game.lock().unwrap();
    let swept = game.flag(x, y);
    DotsAndSweptJson {
        swept: swept_label(swept),
        dots: game.show(),
    }
}

fn swept_label(swept: Swept) -> String {
    match swept {
        Safe => "safe",
        Bomb => "bomb",
        Stay => "stay",
        Clear => "clear",
    }
    .to_string()
}
