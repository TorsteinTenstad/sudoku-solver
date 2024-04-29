#[derive(Debug, Clone)]
pub struct NumberSet {
    bitset: u16,
}

impl NumberSet {
    pub fn new() -> Self {
        Self { bitset: u16::MAX }
    }

    pub fn remove(&mut self, number: u8) {
        self.bitset &= !(1 << number);
    }

    pub fn contains(&self, number: u8) -> bool {
        if number > 15 {
            return false;
        }
        self.bitset & (1 << number) != 0
    }

    pub fn single(&self) -> Option<u8> {
        if self.len() == 1 {
            return self.first();
        }
        None
    }

    pub fn len(&self) -> usize {
        self.bitset.count_ones() as usize
    }

    pub fn first(&self) -> Option<u8> {
        if self.bitset == 0 {
            return None;
        }
        Some(self.bitset.trailing_zeros() as u8)
    }

    pub fn without(&self, number: u8) -> Self {
        let mut new = self.clone();
        new.remove(number);
        new
    }

    pub fn into_iter(self) -> std::vec::IntoIter<u8> {
        (0..16)
            .filter(|&i| self.contains(i))
            .collect::<Vec<_>>()
            .into_iter()
    }
}
