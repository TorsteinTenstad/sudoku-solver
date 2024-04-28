use crate::{board_size::BoardSize, number_set::NumberSet};
use colored::*;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Cell {
    StartingNumber(u8),
    SolvedNumber(u8),
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
                    "_" => Cell::Unsolved(NumberSet::new(board_size.number_set())),
                    "A" => Cell::StartingNumber(10),
                    "B" => Cell::StartingNumber(11),
                    "C" => Cell::StartingNumber(12),
                    "D" => Cell::StartingNumber(13),
                    "E" => Cell::StartingNumber(14),
                    "F" => Cell::StartingNumber(15),
                    _ => Cell::StartingNumber(c.parse().unwrap()),
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
                    Cell::StartingNumber(number) => (format!(" {number:X} ")).cyan().bold(),
                    Cell::SolvedNumber(number) => (format!(" {number:X} ")).white(),
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
            .filter(|cell| matches!(cell, Cell::SolvedNumber(_) | Cell::StartingNumber(_)))
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
