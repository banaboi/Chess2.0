// piece.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

impl From<Colour> for usize {
    fn from(val: Colour) -> usize {
        match val {
            Colour::White => 0,
            Colour::Black => 1,
        }
    }
}

impl From<usize> for Colour {
    fn from(val: usize) -> Colour {
        match val {
            0 => Colour::White,
            1 => Colour::Black,
            _ => panic!("Invalid value for converting to Colour enum"),
        }
    }
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

impl From<PieceType> for usize {
    fn from(val: PieceType) -> usize {
        match val {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        }
    }
}

impl From<usize> for PieceType {
    fn from(val: usize) -> PieceType {
        match val {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => panic!("Invalid value for converting to PieceType enum"),
        }
    }
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "Pawn"),
            PieceType::Knight => write!(f, "Knight"),
            PieceType::Bishop => write!(f, "Bishop"),
            PieceType::Rook => write!(f, "Rook"),
            PieceType::Queen => write!(f, "Queen"),
            PieceType::King => write!(f, "King"),
        }
    }
}
