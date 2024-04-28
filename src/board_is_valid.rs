use crate::{
    board::{Board, Cell},
    board_index_math::{get_all_col_indexes, get_all_row_indexes, get_all_square_indexes},
};

pub fn board_is_valid(board: &Board) -> bool {
    let board_size = board.board_size;
    let rows_are_valid = groups_are_valid(board, get_all_row_indexes(board_size));
    let cols_are_valid = groups_are_valid(board, get_all_col_indexes(board_size));
    let squares_are_valid = groups_are_valid(board, get_all_square_indexes(board_size));

    rows_are_valid && cols_are_valid && squares_are_valid
}

pub fn groups_are_valid(board: &Board, groups: Vec<Vec<usize>>) -> bool {
    for group in groups.iter() {
        let mut unique_numbers = Vec::<u8>::new();
        for i in group {
            if let Cell::SolvedNumber(solved_number) = &board.cells[*i] {
                if unique_numbers.contains(solved_number) {
                    return false;
                }
                unique_numbers.push(*solved_number);
            }
        }
    }
    true
}
