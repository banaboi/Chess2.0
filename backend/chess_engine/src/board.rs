use crate::piece::{Piece, Colour, PieceType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub piece: Option<Piece>
}

impl Square {
    pub fn new() -> Self {
        Square { piece: None}
    }
    pub fn place_piece(&mut self, colour: Colour, piece_type: PieceType) {
        self.piece = Some(Piece::new(colour, piece_type));
    }

    pub fn display(&self) -> String {
        if self.piece != None {
            "i am a square with a piece".to_string()
        } else {
            "i am a square with no piece".to_string()
        }
    }
}

pub struct Board {
    squares: Vec<Square>
}

impl Board {
    pub fn new() -> Self {
        let squares = vec![Square::new(); 64];
        Board { squares }
    }
}

pub fn board_function() -> String {
    let chess_board = Board::new();
    // Access and print a square for demonstration
    chess_board.squares[0].display()
}

