use crate::board::{Board, Cell};
use crate::board_index_math::{get_col_indexes, get_row_indexes, get_square_indexes};

pub fn promote_singles_to_solved(board: &mut Board) {
    for (_index, cell) in board.cells.iter_mut().enumerate() {
        if let Cell::Unsolved(number_set) = cell {
            if let Some(number) = number_set.single() {
                *cell = Cell::SolvedNumber(number);
            }
        }
    }
}

pub fn get_solved(board: &Board) -> Vec<(usize, u8)> {
    board
        .cells
        .iter()
        .enumerate()
        .filter_map(|(index, cell)| match cell {
            Cell::SolvedNumber(number) => Some((index, *number)),
            Cell::StartingNumber(number) => Some((index, *number)),
            _ => None,
        })
        .collect()
}

pub fn reduce_same_group_number_sets<F>(
    board: &mut Board,
    index: usize,
    number: u8,
    get_same_group_indices: F,
) where
    F: Fn(usize) -> Vec<usize>,
{
    for same_group_index in get_same_group_indices(index) {
        if let Cell::Unsolved(number_set) = &mut board.cells[same_group_index] {
            number_set.remove(number);
        }
    }
}

pub fn reduce_unsolved_number_sets(board: &mut Board) {
    let board_size = board.board_size.clone();
    let solved = get_solved(board);
    for (index, number) in solved {
        reduce_same_group_number_sets(board, index, number, |i| {
            get_row_indexes(board_size.clone(), i)
        });
        reduce_same_group_number_sets(board, index, number, |i| {
            get_col_indexes(board_size.clone(), i)
        });
        reduce_same_group_number_sets(board, index, number, |i| {
            get_square_indexes(board_size.clone(), i)
        });
    }
}
