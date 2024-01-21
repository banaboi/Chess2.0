use crate::piece::{Colour, PieceType};

pub struct Board {
    position: [[u64; 6]; 2],
}

impl Board {
    pub fn new() -> Self {
        let position: [[u64; 6]; 2] = [[0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]];

        Board { position }
    }

    pub fn place_piece(&mut self, piece_type: PieceType, colour: Colour, i: usize, j: usize) {
        let index: usize = (i * 8) + j;
        let mask: u64 = 1 << index;
        let i: usize = colour.into();
        let j: usize = piece_type.into();
        self.position[i][j] |= mask;
    }

    pub fn get_value_at_square(&self, i: usize, j: usize) -> String {
        let index: usize = (i * 8) + j;
        let mut result: String = String::from("");
        for (row_index, row) in self.position.iter().enumerate() {
            for (col_index, &element) in row.iter().enumerate() {
                if (element >> index) & 1 == 1 {
                    let colour: Colour = row_index.into();
                    let piece_type: PieceType = col_index.into();
                    let chess_coordinate = indices_to_chess_notation(i, j);
                    let print_result: String =
                        format!("{} {} at {}", colour, piece_type, chess_coordinate);
                    result = print_result;
                    break;
                }
            }
        }
        result
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

pub fn board_function() -> u64 {
    let chess_board = Board::default();
    let i: usize = Colour::White.into();
    let j: usize = PieceType::Pawn.into();
    chess_board.position[i][j]
}

pub fn indices_to_chess_notation(i: usize, j: usize) -> String {
    let file = (j as u8 + b'A') as char;
    let rank = (8 - i).to_string();
    format!("{}{}", file, rank)
}
