use std::ops::Range;

#[derive(Debug)]
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

    pub fn add(&mut self, number: u8) {
        self.numbers.push(number);
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
}
