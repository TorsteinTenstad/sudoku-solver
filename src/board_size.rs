use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter)]
pub enum BoardSize {
    _4x4,
    _9x9,
    _16x16,
}

impl BoardSize {
    pub fn iter() -> impl Iterator<Item = BoardSize> {
        <BoardSize as IntoEnumIterator>::iter()
    }
}

impl BoardSize {
    pub fn from_cell_count(cell_count: usize) -> Option<Self> {
        Self::iter().find(|&board_size| board_size.cell_count() == cell_count)
    }
    pub fn square_size(&self) -> usize {
        match self {
            BoardSize::_4x4 => 2,
            BoardSize::_9x9 => 3,
            BoardSize::_16x16 => 4,
        }
    }
    pub fn size(&self) -> usize {
        self.square_size() * self.square_size()
    }
    pub fn cell_count(&self) -> usize {
        self.size() * self.size()
    }
    pub fn cell_count_in_row_of_square(&self) -> usize {
        self.square_size() * self.square_size() * self.square_size()
    }
    pub fn number_set(&self) -> std::ops::Range<u8> {
        match self {
            BoardSize::_4x4 => 1..5,
            BoardSize::_9x9 => 1..10,
            BoardSize::_16x16 => 0..16,
        }
    }
    pub fn get_checkered_bool(&self, index: usize) -> bool {
        let short_square_period = index % (2 * self.square_size()) / self.square_size() == 0;

        let row_period = match &self {
            BoardSize::_4x4 => false,
            BoardSize::_9x9 => index % (2 * self.size()) / self.size() == 0,
            BoardSize::_16x16 => false,
        };
        let long_square_period = index % (2 * self.cell_count_in_row_of_square())
            / self.cell_count_in_row_of_square()
            == 0;
        (short_square_period != long_square_period) != row_period
    }
}
