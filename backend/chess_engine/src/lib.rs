// chess_engine/src/lib.rs

// Define your chess-related modules and their contents
mod board;
mod moves;

// Public API for the chess engine
pub use board::ChessBoard;
pub use moves::{Move, generate_legal_moves};

// Example function
pub fn your_chess_engine_function() -> String {
    // Your chess engine logic here
    "Chess engine function called!".to_string()
}

// You can continue to add more modules, structs, and functions as needed.