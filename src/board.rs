use crate::{board_size::BoardSize, number_set::NumberSet};
use colored::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct SolvedNumber {
    pub number: u8,
    pub checked: bool,
    pub is_starting: bool,
}

impl SolvedNumber {
    pub fn new_starting(number: u8) -> Self {
        Self {
            number,
            checked: false,
            is_starting: true,
        }
    }
    pub fn new(number: u8) -> Self {
        Self {
            number,
            checked: false,
            is_starting: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Solved(SolvedNumber),
    Unsolved(NumberSet),
}

#[derive(Debug, Clone)]
pub struct Board {
    pub board_size: BoardSize,
    pub cells: Vec<Cell>,
}

impl Board {
    pub fn from_board_str(board_str: &str) -> Option<Self> {
        let board_size = BoardSize::from_cell_count(board_str.split_whitespace().count())?;
        Some(Self {
            cells: board_str
                .split_whitespace()
                .map(|c| match c {
                    "_" => Cell::Unsolved(NumberSet::new()),
                    "A" => Cell::Solved(SolvedNumber::new_starting(10)),
                    "B" => Cell::Solved(SolvedNumber::new_starting(11)),
                    "C" => Cell::Solved(SolvedNumber::new_starting(12)),
                    "D" => Cell::Solved(SolvedNumber::new_starting(13)),
                    "E" => Cell::Solved(SolvedNumber::new_starting(14)),
                    "F" => Cell::Solved(SolvedNumber::new_starting(15)),
                    _ => Cell::Solved(SolvedNumber::new_starting(c.parse().unwrap())),
                })
                .collect(),
            board_size,
        })
    }
    pub fn to_display_string(&self) -> String {
        self.cells
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                match cell {
                    Cell::Solved(solved) => {
                        (format!(" {:X} ", solved.number)).color(if solved.is_starting {
                            Color::Cyan
                        } else {
                            Color::White
                        })
                    }
                    //Cell::Unsolved(set) => {format!(" {} ", set.len()).to_string().red()}
                    Cell::Unsolved(_) => "   ".red(),
                }
                .on_custom_color(if self.board_size.get_checkered_bool(index) {
                    CustomColor::new(64, 64, 64)
                } else {
                    CustomColor::new(32, 32, 32)
                })
            })
            .chunks(self.board_size.size())
            .into_iter()
            .map(|chunk| chunk.into_iter().join(""))
            .join("\n")
    }
    pub fn number_of_solved_squares(&self) -> usize {
        self.cells
            .iter()
            .filter(|cell| matches!(cell, Cell::Solved(_)))
            .count()
    }
    pub fn unsolved_metric(&self) -> usize {
        self.cells
            .iter()
            .filter_map(|cell| match cell {
                Cell::Unsolved(number_set) => Some(number_set.len()),
                _ => None,
            })
            .sum()
    }
    pub fn is_solved(&self) -> bool {
        self.number_of_solved_squares() == self.board_size.cell_count()
    }
}
