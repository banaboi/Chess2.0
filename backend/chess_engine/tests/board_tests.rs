// chess_engine/tests/board_tests.rs

use chess_engine::board::board_function;

#[test]
fn test_board_function() {
    assert_eq!(board_function(), 0);
}
