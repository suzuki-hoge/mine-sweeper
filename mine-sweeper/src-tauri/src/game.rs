use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::game::Dot::{Eight, Five, Flag, Four, One, Seven, Six, Three, Two};

pub struct GameState {
    pub value: Mutex<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Dots {
    dots: Vec<Vec<Dot>>,
}

impl Dots {
    pub fn init(_w: usize, _h: usize) -> Self {
        Self {
            dots: vec![
                vec![Two, Eight, One],
                vec![Flag, Three, Four],
                vec![Five, Six, Seven],
            ],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Dot {
    Unexplored,
    Flag,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}
