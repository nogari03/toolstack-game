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
        let turn = match self.state.current_turn {
            board::Color::Han => "White", // Map Han to White for UI compat
            board::Color::Cho => "Black", // Map Cho to Black for UI compat
        };
        serde_json::to_string(&turn).unwrap()
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

    pub fn get_best_move(&self, depth: u8) -> String {
        if let Some(((fr, fc), (tr, tc))) = crate::rules::get_best_move(&self.state, depth, self.state.current_turn) {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"fr".into(), &JsValue::from(fr as u32)).unwrap();
            js_sys::Reflect::set(&obj, &"fc".into(), &JsValue::from(fc as u32)).unwrap();
            js_sys::Reflect::set(&obj, &"tr".into(), &JsValue::from(tr as u32)).unwrap();
            js_sys::Reflect::set(&obj, &"tc".into(), &JsValue::from(tc as u32)).unwrap();
            return js_sys::JSON::stringify(&obj).unwrap().into();
        }
        "null".to_string()
    }
}
