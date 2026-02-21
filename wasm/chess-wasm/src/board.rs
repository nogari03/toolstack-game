use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub has_moved: bool, // For castling and double-step pawn moves
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece {
            piece_type,
            color,
            has_moved: false,
        }
    }
}

pub type Square = Option<Piece>;
pub type BoardGrid = [[Square; 8]; 8];
