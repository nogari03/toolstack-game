use crate::board::{BoardGrid, Color, Piece, PieceType};

pub struct GameState {
    pub board: BoardGrid,
    pub current_turn: Color,
}

impl GameState {
    pub fn new() -> Self {
        let mut board: BoardGrid = Default::default();

        let han = Color::Han;
        let cho = Color::Cho;

        // Cho starts first. Han is Red (top usually in UI but can be bottom, let's keep Han top for standard mapping).
        // Standard setup: Ma-Sang-Ma-Sang (Horse-Elephant-Horse-Elephant) typical arrangement but we'll use a basic Ma-Sang-Sang-Ma for now
        // Let's implement Cho (Green) bottom, Han (Red) top.

        // Row 0 (Han): Chariot, Horse, Elephant, Advisor, Empty, Advisor, Elephant, Horse, Chariot
        board[0][0] = Some(Piece::new(PieceType::Chariot, han));
        board[0][1] = Some(Piece::new(PieceType::Horse, han));
        board[0][2] = Some(Piece::new(PieceType::Elephant, han));
        board[0][3] = Some(Piece::new(PieceType::Advisor, han));
        board[0][5] = Some(Piece::new(PieceType::Advisor, han));
        board[0][6] = Some(Piece::new(PieceType::Elephant, han));
        board[0][7] = Some(Piece::new(PieceType::Horse, han));
        board[0][8] = Some(Piece::new(PieceType::Chariot, han));

        board[1][4] = Some(Piece::new(PieceType::General, han));

        board[2][1] = Some(Piece::new(PieceType::Cannon, han));
        board[2][7] = Some(Piece::new(PieceType::Cannon, han));

        board[3][0] = Some(Piece::new(PieceType::Soldier, han));
        board[3][2] = Some(Piece::new(PieceType::Soldier, han));
        board[3][4] = Some(Piece::new(PieceType::Soldier, han));
        board[3][6] = Some(Piece::new(PieceType::Soldier, han));
        board[3][8] = Some(Piece::new(PieceType::Soldier, han));

        // Row 9 (Cho):
        board[9][0] = Some(Piece::new(PieceType::Chariot, cho));
        board[9][1] = Some(Piece::new(PieceType::Horse, cho));
        board[9][2] = Some(Piece::new(PieceType::Elephant, cho));
        board[9][3] = Some(Piece::new(PieceType::Advisor, cho));
        board[9][5] = Some(Piece::new(PieceType::Advisor, cho));
        board[9][6] = Some(Piece::new(PieceType::Elephant, cho));
        board[9][7] = Some(Piece::new(PieceType::Horse, cho));
        board[9][8] = Some(Piece::new(PieceType::Chariot, cho));

        board[8][4] = Some(Piece::new(PieceType::General, cho));

        board[7][1] = Some(Piece::new(PieceType::Cannon, cho));
        board[7][7] = Some(Piece::new(PieceType::Cannon, cho));

        board[6][0] = Some(Piece::new(PieceType::Soldier, cho));
        board[6][2] = Some(Piece::new(PieceType::Soldier, cho));
        board[6][4] = Some(Piece::new(PieceType::Soldier, cho));
        board[6][6] = Some(Piece::new(PieceType::Soldier, cho));
        board[6][8] = Some(Piece::new(PieceType::Soldier, cho));

        GameState {
            board,
            current_turn: Color::Cho, // Cho goes first
        }
    }

    pub fn get_valid_moves(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        if let Some(piece) = self.board[row][col] {
            if piece.color != self.current_turn {
                return moves;
            }

            for r in 0..10 {
                for c in 0..9 {
                    if self.is_valid_move(row, col, r, c) {
                        if !self.would_be_in_check(row, col, r, c) {
                            moves.push((r, c));
                        }
                    }
                }
            }
        }
        moves
    }

