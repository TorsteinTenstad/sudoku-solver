use crate::{board, board_size::BoardSize};

pub fn get_row_indexes(board_size: BoardSize, index: usize) -> Vec<usize> {
    let row = index / board_size.size();
    (0..board_size.size())
        .map(|col| row * board_size.size() + col)
        .collect()
}

pub fn get_col_indexes(board_size: BoardSize, index: usize) -> Vec<usize> {
    let col = index % board_size.size();
    (0..board_size.size())
        .map(|row| row * board_size.size() + col)
        .collect()
}

pub fn get_square_indexes(board_size: BoardSize, index: usize) -> Vec<usize> {
    let row = index / board_size.size();
    let col = index % board_size.size();
    let square_row = row / board_size.square_size();
    let square_col = col / board_size.square_size();
    (0..board_size.square_size())
        .flat_map(|square_row_offset| {
            (0..board_size.square_size()).map(move |square_col_offset| {
                (square_row * board_size.square_size() + square_row_offset) * board_size.size()
                    + square_col * board_size.square_size()
                    + square_col_offset
            })
        })
        .collect()
}
