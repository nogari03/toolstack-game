use crate::board::{BoardGrid, Color, Piece, PieceType};

pub struct GameState {
    pub board: BoardGrid,
    pub current_turn: Color,
    // Add history for En Passant and 50-move rules later
}

impl GameState {
    pub fn new() -> Self {
        let mut board: BoardGrid = Default::default();
        
        // Initialize pawns
        for col in 0..8 {
            board[1][col] = Some(Piece::new(PieceType::Pawn, Color::Black));
            board[6][col] = Some(Piece::new(PieceType::Pawn, Color::White));
        }

        // Initialize other pieces
        let piece_order = [
            PieceType::Rook, PieceType::Knight, PieceType::Bishop, PieceType::Queen,
            PieceType::King, PieceType::Bishop, PieceType::Knight, PieceType::Rook,
        ];

        for col in 0..8 {
            board[0][col] = Some(Piece::new(piece_order[col], Color::Black));
            board[7][col] = Some(Piece::new(piece_order[col], Color::White));
        }

        GameState {
            board,
            current_turn: Color::White,
        }
    }

    pub fn get_valid_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();
        if let Some(piece) = self.board[row][col] {
            if piece.color != self.current_turn { return vec![]; }
            
            // Iterate over the whole board and check pseudo-legality + avoid self-check
            for r in 0..8 {
                for c in 0..8 {
                    if self.is_valid_move(row, col, r, c) {
                        if !self.would_be_in_check(row, col, r, c) {
                            valid_moves.push((r, c));
                        }
                    }
                }
            }
        }
        valid_moves
    }

    pub fn get_all_legal_moves(&self, color: Color) -> Vec<((usize, usize), (usize, usize))> {
        let mut all_moves = Vec::new();
        for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = self.board[r][c] {
                    if p.color == color {
                        let moves = self.get_valid_moves(r, c);
                        for m in moves {
                            all_moves.push(((r, c), m));
                        }
                    }
                }
            }
        }
        all_moves
    }

    pub fn is_valid_move(&self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> bool {
        let piece = match self.board[from_row][from_col] {
            Some(p) => p,
            None => return false,
        };

        if piece.color != self.current_turn { return false; }

        let target = self.board[to_row][to_col];
        if let Some(t) = target {
            if t.color == piece.color { return false; } // Cannot capture own piece
        }

        let dr = to_row as isize - from_row as isize;
        let dc = to_col as isize - from_col as isize;

        if dr == 0 && dc == 0 { return false; }

        let is_pseudo_legal = match piece.piece_type {
            PieceType::Pawn => self.is_valid_pawn_move(piece.color, from_row, from_col, to_row, to_col, piece.has_moved),
            PieceType::Knight => (dr.abs() == 2 && dc.abs() == 1) || (dr.abs() == 1 && dc.abs() == 2),
            PieceType::Bishop => dr.abs() == dc.abs() && self.is_path_clear(from_row, from_col, to_row, to_col),
            PieceType::Rook => (dr == 0 || dc == 0) && self.is_path_clear(from_row, from_col, to_row, to_col),
            PieceType::Queen => (dr.abs() == dc.abs() || dr == 0 || dc == 0) && self.is_path_clear(from_row, from_col, to_row, to_col),
            PieceType::King => dr.abs() <= 1 && dc.abs() <= 1, // Ignore castling and checks for now
        };

        if !is_pseudo_legal { return false; }

        true
    }

    fn is_valid_pawn_move(&self, color: Color, fr: usize, fc: usize, tr: usize, tc: usize, has_moved: bool) -> bool {
        let dir = if color == Color::White { -1 } else { 1 };
        let dr = tr as isize - fr as isize;
        let dc = tc as isize - fc as isize;

        // Move forward
        if dc == 0 {
            if dr == dir && self.board[tr][tc].is_none() {
                return true;
            }
            if dr == 2 * dir && !has_moved && self.board[tr][tc].is_none() && self.board[(fr as isize + dir) as usize][tc].is_none() {
                return true;
            }
        } else if dc.abs() == 1 && dr == dir {
            // Capture
            if self.board[tr][tc].is_some() {
                return true;
            }
            // Add En Passant logic here later
        }

        false
    }

    fn is_path_clear(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        let dr = (tr as isize - fr as isize).signum();
        let dc = (tc as isize - fc as isize).signum();
        
        let mut r = fr as isize + dr;
        let mut c = fc as isize + dc;

        while r != tr as isize || c != tc as isize {
            if self.board[r as usize][c as usize].is_some() {
                return false;
            }
            r += dr;
            c += dc;
        }
        true
    }

    pub fn would_be_in_check(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        // Clone board to simulate move (for simplicity)
        let mut temp_board = self.board.clone();
        temp_board[tr][tc] = temp_board[fr][fc];
        temp_board[fr][fc] = None;

        let king_color = temp_board[tr][tc].unwrap().color;
        let mut king_pos = None;

        for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = temp_board[r][c] {
                    if p.piece_type == PieceType::King && p.color == king_color {
                        king_pos = Some((r, c));
                        break;
                    }
                }
            }
        }

        let (kr, kc) = king_pos.unwrap();

        // See if any enemy piece can attack king_pos
        for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = temp_board[r][c] {
                    if p.color != king_color {
                        let is_attack = match p.piece_type {
                            PieceType::Pawn => {
                                let dir = if p.color == Color::White { -1 } else { 1 };
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                dr == dir && dc.abs() == 1
                            },
                            PieceType::Knight => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                (dr.abs() == 2 && dc.abs() == 1) || (dr.abs() == 1 && dc.abs() == 2)
                            },
                            PieceType::Bishop => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                dr.abs() == dc.abs() && self.is_path_clear_on_board(&temp_board, r, c, kr, kc)
                            },
                            PieceType::Rook => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                (dr == 0 || dc == 0) && self.is_path_clear_on_board(&temp_board, r, c, kr, kc)
                            },
                            PieceType::Queen => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                (dr.abs() == dc.abs() || dr == 0 || dc == 0) && self.is_path_clear_on_board(&temp_board, r, c, kr, kc)
                            },
                            PieceType::King => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                dr.abs() <= 1 && dc.abs() <= 1
                            }
                        };
                        if is_attack { return true; }
                    }
                }
            }
        }
        false
    }

    fn is_path_clear_on_board(&self, board: &BoardGrid, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        let dr = (tr as isize - fr as isize).signum();
        let dc = (tc as isize - fc as isize).signum();
        
        let mut r = fr as isize + dr;
        let mut c = fc as isize + dc;

        while r != tr as isize || c != tc as isize {
            if board[r as usize][c as usize].is_some() {
                return false;
            }
            r += dr;
            c += dc;
        }
        true
    }

    pub fn get_status(&self) -> String {
        let mut white_king = false;
        let mut black_king = false;
        for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = self.board[r][c] {
                    if p.piece_type == PieceType::King {
                        if p.color == Color::White { white_king = true; }
                        if p.color == Color::Black { black_king = true; }
                    }
                }
            }
        }
        if !white_king { return "White King Captured: Black Wins".to_string(); }
        if !black_king { return "Black King Captured: White Wins".to_string(); }

        let mut has_moves = false;
        'outer: for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = self.board[r][c] {
                    if p.color == self.current_turn {
                        let moves = self.get_valid_moves(r, c);
                        if !moves.is_empty() {
                            has_moves = true;
                            break 'outer;
                        }
                    }
                }
            }
        }

        if !has_moves {
            if self.is_in_check(self.current_turn) {
                let winner = if self.current_turn == Color::White { "Black" } else { "White" };
                return format!("Checkmate: {} Wins", winner);
            } else {
                return "Stalemate: Draw".to_string();
            }
        }
        "Active".to_string()
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let mut king_pos = None;
        for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = self.board[r][c] {
                    if p.piece_type == PieceType::King && p.color == color {
                        king_pos = Some((r, c));
                        break;
                    }
                }
            }
        }
        let (kr, kc) = match king_pos {
            Some(pos) => pos,
            None => return false,
        };

        for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = self.board[r][c] {
                    if p.color != color {
                        let is_attack = match p.piece_type {
                            PieceType::Pawn => {
                                let dir = if p.color == Color::White { -1 } else { 1 };
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                dr == dir && dc.abs() == 1
                            },
                            PieceType::Knight => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                (dr.abs() == 2 && dc.abs() == 1) || (dr.abs() == 1 && dc.abs() == 2)
                            },
                            PieceType::Bishop => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                dr.abs() == dc.abs() && self.is_path_clear_on_board(&self.board, r, c, kr, kc)
                            },
                            PieceType::Rook => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                (dr == 0 || dc == 0) && self.is_path_clear_on_board(&self.board, r, c, kr, kc)
                            },
                            PieceType::Queen => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                (dr.abs() == dc.abs() || dr == 0 || dc == 0) && self.is_path_clear_on_board(&self.board, r, c, kr, kc)
                            },
                            PieceType::King => {
                                let dr = kr as isize - r as isize;
                                let dc = kc as isize - c as isize;
                                dr.abs() <= 1 && dc.abs() <= 1
                            }
                        };
                        if is_attack { return true; }
                    }
                }
            }
        }
        false
    }

    pub fn move_piece(&mut self, fr: usize, fc: usize, tr: usize, tc: usize) -> Result<(), &'static str> {
        if !self.is_valid_move(fr, fc, tr, tc) {
            return Err("Invalid move");
        }
        if self.would_be_in_check(fr, fc, tr, tc) {
            return Err("Move would put or leave king in check");
        }

        let mut piece = self.board[fr][fc].unwrap();
        piece.has_moved = true;
        
        self.board[tr][tc] = Some(piece);
        self.board[fr][fc] = None;
        
        self.current_turn = if self.current_turn == Color::White { Color::Black } else { Color::White };
        
        Ok(())
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        for r in 0..8 {
            for c in 0..8 {
                if let Some(p) = self.board[r][c] {
                    let val = match p.piece_type {
                        PieceType::Pawn => 10,
                        PieceType::Knight => 30,
                        PieceType::Bishop => 30,
                        PieceType::Rook => 50,
                        PieceType::Queen => 90,
                        PieceType::King => 900,
                    };
                    if p.color == Color::White {
                        score += val;
                    } else {
                        score -= val;
                    }
                }
            }
        }
        score
    }
}