    pub fn get_all_legal_moves(&self, color: Color) -> Vec<((usize, usize), (usize, usize))> {
        let mut all_moves = Vec::new();
        for r in 0..10 {
            for c in 0..9 {
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

    pub fn is_valid_move(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        self.is_pseudo_legal(fr, fc, tr, tc)
    }

    pub fn would_be_in_check(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        let mut temp_state = GameState {
            board: self.board,
            current_turn: self.board[fr][fc].unwrap().color,
        };
        temp_state.board[tr][tc] = temp_state.board[fr][fc];
        temp_state.board[fr][fc] = None;

        let my_color = temp_state.board[tr][tc].unwrap().color;
        let mut general_pos = None;
        for r in 0..10 {
            for c in 0..9 {
                if let Some(p) = temp_state.board[r][c] {
                    if p.piece_type == PieceType::General && p.color == my_color {
                        general_pos = Some((r, c));
                        break;
                    }
                }
            }
        }
        let (gr, gc) = match general_pos {
            Some(pos) => pos,
            None => return true, 
        };

        for r in 0..10 {
            for c in 0..9 {
                if let Some(p) = temp_state.board[r][c] {
                    if p.color != my_color {
                        if temp_state.is_pseudo_legal(r, c, gr, gc) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let mut general_pos = None;
        for r in 0..10 {
            for c in 0..9 {
                if let Some(p) = self.board[r][c] {
                    if p.piece_type == PieceType::General && p.color == color {
                        general_pos = Some((r, c));
                        break;
                    }
                }
            }
        }
        let (gr, gc) = match general_pos {
            Some(pos) => pos,
            None => return false,
        };

        for r in 0..10 {
            for c in 0..9 {
                if let Some(p) = self.board[r][c] {
                    if p.color != color {
                        if self.is_pseudo_legal(r, c, gr, gc) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn get_status(&self) -> String {
        let mut han_general = false;
        let mut cho_general = false;
        for r in 0..10 {
            for c in 0..9 {
                if let Some(p) = self.board[r][c] {
                    if p.piece_type == PieceType::General {
                        if p.color == Color::Han { han_general = true; }
                        if p.color == Color::Cho { cho_general = true; }
                    }
                }
            }
        }
        if !han_general { return "Han General Captured: Cho Wins".to_string(); }
        if !cho_general { return "Cho General Captured: Han Wins".to_string(); }

        let mut has_moves = false;
        'outer: for r in 0..10 {
            for c in 0..9 {
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
                let winner = if self.current_turn == Color::Han { "Cho" } else { "Han" };
                return format!("Checkmate: {} Wins", winner);
            } else {
                return "Stalemate: Draw".to_string();
            }
        }
        "Active".to_string()
    }

    pub fn is_pseudo_legal(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        if fr == tr && fc == tc { return false; }
        
        let piece = match self.board[fr][fc] {
            Some(p) => p,
            None => return false,
        };

        if let Some(target) = self.board[tr][tc] {
            if target.color == piece.color {
                return false;
            }
        }

        match piece.piece_type {
            PieceType::Chariot => self.is_valid_chariot(fr, fc, tr, tc),
            PieceType::Horse => self.is_valid_horse(fr, fc, tr, tc),
            PieceType::Elephant => self.is_valid_elephant(fr, fc, tr, tc),
            PieceType::Advisor | PieceType::General => self.is_valid_palace_piece(fr, fc, tr, tc, piece.color),
            PieceType::Cannon => self.is_valid_cannon(fr, fc, tr, tc),
            PieceType::Soldier => self.is_valid_soldier(fr, fc, tr, tc, piece.color),
        }
    }

    fn in_palace(&self, r: usize, c: usize) -> bool {
        (r <= 2 || r >= 7) && (c >= 3 && c <= 5)
    }

    // King & Guard: 1 space along lines in 3x3 palace
    fn is_valid_palace_piece(&self, fr: usize, fc: usize, tr: usize, tc: usize, color: Color) -> bool {
        if !self.in_palace(tr, tc) { return false; }
        
        // Ensure it doesn't leave its own palace hemisphere
        let is_top_palace = fr <= 2 && tr <= 2;
        let is_bot_palace = fr >= 7 && tr >= 7;
        if !is_top_palace && !is_bot_palace { return false; }

        let dr = (tr as isize - fr as isize).abs();
        let dc = (tc as isize - fc as isize).abs();

        if dr + dc == 1 { return true; } // orthogonal 1 step
        
        // Diagonal 1 step in palace
        if dr == 1 && dc == 1 {
            let center_r = if fr <= 2 { 1 } else { 8 };
            if (fr == center_r && fc == 4) || (tr == center_r && tc == 4) {
                return true;
            }
        }
        false
    }
    
    // Soldier: 1 space forward or sideways, diagonal forward in palace
    fn is_valid_soldier(&self, fr: usize, fc: usize, tr: usize, tc: usize, color: Color) -> bool {
        let dr = tr as isize - fr as isize;
        let dc = (tc as isize - fc as isize).abs();
        
        // Forward is +1 for Han (starts top), -1 for Cho (starts rank 9 bottom)
        let forward = if color == Color::Han { 1 } else { -1 };
        
        // Forward or sideways
        if (dr == forward && dc == 0) || (dr == 0 && dc == 1) { return true; }
        
        // Diagonal forward in enemy palace
        if self.in_palace(fr, fc) && self.in_palace(tr, tc) {
            let center_r = if color == Color::Han { 8 } else { 1 };
            if dr == forward && dc == 1 {
                if (fr == center_r && fc == 4) || (tr == center_r && tc == 4) {
                    return true;
                }
            }
        }
        false
    }

    // Chariot: straight line or diagonal in palace
    fn is_valid_chariot(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        let dr = (tr as isize - fr as isize).abs();
        let dc = (tc as isize - fc as isize).abs();

        if (dr == 0 && dc > 0) || (dr > 0 && dc == 0) {
            return self.is_path_clear(fr, fc, tr, tc);
        }

        // Diagonal in palace
        if self.in_palace(fr, fc) && self.in_palace(tr, tc) && dr == dc {
            let is_top_palace = fr <= 2 && tr <= 2;
            let is_bot_palace = fr >= 7 && tr >= 7;
            if !is_top_palace && !is_bot_palace { return false; }

            let center_r = if tr <= 2 { 1 } else { 8 };
            if (fr == center_r && fc == 4) || (tr == center_r && tc == 4) || (fr != center_r && tr != center_r) {
               return self.is_path_clear_diag(fr, fc, tr, tc);
            }
        }
        false
    }

    // Cannon: Jump exactly 1 piece. Cannot jump over cannon, cannot capture cannon. Can jump diagonally in palace.
    fn is_valid_cannon(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        if let Some(target) = self.board[tr][tc] {
            if target.piece_type == PieceType::Cannon { return false; }
        }

        let dr = (tr as isize - fr as isize).abs();
        let dc = (tc as isize - fc as isize).abs();

        if (dr == 0 && dc > 0) || (dr > 0 && dc == 0) {
            let dr_step = if tr > fr { 1 } else if tr < fr { -1 } else { 0 };
            let dc_step = if tc > fc { 1 } else if tc < fc { -1 } else { 0 };
            let mut r = fr as isize + dr_step;
            let mut c = fc as isize + dc_step;
            let mut screens = 0;
            while r != tr as isize || c != tc as isize {
                if let Some(p) = self.board[r as usize][c as usize] {
                    if p.piece_type == PieceType::Cannon { return false; } // Cannot use Cannon as screen
                    screens += 1;
                }
                r += dr_step;
                c += dc_step;
            }
            return screens == 1;
        }

        // Diagonal jump across palace center
        if self.in_palace(fr, fc) && self.in_palace(tr, tc) && dr == 2 && dc == 2 {
            let center_r = if tr <= 2 { 1 } else { 8 };
            if fr != center_r && tr != center_r { // corner to corner jump
                if let Some(p) = self.board[center_r][4] {
                    if p.piece_type != PieceType::Cannon {
                        return true;
                    }
                }
            }
        }
        false
    }

    // Horse (Ma): 1 step ortho, 1 step diag
    fn is_valid_horse(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        let dr = tr as isize - fr as isize;
        let dc = tc as isize - fc as isize;
        if dr.abs() == 2 && dc.abs() == 1 {
            let block_r = (fr as isize + dr.signum()) as usize;
            return self.board[block_r][fc].is_none();
        } else if dr.abs() == 1 && dc.abs() == 2 {
            let block_c = (fc as isize + dc.signum()) as usize;
            return self.board[fr][block_c].is_none();
        }
        false
    }

    // Elephant (Sang): 1 step ortho, 2 steps diag
    fn is_valid_elephant(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        let dr = tr as isize - fr as isize;
        let dc = tc as isize - fc as isize;
        if dr.abs() == 3 && dc.abs() == 2 {
            let block1_r = (fr as isize + dr.signum()) as usize;
            if self.board[block1_r][fc].is_some() { return false; }
            let block2_r = (fr as isize + dr.signum() * 2) as usize;
            let block2_c = (fc as isize + dc.signum()) as usize;
            return self.board[block2_r][block2_c].is_none();
        } else if dr.abs() == 2 && dc.abs() == 3 {
            let block1_c = (fc as isize + dc.signum()) as usize;
            if self.board[fr][block1_c].is_some() { return false; }
            let block2_r = (fr as isize + dr.signum()) as usize;
            let block2_c = (fc as isize + dc.signum() * 2) as usize;
            return self.board[block2_r][block2_c].is_none();
        }
        false
    }

    fn is_path_clear(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        let dr = if tr > fr { 1 } else if tr < fr { -1 } else { 0 };
        let dc = if tc > fc { 1 } else if tc < fc { -1 } else { 0 };
        let mut r = fr as isize + dr;
        let mut c = fc as isize + dc;
        while r != tr as isize || c != tc as isize {
            if self.board[r as usize][c as usize].is_some() { return false; }
            r += dr;
            c += dc;
        }
        true
    }
    
    fn is_path_clear_diag(&self, fr: usize, fc: usize, tr: usize, tc: usize) -> bool {
        let dr = (tr as isize - fr as isize).signum();
        let dc = (tc as isize - fc as isize).signum();
        let mut r = fr as isize + dr;
        let mut c = fc as isize + dc;
        while r != tr as isize && c != tc as isize {
            if self.board[r as usize][c as usize].is_some() { return false; }
            r += dr;
            c += dc;
        }
        true
    }

    pub fn move_piece(&mut self, fr: usize, fc: usize, tr: usize, tc: usize) -> Result<(), &'static str> {
        if !self.is_valid_move(fr, fc, tr, tc) {
            return Err("Invalid move");
        }
        if self.would_be_in_check(fr, fc, tr, tc) {
            return Err("Move would put or leave general in check");
        }

        let mut piece = self.board[fr][fc].unwrap();
        piece.has_moved = true;
        
        self.board[tr][tc] = Some(piece);
        self.board[fr][fc] = None;
        self.current_turn = self.current_turn.opposite();

        Ok(())
    }

    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        for r in 0..10 {
            for c in 0..9 {
                if let Some(p) = self.board[r][c] {
                    // Typical piece values in Janggi: Chariot(13), Cannon(7), Horse(5), Elephant(3), Advisor(3), Soldier(2), General(inf)
                    let val = match p.piece_type {
                        PieceType::Soldier => 20,
                        PieceType::Advisor => 30,
                        PieceType::Elephant => 30,
                        PieceType::Horse => 50,
                        PieceType::Cannon => 70,
                        PieceType::Chariot => 130,
                        PieceType::General => 9000,
                    };
                    if p.color == Color::Han {
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
    if status.contains("Cho Wins") { return -100000; }
    if status.contains("Han Wins") { return 100000; }
    if status.contains("Draw") { return 0; }
    if depth == 0 { return state.evaluate(); }

    if is_maximizing {
        let mut max_eval = i32::MIN;
        let moves = state.get_all_legal_moves(Color::Han);
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
        let moves = state.get_all_legal_moves(Color::Cho);
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

    if color == Color::Han {
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

impl Color {
    fn opposite(&self) -> Self {
        match self {
            Color::Han => Color::Cho,
            Color::Cho => Color::Han,
        }
    }
}
