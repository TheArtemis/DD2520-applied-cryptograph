use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    /* 16 bytes of data
    
        Column Major matrix representation:
    
        | b0  b4  b8  b12 |
        | b1  b5  b9  b13 |
        | b2  b6  b10 b14 |
        | b3  b7  b11 b15 |
    */
    
    data: [u8; 16],
}

impl State {
    pub fn new(data: [u8; 16]) -> Self {
        Self { data }
    }

    pub fn zero() -> Self {
        Self::new([0; 16])
    }

    #[inline]
    fn index_offset(row: usize, col: usize) -> usize {
        col * 4 + row
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> u8 {
        assert!(row < 4 && col < 4, "Row and column must be between 0 and 3");
        self.data[Self::index_offset(row, col)]
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        assert!(row < 4 && col < 4, "Row and column must be between 0 and 3");
        self.data[Self::index_offset(row, col)] = value;
    }   
}

impl Index<(usize, usize)> for State {
    type Output = u8;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(row < 4 && col < 4, "Row and column must be between 0 and 3");
        &self.data[Self::index_offset(row, col)]
    }    
}

impl IndexMut<(usize, usize)> for State {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        assert!(row < 4 && col < 4, "Row and column must be between 0 and 3");
        &mut self.data[Self::index_offset(row, col)]
    } 
}