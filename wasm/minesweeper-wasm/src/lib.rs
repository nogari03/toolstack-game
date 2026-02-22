pub mod board;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::board::{GameState, GameStatus};

#[wasm_bindgen]
pub struct GameEngine {
    state: GameState,
}

#[derive(Serialize, Deserialize)]
pub struct ActionResponse {
    pub success: bool,
    pub message: String,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameEngine {
        // Default to Beginner level: 9x9 with 10 mines
        GameEngine {
            state: GameState::new(9, 9, 10),
        }
    }

    pub fn new_beginner() -> GameEngine {
        GameEngine {
            state: GameState::new(9, 9, 10),
        }
    }

    pub fn new_intermediate() -> GameEngine {
        GameEngine {
            state: GameState::new(16, 16, 40),
        }
    }

    pub fn new_expert() -> GameEngine {
        GameEngine {
            state: GameState::new(16, 30, 99),
        }
    }

    pub fn get_board_state(&self) -> String {
        serde_json::to_string(&self.state.board).unwrap()
    }

    pub fn get_status(&self) -> String {
        match self.state.status {
            GameStatus::Active => "Active".to_string(),
            GameStatus::Won => "Won".to_string(),
            GameStatus::Lost => "Lost".to_string(),
        }
    }

    pub fn reveal(&mut self, row: usize, col: usize) -> String {
        match self.state.reveal(row, col) {
            Ok(_) => {
                let res = ActionResponse { success: true, message: "Reveal successful".to_string() };
                serde_json::to_string(&res).unwrap()
            },
            Err(e) => {
                let res = ActionResponse { success: false, message: e.to_string() };
                serde_json::to_string(&res).unwrap()
            }
        }
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) -> String {
        match self.state.toggle_flag(row, col) {
            Ok(_) => {
                let res = ActionResponse { success: true, message: "Flag toggled".to_string() };
                serde_json::to_string(&res).unwrap()
            },
            Err(e) => {
                let res = ActionResponse { success: false, message: e.to_string() };
                serde_json::to_string(&res).unwrap()
            }
        }
    }
}
