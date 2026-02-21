use serde::{Deserialize, Serialize};
use js_sys::Math;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Cell {
    pub is_mine: bool,
    pub adjacent_mines: u8,
    pub state: CellState,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum GameStatus {
    Active,
    Won,
    Lost,
}

pub struct GameState {
    pub rows: usize,
    pub cols: usize,
    pub total_mines: usize,
    pub board: Vec<Vec<Cell>>,
    pub status: GameStatus,
    pub first_click: bool,
}

impl GameState {
    pub fn new(rows: usize, cols: usize, total_mines: usize) -> Self {
        let board = vec![vec![Cell {
            is_mine: false,
            adjacent_mines: 0,
            state: CellState::Hidden,
        }; cols]; rows];
        
        Self {
            rows,
            cols,
            total_mines,
            board,
            status: GameStatus::Active,
            first_click: true,
        }
    }

    pub fn place_mines(&mut self, first_row: usize, first_col: usize) {
        let mut mines_placed = 0;
        let mut attempts = 0;
        while mines_placed < self.total_mines && attempts < 1000 {
            attempts += 1;
            let r = (Math::random() * self.rows as f64).floor() as usize;
            let c = (Math::random() * self.cols as f64).floor() as usize;
            
            if r >= self.rows || c >= self.cols {
                continue;
            }
            
            if (r == first_row && c == first_col) || self.board[r][c].is_mine {
                continue;
            }
            
            self.board[r][c].is_mine = true;
            mines_placed += 1;
        }
        
        self.calculate_adjacent_mines();
    }

    fn calculate_adjacent_mines(&mut self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.board[r][c].is_mine {
                    continue;
                }
                let mut count = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 { continue; }
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                            if self.board[nr as usize][nc as usize].is_mine {
                                count += 1;
                            }
                        }
                    }
                }
                self.board[r][c].adjacent_mines = count;
            }
        }
    }

    pub fn reveal(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if self.status != GameStatus::Active {
            return Err("Game is over");
        }
        if row >= self.rows || col >= self.cols {
            return Err("Invalid coordinates");
        }
        
        let cell = &self.board[row][col];
        if cell.state == CellState::Revealed || cell.state == CellState::Flagged {
            return Ok(());
        }

        if self.first_click {
            self.first_click = false;
            self.place_mines(row, col);
        }

        if self.board[row][col].is_mine {
            self.board[row][col].state = CellState::Revealed;
            self.status = GameStatus::Lost;
            return Ok(());
        }

        self.flood_fill(row, col);
        self.check_win_condition();
        
        Ok(())
    }

    fn flood_fill(&mut self, row: usize, col: usize) {
        if self.board[row][col].state != CellState::Hidden {
            return;
        }
        
        self.board[row][col].state = CellState::Revealed;
        
        if self.board[row][col].adjacent_mines == 0 {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 { continue; }
                    let nr = row as i32 + dr;
                    let nc = col as i32 + dc;
                    if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                        self.flood_fill(nr as usize, nc as usize);
                    }
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if self.status != GameStatus::Active {
            return Err("Game is over");
        }
        if row >= self.rows || col >= self.cols {
            return Err("Invalid coordinates");
        }
        
        match self.board[row][col].state {
            CellState::Hidden => {
                self.board[row][col].state = CellState::Flagged;
            }
            CellState::Flagged => {
                self.board[row][col].state = CellState::Hidden;
            }
            CellState::Revealed => {}
        }
        
        Ok(())
    }

    fn check_win_condition(&mut self) {
        let mut revealed_count = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.board[r][c].state == CellState::Revealed {
                    revealed_count += 1;
                }
            }
        }
        
        if revealed_count == (self.rows * self.cols) - self.total_mines {
            self.status = GameStatus::Won;
        }
    }
}
