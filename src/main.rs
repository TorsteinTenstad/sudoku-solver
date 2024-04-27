mod board;
mod board_index_math;
mod board_size;
mod board_transformations;
mod number_set;

use board::Board;
use board_size::BoardSize;

fn main() {
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

    let mut number_of_solved_squares = board.number_of_solved_squares();
    let mut unsolved_metric = board.unsolved_metric();
    let mut i = 0;
    let solved = loop {
        println!(
            "Solved: {}, unsolved metric {}",
            number_of_solved_squares, unsolved_metric
        );
        println!("{}", board.to_display_string());

        board_transformations::reduce_unsolved_number_sets(&mut board);
        board_transformations::promote_singles_to_solved(&mut board);
        i += 1;

        let new_number_of_solved_squares = board.number_of_solved_squares();
        let new_unsolved_metric = board.unsolved_metric();
        if number_of_solved_squares == new_number_of_solved_squares
            && unsolved_metric == new_unsolved_metric
        {
            break false;
        }
        number_of_solved_squares = new_number_of_solved_squares;
        unsolved_metric = new_unsolved_metric;
        if board.is_solved() {
            break true;
        }
    };
    match solved {
        true => println!("Solved after {} iterations", i),
        false => println!("Failed to progress after {} iterations", i),
    }
    println!(
        "Solved: {}, unsolved metric {}",
        number_of_solved_squares, unsolved_metric
    );
    println!("{}", board.to_display_string());
}
