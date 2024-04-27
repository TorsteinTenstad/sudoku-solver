mod board;
mod board_index_math;
mod board_size;
mod board_transformations;
mod number_set;

use board::Board;
use board_size::BoardSize;

fn main() {
    /*
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
    let mut board = Board::from_board_str(
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
