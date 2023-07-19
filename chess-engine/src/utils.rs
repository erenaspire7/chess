pub fn is_within_board_limits(row: i32, col: i32) -> bool {
    row >= 1 && row <= 8 && col >= 1 && col <= 8
}
