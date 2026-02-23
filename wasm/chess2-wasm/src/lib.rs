use wasm_bindgen::prelude::*;
use shakmaty::{Chess, Position, Board, Color as SColor, Role, Square as SSquare, Move, Setup};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PieceData {
    pub piece_type: String,
    pub color: String,
    pub has_moved: bool, // We might not need this exactly for shakmaty but it matches frontend.
}

#[derive(Serialize)]
pub struct MoveData {
    pub fr: usize,
    pub fc: usize,
    pub tr: usize,
    pub tc: usize,
}

#[derive(Serialize)]
pub struct MoveResult {
    pub success: bool,
    pub message: String,
}

#[wasm_bindgen]
pub struct GameEngine {
    game: Chess,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameEngine {
        GameEngine {
            game: Chess::default(),
        }
    }

    pub fn get_status(&self) -> String {
        if self.game.is_checkmate() {
            let turn = match self.game.turn() {
                SColor::White => "Black",
                SColor::Black => "White",
            };
            return format!("Checkmate! {} wins!", turn);
        }
        if self.game.is_stalemate() {
            return "Stalemate!".to_string();
        }
        if self.game.is_insufficient_material() {
            return "Draw by insufficient material!".to_string();
        }
        "Active".to_string()
    }

    pub fn get_current_turn(&self) -> String {
        match self.game.turn() {
            SColor::White => "\"White\"".to_string(),
            SColor::Black => "\"Black\"".to_string(),
        }
    }

    pub fn get_valid_moves(&self, row: usize, col: usize) -> Vec<JsValue> {
        let sq = Self::row_col_to_square(row, col);
        let moves = self.game.legal_moves();
        
        moves.into_iter()
            .filter(|m| m.from() == Some(sq))
            .map(|m| {
                let to = m.to();
                let (tr, tc) = Self::square_to_row_col(to);
                
                let obj = js_sys::Object::new();
                js_sys::Reflect::set(&obj, &"row".into(), &JsValue::from(tr as u32)).unwrap();
                js_sys::Reflect::set(&obj, &"col".into(), &JsValue::from(tc as u32)).unwrap();
                obj.into()
            })
            .collect()
    }

    pub fn move_piece(&mut self, fr: usize, fc: usize, tr: usize, tc: usize) -> String {
        let from_sq = Self::row_col_to_square(fr, fc);
        let to_sq = Self::row_col_to_square(tr, tc);
        
        // We just need to find a Move that matches from and to.
        // For promotions, shakmaty will generate multiple moves (Q, R, B, N). We pick Queen for simplicity.
        let moves = self.game.legal_moves();
        if let Some(valid_move) = moves.into_iter().find(|m| m.from() == Some(from_sq) && m.to() == to_sq && 
            (m.promotion().is_none() || m.promotion() == Some(Role::Queen))) 
        {
            let mut next_game = self.game.clone();
            next_game.play_unchecked(&valid_move);
            self.game = next_game;
            return serde_json::to_string(&MoveResult {
                success: true,
                message: "OK".to_string(),
            }).unwrap();
        }

        serde_json::to_string(&MoveResult {
            success: false,
            message: "Invalid move".to_string(),
        }).unwrap()
    }

    pub fn get_board_state(&self) -> String {
        let mut grid: Vec<Vec<Option<PieceData>>> = vec![vec![None; 8]; 8];
        let board = self.game.board();
        
        for row in 0..8 {
            for col in 0..8 {
                let sq = Self::row_col_to_square(row, col);
                if let Some(piece) = board.piece_at(sq) {
                    let color = match piece.color {
                        SColor::White => "White",
                        SColor::Black => "Black",
                    };
                    let ptype = match piece.role {
                        Role::Pawn => "Pawn",
                        Role::Knight => "Knight",
                        Role::Bishop => "Bishop",
                        Role::Rook => "Rook",
                        Role::Queen => "Queen",
                        Role::King => "King",
                    };
                    grid[row][col] = Some(PieceData {
                        piece_type: ptype.to_string(),
                        color: color.to_string(),
                        has_moved: false,
                    });
                }
            }
        }
        
        serde_json::to_string(&grid).unwrap()
    }

    pub fn get_best_move(&mut self, _depth: i32) -> String {
        let moves = self.game.legal_moves();
        if moves.is_empty() {
            return "null".to_string();
        }
        
        // Simple heuristic: Take pieces!
        let mut best_move = moves.first().unwrap().clone();
        for m in moves {
            if m.is_capture() {
                best_move = m;
                break;
            }
        }
        
        if let Some(from) = best_move.from() {
            let to = best_move.to();
            let (fr, fc) = Self::square_to_row_col(from);
            let (tr, tc) = Self::square_to_row_col(to);
            let md = MoveData { fr, fc, tr, tc };
            return serde_json::to_string(&md).unwrap();
        }
        
        "null".to_string()
    }

    fn row_col_to_square(row: usize, col: usize) -> SSquare {
        let file = col as u32;
        let rank = (7 - row) as u32;
        // The index for Square is 0..64. A1=0, B1=1, ... H8=63.
        // A1 is file 0, rank 0.
        // H8 is file 7, rank 7.
        // Our: `sq = rank * 8 + file` -> `new(rank * 8 + file)`
        SSquare::new(rank * 8 + file)
    }

    fn square_to_row_col(sq: SSquare) -> (usize, usize) {
        let file = (sq as u32) % 8;
        let rank = (sq as u32) / 8;
        let row = 7 - rank as usize;
        let col = file as usize;
        (row, col)
    }
}
