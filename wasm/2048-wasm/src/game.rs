use serde::{Deserialize, Serialize};
use js_sys::Math;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameStatus {
    Active,
    Won,
    Lost,
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub board: [[u32; 4]; 4],
    pub score: u32,
    pub status: GameStatus,
    pub target_score: u32,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            board: [[0; 4]; 4],
            score: 0,
            status: GameStatus::Active,
            target_score: 2048,
        };
        state.spawn_tile();
        state.spawn_tile();
        state
    }

    fn spawn_tile(&mut self) -> bool {
        let mut empty_cells = Vec::new();
        for r in 0..4 {
            for c in 0..4 {
                if self.board[r][c] == 0 {
                    empty_cells.push((r, c));
                }
            }
        }

        if empty_cells.is_empty() {
            return false;
        }

        let idx = (Math::random() * empty_cells.len() as f64).floor() as usize;
        let val = if Math::random() < 0.9 { 2 } else { 4 };
        let (r, c) = empty_cells[idx];
        self.board[r][c] = val;
        true
    }

    pub fn move_board(&mut self, direction: &str) -> Result<bool, &'static str> {
        if self.status != GameStatus::Active {
            return Err("Game is over");
        }

        let moved = match direction {
            "Up" => self.move_up(),
            "Down" => self.move_down(),
            "Left" => self.move_left(),
            "Right" => self.move_right(),
            _ => return Err("Invalid direction"),
        };

        if moved {
            self.spawn_tile();
            self.check_game_over();
        }

        Ok(moved)
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for r in 0..4 {
            let mut new_row = [0; 4];
            let mut idx = 0;
            let mut last_merged = false;

            for c in 0..4 {
                if self.board[r][c] != 0 {
                    if idx > 0 && new_row[idx - 1] == self.board[r][c] && !last_merged {
                        new_row[idx - 1] *= 2;
                        self.score += new_row[idx - 1];
                        last_merged = true;
                        moved = true;
                        if new_row[idx - 1] >= self.target_score {
                            self.status = GameStatus::Won;
                        }
                    } else {
                        new_row[idx] = self.board[r][c];
                        if c != idx {
                            moved = true;
                        }
                        idx += 1;
                        last_merged = false;
                    }
                }
            }
            self.board[r] = new_row;
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        let mut moved = false;
        for r in 0..4 {
            let mut new_row = [0; 4];
            let mut idx = 3;
            let mut last_merged = false;

            for c in (0..4).rev() {
                if self.board[r][c] != 0 {
                    if idx < 3 && new_row[idx + 1] == self.board[r][c] && !last_merged {
                        new_row[idx + 1] *= 2;
                        self.score += new_row[idx + 1];
                        last_merged = true;
                        moved = true;
                        if new_row[idx + 1] >= self.target_score {
                            self.status = GameStatus::Won;
                        }
                    } else {
                        new_row[idx] = self.board[r][c];
                        if c != idx {
                            moved = true;
                        }
                        if idx > 0 { idx -= 1; }
                        last_merged = false;
                    }
                }
            }
            self.board[r] = new_row;
        }
        moved
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;
        for c in 0..4 {
            let mut new_col = [0; 4];
            let mut idx = 0;
            let mut last_merged = false;

            for r in 0..4 {
                if self.board[r][c] != 0 {
                    if idx > 0 && new_col[idx - 1] == self.board[r][c] && !last_merged {
                        new_col[idx - 1] *= 2;
                        self.score += new_col[idx - 1];
                        last_merged = true;
                        moved = true;
                        if new_col[idx - 1] >= self.target_score {
                            self.status = GameStatus::Won;
                        }
                    } else {
                        new_col[idx] = self.board[r][c];
                        if r != idx {
                            moved = true;
                        }
                        idx += 1;
                        last_merged = false;
                    }
                }
            }
            for r in 0..4 {
                self.board[r][c] = new_col[r];
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;
        for c in 0..4 {
            let mut new_col = [0; 4];
            let mut idx = 3;
            let mut last_merged = false;

            for r in (0..4).rev() {
                if self.board[r][c] != 0 {
                    if idx < 3 && new_col[idx + 1] == self.board[r][c] && !last_merged {
                        new_col[idx + 1] *= 2;
                        self.score += new_col[idx + 1];
                        last_merged = true;
                        moved = true;
                        if new_col[idx + 1] >= self.target_score {
                            self.status = GameStatus::Won;
                        }
                    } else {
                        new_col[idx] = self.board[r][c];
                        if r != idx {
                            moved = true;
                        }
                        if idx > 0 { idx -= 1; }
                        last_merged = false;
                    }
                }
            }
            for r in 0..4 {
                self.board[r][c] = new_col[r];
            }
        }
        moved
    }

    fn check_game_over(&mut self) {
        if self.status != GameStatus::Active {
            return;
        }

        // Check for empty cells
        for r in 0..4 {
            for c in 0..4 {
                if self.board[r][c] == 0 {
                    return; // Game can continue
                }
            }
        }

        // Check for valid adjacent merges
        for r in 0..4 {
            for c in 0..4 {
                let val = self.board[r][c];
                if r < 3 && self.board[r + 1][c] == val {
                    return;
                }
                if c < 3 && self.board[r][c + 1] == val {
                    return;
                }
            }
        }

        self.status = GameStatus::Lost;
    }
}
