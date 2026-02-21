pub mod game;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::game::{GameState, GameStatus};

#[wasm_bindgen]
pub struct GameEngine {
    state: GameState,
}

#[derive(Serialize, Deserialize)]
pub struct MoveResponse {
    pub success: bool,
    pub message: String,
    pub moved: bool,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameEngine {
        GameEngine {
            state: GameState::new(),
        }
    }

    pub fn get_board_state(&self) -> String {
        serde_json::to_string(&self.state.board).unwrap()
    }

    pub fn get_score(&self) -> u32 {
        self.state.score
    }

    pub fn get_status(&self) -> String {
        match self.state.status {
            GameStatus::Active => "Active".to_string(),
            GameStatus::Won => "Won".to_string(),
            GameStatus::Lost => "Lost".to_string(),
        }
    }

    pub fn execute_move(&mut self, direction: &str) -> String {
        match self.state.move_board(direction) {
            Ok(moved) => {
                let res = MoveResponse { success: true, message: "Move executed".to_string(), moved };
                serde_json::to_string(&res).unwrap()
            },
            Err(e) => {
                let res = MoveResponse { success: false, message: e.to_string(), moved: false };
                serde_json::to_string(&res).unwrap()
            }
        }
    }
}
