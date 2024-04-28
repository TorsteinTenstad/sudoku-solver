use crate::board::{Board, Cell};
use crate::board_index_math::{
    get_all_col_indexes, get_all_row_indexes, get_all_square_indexes, get_col_indexes,
    get_row_indexes, get_square_indexes,
};

pub fn promote_singles_to_solved(board: &mut Board) {
    for cell in board.cells.iter_mut() {
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

pub fn solve_only_spot_for_number_in_group(board: &mut Board, number: u8, group: Vec<usize>) {
    let mut only_possible_cell_index: Option<usize> = None;
    for index in group.iter() {
        match &board.cells.get(*index).unwrap() {
            Cell::Unsolved(number_set) => {
                if number_set.contains(number) {
                    if only_possible_cell_index.is_none() {
                        only_possible_cell_index = Some(*index);
                    } else {
                        only_possible_cell_index = None;
                        break;
                    }
                }
            }
            Cell::SolvedNumber(solved_number) => {
                if *solved_number == number {
                    only_possible_cell_index = None;
                    break;
                }
            }
            Cell::StartingNumber(starting_number) => {
                if *starting_number == number {
                    only_possible_cell_index = None;
                    break;
                }
            }
        }
    }
    if let Some(index) = only_possible_cell_index {
        board.cells[index] = Cell::SolvedNumber(number);
    }
}

pub fn solve_only_spot(board: &mut Board) {
    let board_size = board.board_size;
    for number in board_size.number_set() {
        for group in get_all_row_indexes(board_size) {
            solve_only_spot_for_number_in_group(board, number, group);
        }
        for group in get_all_col_indexes(board_size) {
            solve_only_spot_for_number_in_group(board, number, group);
        }
        for group in get_all_square_indexes(board_size) {
            solve_only_spot_for_number_in_group(board, number, group);
        }
    }
}

pub fn reduce_from_solved_group<F>(
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

pub fn reduce_from_solved(board: &mut Board) {
    let board_size = board.board_size;
    let solved = get_solved(board);
    for (index, number) in solved {
        reduce_from_solved_group(board, index, number, |i| get_row_indexes(board_size, i));
        reduce_from_solved_group(board, index, number, |i| get_col_indexes(board_size, i));
        reduce_from_solved_group(board, index, number, |i| get_square_indexes(board_size, i));
    }
}
