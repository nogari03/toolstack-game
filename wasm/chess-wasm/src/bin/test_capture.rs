use chess_wasm::rules::GameState;

fn main() {
    let mut state = GameState::new();
    
    // Default board has white pawn at 6,0. Move it to 4,0.
    state.move_piece(6, 0, 4, 0).unwrap();
    // Black pawn at 1,0. Move to 3,0.
    state.move_piece(1, 0, 3, 0).unwrap();
    // White pawn capturing? Not yet. Move 4,0 to 3,1? It's not a valid move yet because no black piece.
    
    // Instead, let's print ALL valid moves for a newly created board's White Pawn at 6,0.
    println!("Valid moves for pawn at 6,0: {:?}", state.get_valid_moves(4, 0));
    
    // Now let's try moving a Knight to a square occupied by Black. No, let's try capturing directly.
    let mut state2 = GameState::new();
    // Clear the board for a clean test
    state2.board = Default::default();
    
    use chess_wasm::board::{Piece, PieceType, Color};
    // Put White King at 7,4
    state2.board[7][4] = Some(Piece::new(PieceType::King, Color::White));
    // Put Black King at 0,4
    state2.board[0][4] = Some(Piece::new(PieceType::King, Color::Black));
    
    // Put White Rook at 4,4
    state2.board[4][4] = Some(Piece::new(PieceType::Rook, Color::White));
    // Put Black Piece at 4,7
    state2.board[4][7] = Some(Piece::new(PieceType::Pawn, Color::Black));
    state2.current_turn = Color::White;
    
    println!("Valid moves for White Rook at 4,4: {:?}", state2.get_valid_moves(4, 4));
    
    // Check if it can capture the Black Pawn at 4,7.
    let moves = state2.get_valid_moves(4, 4);
    println!("Can Rook capture pawn at 4,7? {}", moves.contains(&(4, 7)));
    
    // What if we try to move King to a square attacked by Black?
    // Black Rook at 1,5
    state2.board[1][5] = Some(Piece::new(PieceType::Rook, Color::Black));
    // Valid moves for White King at 7,4?
    println!("Valid moves for White King at 7,4: {:?}", state2.get_valid_moves(7, 4));
}

