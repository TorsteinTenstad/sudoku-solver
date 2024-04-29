use crate::{
    board::{Board, Cell, SolvedNumber},
    board_is_valid::board_is_valid,
    board_transformations::{promote_singles_to_solved, reduce_from_solved, solve_only_spot},
    number_set::NumberSet,
};

pub enum SolveExitCondition {
    Solved(Board),
    NoChange(Board),
    InvalidBoard,
}

pub struct SolveResult {
    pub exit_condition: SolveExitCondition,
    pub number_of_solved_squares: usize,
    pub unsolved_metric: usize,
    pub iterations: usize,
}

pub fn try_solve(board: Board) -> SolveResult {
    let mut board = board;
    let mut iterations = 0;
    let mut number_of_solved_squares = board.number_of_solved_squares();
    let mut unsolved_metric = board.unsolved_metric();
    let exit_condition = loop {
        if !board_is_valid(&board) {
            break SolveExitCondition::InvalidBoard;
        }
        reduce_from_solved(&mut board);
        promote_singles_to_solved(&mut board);
        solve_only_spot(&mut board);
        iterations += 1;

        let new_number_of_solved_squares = board.number_of_solved_squares();
        let new_unsolved_metric = board.unsolved_metric();
        if number_of_solved_squares == new_number_of_solved_squares
            && unsolved_metric == new_unsolved_metric
        {
            break SolveExitCondition::NoChange(board);
        }
        number_of_solved_squares = new_number_of_solved_squares;
        unsolved_metric = new_unsolved_metric;
        if board.is_solved() {
            break SolveExitCondition::Solved(board);
        }
    };
    SolveResult {
        exit_condition,
        number_of_solved_squares,
        unsolved_metric,
        iterations,
    }
}

struct Guess {
    index: usize,
    set: NumberSet,
    number: u8,
}
fn get_best_guess(board: &Board) -> Option<Guess> {
    let best_option_count = (2..board.board_size.size())
        .find(|option_count| {
            for cell in board.cells.iter() {
                if let Cell::Unsolved(set) = cell {
                    if set.len() == *option_count {
                        return true;
                    }
                }
            }
            false
        })
        .unwrap();
    board
        .cells
        .iter()
        .enumerate()
        .filter_map(|(index, cell)| match cell {
            Cell::Unsolved(set) if set.len() == best_option_count => Some((index, set.clone())),
            _ => None,
        })
        .min_by_key(|(index, set)| {
            set.clone()
                .into_iter()
                .map(|number| {
                    let mut board = board.clone();
                    board.cells[*index] = Cell::Solved(SolvedNumber::new(number));
                    try_solve(board).unsolved_metric
                })
                .sum::<usize>()
        })
        .map(|(index, set)| Guess {
            index,
            number: set.first().unwrap(),
            set,
        })
}

pub fn solve(board: Board) -> SolveResult {
    let solve_result = try_solve(board);
    match solve_result.exit_condition {
        SolveExitCondition::Solved(_) => solve_result,
        SolveExitCondition::InvalidBoard => solve_result,
        SolveExitCondition::NoChange(mut board) => {
            let Some(guess) = get_best_guess(&board) else {
                return SolveResult {
                    exit_condition: SolveExitCondition::InvalidBoard,
                    number_of_solved_squares: 0,
                    unsolved_metric: 0,
                    iterations: 0,
                };
            };
            let mut guess_board = board.clone();
            guess_board.cells[guess.index] = Cell::Solved(SolvedNumber::new(guess.number));
            let solve_result = solve(guess_board);
            match solve_result.exit_condition {
                SolveExitCondition::Solved(_) => solve_result,
                SolveExitCondition::InvalidBoard => {
                    let reduced_guess_set = guess.set.without(guess.number);
                    if Some(guess.number) == reduced_guess_set.single() {
                        board.cells[guess.index] = Cell::Solved(SolvedNumber::new(guess.number));
                    } else {
                        board.cells[guess.index] = Cell::Unsolved(reduced_guess_set);
                    }
                    solve(board)
                }
                SolveExitCondition::NoChange(board) => solve(board),
            }
        }
    }
}
