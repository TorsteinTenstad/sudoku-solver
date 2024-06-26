mod board;
mod board_index_math;
mod board_is_valid;
mod board_size;
mod board_transformations;
mod number_set;
mod number_set_tests;
mod solver;
use anyhow::Context;
use board::Board;
use clap::Parser;
use solver::{solve, SolveExitCondition};
use std::time::Instant;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    board_file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let board_string = std::fs::read_to_string(args.board_file).context("reading board file")?;
    let board = Board::from_board_str(&board_string).context("parsing board")?;

    let now = Instant::now();
    let solve_result = solve(board);
    let elapsed = now.elapsed();

    match solve_result.exit_condition {
        SolveExitCondition::Solved(board) => {
            println!("Solved in {:?}", elapsed);
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
    Ok(())
}
