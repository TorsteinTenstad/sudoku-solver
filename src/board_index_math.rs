use crate::board_size::BoardSize;

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

pub fn get_all_row_indexes(board_size: BoardSize) -> Vec<Vec<usize>> {
    (0..board_size.size())
        .map(|index| get_row_indexes(board_size, index * board_size.size()))
        .collect()
}

pub fn get_all_col_indexes(board_size: BoardSize) -> Vec<Vec<usize>> {
    (0..board_size.size())
        .map(|index| get_col_indexes(board_size, index))
        .collect()
}

pub fn get_all_square_indexes(board_size: BoardSize) -> Vec<Vec<usize>> {
    let v: Vec<usize> = match board_size {
        BoardSize::_16x16 => vec![
            0, 4, 8, 12, 64, 68, 72, 76, 128, 132, 136, 140, 192, 196, 200, 204,
        ],
        _ => todo!(),
    };
    v.iter()
        .map(|index| get_square_indexes(board_size, *index))
        .collect()
}
