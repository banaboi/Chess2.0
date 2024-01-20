// piece.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: Colour,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Colour, piece_type: PieceType) -> Self {
        Piece { color, piece_type }
    }

    pub fn get_color(&self) -> Colour {
        self.color
    }

    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }
}
