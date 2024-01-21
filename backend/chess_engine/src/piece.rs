// piece.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Colour::Black => write!(f, "Black"),
            Colour::White => write!(f, "White"),
        }
    }
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

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "Black"),
            PieceType::Knight => write!(f, "Knight"),
            PieceType::Bishop => write!(f, "Bishop"),
            PieceType::Rook => write!(f, "Rook"),
            PieceType::Queen => write!(f, "Queen"),
            PieceType::King => write!(f, "King"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    colour: Colour,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(colour: Colour, piece_type: PieceType) -> Self {
        Piece { colour, piece_type }
    }

    pub fn get_colour(&self) -> Colour {
        self.colour
    }

    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }
}