pub fn minimax(state: &mut GameState, depth: u8, mut alpha: i32, mut beta: i32, is_maximizing: bool) -> i32 {
    let status = state.get_status();
    if status.contains("Black Wins") { return -10000; }
    if status.contains("White Wins") { return 10000; }
    if status.contains("Draw") { return 0; }
    if depth == 0 { return state.evaluate(); }

    if is_maximizing {
        let mut max_eval = i32::MIN;
        let moves = state.get_all_legal_moves(Color::White);
        for ((fr, fc), (tr, tc)) in moves {
            let mut next_state = GameState { board: state.board, current_turn: state.current_turn };
            if next_state.move_piece(fr, fc, tr, tc).is_ok() {
                let eval = minimax(&mut next_state, depth - 1, alpha, beta, false);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha { break; }
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        let moves = state.get_all_legal_moves(Color::Black);
        for ((fr, fc), (tr, tc)) in moves {
            let mut next_state = GameState { board: state.board, current_turn: state.current_turn };
            if next_state.move_piece(fr, fc, tr, tc).is_ok() {
                let eval = minimax(&mut next_state, depth - 1, alpha, beta, true);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha { break; }
            }
        }
        min_eval
    }
}

pub fn get_best_move(state: &GameState, depth: u8, color: Color) -> Option<((usize, usize), (usize, usize))> {
    let moves = state.get_all_legal_moves(color);
    if moves.is_empty() { return None; }

    let mut best_move = None;
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;

    if color == Color::White {
        let mut max_eval = i32::MIN;
        for ((fr, fc), (tr, tc)) in moves {
            let mut next_state = GameState { board: state.board, current_turn: state.current_turn };
            if next_state.move_piece(fr, fc, tr, tc).is_ok() {
                let eval = minimax(&mut next_state, depth - 1, alpha, beta, false);
                if eval > max_eval {
                    max_eval = eval;
                    best_move = Some(((fr, fc), (tr, tc)));
                }
                alpha = alpha.max(eval);
            }
        }
    } else {
        let mut min_eval = i32::MAX;
        for ((fr, fc), (tr, tc)) in moves {
            let mut next_state = GameState { board: state.board, current_turn: state.current_turn };
            if next_state.move_piece(fr, fc, tr, tc).is_ok() {
                let eval = minimax(&mut next_state, depth - 1, alpha, beta, true);
                if eval < min_eval {
                    min_eval = eval;
                    best_move = Some(((fr, fc), (tr, tc)));
                }
                beta = beta.min(eval);
            }
        }
    }

    best_move
}
