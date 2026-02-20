pub mod board;
pub mod rules;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::rules::GameState;

#[wasm_bindgen]
pub struct GameEngine {
    state: GameState,
}

#[derive(Serialize, Deserialize)]
pub struct MoveResponse {
    pub success: bool,
    pub message: String,
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

    pub fn get_current_turn(&self) -> String {
        serde_json::to_string(&self.state.current_turn).unwrap()
    }

    pub fn get_status(&self) -> String {
        self.state.get_status()
    }

    pub fn get_valid_moves(&self, row: usize, col: usize) -> Vec<JsValue> {
        let moves = self.state.get_valid_moves(row, col);
        moves.into_iter()
            .map(|(r, c)| {
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"row".into(), &JsValue::from(r as u32)).unwrap();
                js_sys::Reflect::set(&obj, &"col".into(), &JsValue::from(c as u32)).unwrap();
                obj.into()
            })
            .collect()
    }

    pub fn move_piece(&mut self, fr: usize, fc: usize, tr: usize, tc: usize) -> String {
        match self.state.move_piece(fr, fc, tr, tc) {
            Ok(_) => {
                let res = MoveResponse { success: true, message: "Move successful".to_string() };
                serde_json::to_string(&res).unwrap()
            },
            Err(e) => {
                let res = MoveResponse { success: false, message: e.to_string() };
                serde_json::to_string(&res).unwrap()
            }
        }
    }
}
