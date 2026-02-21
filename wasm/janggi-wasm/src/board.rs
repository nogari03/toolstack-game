use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Cho, // Blue/Green, usually goes first
    Han, // Red, usually goes second
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    General,   // King (궁/장)
    Advisor,   // Guard (사)
    Elephant,  // (상)
    Horse,     // (마)
    Chariot,   // Rook (차)
    Cannon,    // (포)
    Soldier,   // Pawn (졸/병)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub has_moved: bool,
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
// Janggi board is 10 rows by 9 columns.
// Row 0 is the top (Han's side usually), Row 9 is the bottom (Cho's side usually).
pub type BoardGrid = [[Square; 9]; 10];
