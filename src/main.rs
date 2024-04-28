mod board;
mod board_index_math;
mod board_is_valid;
mod board_size;
mod board_transformations;
mod number_set;
mod solver;
use board::Board;
use board_size::BoardSize;
use solver::{solve, SolveExitCondition};

fn main() {
    /*
    // Simple example
    let mut board = Board::from_board_str(
        BoardSize::_9x9,
        "
        _ _ _ _ _ _ 4 _ _
        7 3 _ 5 9 _ _ _ _
        _ _ _ 7 _ _ 5 3 _
        _ 2 _ _ 7 _ 3 _ _
        _ _ 3 2 5 6 8 9 _
        8 _ 1 _ _ _ _ 2 _
        _ _ _ _ 1 _ 6 _ 7
        _ 6 _ 4 _ _ 9 _ _
        _ _ _ 6 _ 3 _ 5 4
        ",
    );
    */

    // Mestersudoku 5 stars
    let mut board_5_star = Board::from_board_str(
        BoardSize::_16x16,
        "
        F _ _ 0 _ 7 _ _ 4 _ _ _ 1 D _ E
        _ 3 _ 7 _ D _ F _ _ _ _ _ 0 _ _
        8 _ 2 A _ _ _ 5 _ _ D _ C 7 _ _
        C _ 4 _ 2 _ _ _ _ A _ _ _ 8 B _
        _ A _ E _ _ _ _ 2 4 _ _ B C 6 _
        _ _ _ _ _ _ C _ _ _ _ 8 _ _ 1 _
        _ _ B 8 9 4 A _ _ 0 5 _ F _ _ _
        _ _ _ 9 _ B _ _ A 3 C _ _ _ _ 0
        A _ _ _ E 8 9 2 D _ _ _ _ _ _ _
        D 7 _ _ 6 3 _ 1 B _ _ _ _ _ C _
        6 _ F B _ _ 7 _ _ _ _ 3 _ 5 _ _
        _ _ _ C _ 5 _ _ 1 _ _ A 4 _ E _
        _ _ _ _ 8 _ E _ 5 C B _ 3 _ 9 6
        E _ D F _ _ 6 _ _ _ _ 9 0 _ _ B
        _ _ _ _ _ _ _ 9 _ _ _ _ _ _ _ _
        1 _ _ _ _ _ _ _ _ 7 E 0 8 4 _ 2
        ",
    );

    // Mestersudoku 4 stars
    let mut board_4_star = Board::from_board_str(
        BoardSize::_16x16,
        "
        _ B 6 F 2 _ 0 _ _ _ 4 _ 1 _ _ _
        _ _ _ 0 _ _ B 9 _ E _ C _ _ _ _
        _ 5 _ _ _ A _ _ 1 _ _ 0 4 _ _ C
        _ _ _ _ _ _ 5 _ _ _ _ _ D 3 A _
        _ _ _ _ 3 _ _ _ _ _ _ _ A 0 C _
        _ 4 8 D _ _ 1 _ _ _ C _ _ _ _ _
        _ _ 9 _ _ 8 _ 0 _ A _ _ 2 _ _ 3
        _ 3 _ _ D 2 7 C _ _ _ _ 9 _ 4 _
        _ _ _ _ 1 _ _ _ _ _ 9 _ _ 4 _ D
        _ A C _ _ 4 _ _ D 0 E _ _ _ _ _
        0 _ _ _ _ _ 3 _ _ 2 _ _ 6 8 9 _
        9 _ _ _ 5 C _ _ F _ _ 8 _ _ _ _
        _ 8 A _ 0 _ _ _ 5 _ _ 6 _ _ _ _
        4 _ _ E _ _ _ A 9 F _ _ _ 2 3 _
        5 9 _ _ _ 7 4 1 _ _ _ _ _ B _ _
        2 D _ _ 6 _ _ _ B _ _ 4 _ E _ _
        ",
    );

    let board = board_5_star;

    let solve_result = solve(board);
    match solve_result.exit_condition {
        SolveExitCondition::Solved(board) => {
            println!("Solved!");
            println!("{}", board.to_display_string());
        }
        SolveExitCondition::NoChange(board) => {
            println!("No change!");
            println!("{}", board.to_display_string());
        }
        SolveExitCondition::InvalidBoard => {
            println!("Invalid board!");
        }
    }
}
