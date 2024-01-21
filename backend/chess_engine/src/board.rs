use crate::piece::{Colour, Piece, PieceType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub piece: Option<Piece>,
}

impl Square {
    pub fn new() -> Self {
        Square { piece: None }
    }
    pub fn place_piece(&mut self, colour: Colour, piece_type: PieceType) {
        self.piece = Some(Piece::new(colour, piece_type));
    }

    pub fn display(&self) -> String {
        if self.piece.is_some() {
            "i am a square with a piece".to_string()
        } else {
            "i am a square with no piece".to_string()
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Board {
    squares: Vec<Square>,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = vec![Square::new(); 64];

        // Black pieces
        squares[0].piece = Some(Piece::new(Colour::Black, PieceType::Rook));
        squares[1].piece = Some(Piece::new(Colour::Black, PieceType::Knight));
        squares[2].piece = Some(Piece::new(Colour::Black, PieceType::Bishop));
        squares[3].piece = Some(Piece::new(Colour::Black, PieceType::Queen));
        squares[4].piece = Some(Piece::new(Colour::Black, PieceType::King));
        squares[5].piece = Some(Piece::new(Colour::Black, PieceType::Bishop));
        squares[6].piece = Some(Piece::new(Colour::Black, PieceType::Knight));
        squares[7].piece = Some(Piece::new(Colour::Black, PieceType::Rook));

        (8..16).for_each(|i| {
            squares[i].piece = Some(Piece::new(Colour::Black, PieceType::Pawn));
        });

        // White pieces
        squares[56].piece = Some(Piece::new(Colour::White, PieceType::Rook));
        squares[57].piece = Some(Piece::new(Colour::White, PieceType::Knight));
        squares[58].piece = Some(Piece::new(Colour::White, PieceType::Bishop));
        squares[59].piece = Some(Piece::new(Colour::White, PieceType::Queen));
        squares[60].piece = Some(Piece::new(Colour::White, PieceType::King));
        squares[61].piece = Some(Piece::new(Colour::White, PieceType::Bishop));
        squares[62].piece = Some(Piece::new(Colour::White, PieceType::Knight));
        squares[63].piece = Some(Piece::new(Colour::White, PieceType::Rook));

        (48..56).for_each(|i| {
            squares[i].piece = Some(Piece::new(Colour::White, PieceType::Pawn));
        });

        Board { squares }
    }

    pub fn get_value_at_square(&self, i: usize, j: usize) -> &Square {
        // convert i and j value to single index
        let index = (i * 8) + j;
        self.squares.get(index).unwrap()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

pub fn board_function() -> String {
    let chess_board = Board::new();
    // Access and print a square for demonstration
    chess_board.squares[0].display()
}
