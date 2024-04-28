use std::ops::Range;

#[derive(Debug, Clone)]
pub struct NumberSet {
    numbers: Vec<u8>,
}

impl NumberSet {
    pub fn new(set: Range<u8>) -> Self {
        NumberSet {
            numbers: set.collect(),
        }
    }

    pub fn remove(&mut self, number: u8) {
        if let Some(index) = self.numbers.iter().position(|&x| x == number) {
            self.numbers.remove(index);
        }
    }

    pub fn contains(&self, number: u8) -> bool {
        self.numbers.contains(&number)
    }

    pub fn single(&self) -> Option<u8> {
        if self.numbers.len() == 1 {
            Some(self.numbers[0])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.numbers.len()
    }

    pub fn first(&self) -> Option<u8> {
        self.numbers.first().copied()
    }

    pub fn without(&self, number: u8) -> Self {
        let mut new_numbers = self.numbers.clone();
        new_numbers.retain(|&x| x != number);
        NumberSet {
            numbers: new_numbers,
        }
    }

    pub fn into_iter(&self) -> std::vec::IntoIter<u8> {
        self.numbers.clone().into_iter()
    }
}
