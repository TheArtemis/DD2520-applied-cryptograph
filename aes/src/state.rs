use std::fmt;
use std::ops::{Index, IndexMut, BitXor, BitXorAssign};

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
    
    #[inline]
    pub fn get_col(&self, col: usize) -> [u8; 4] {
        assert!(col < 4, "Column must be between 0 and 3");
        [
            self.get(0, col),
            self.get(1, col),
            self.get(2, col),
            self.get(3, col),
        ]
    }

    #[inline]
    pub fn set_col(&mut self, col: usize, values: [u8; 4]) {
        assert!(col < 4, "Column must be between 0 and 3");
        for row in 0..4 {
            self.set(row, col, values[row]);
        }
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

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..4 {
            for col in 0..4 {
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.get(row, col))?;
            }
            if row < 3 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl BitXor for State {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        let mut result = Self::zero();
        for row in 0..4 {
            for col in 0..4 {
                result[(row, col)] = self[(row, col)] ^ other[(row, col)];
            }
        }
        result
    }
}

impl BitXorAssign for State {
    fn bitxor_assign(&mut self, other: Self) {
        for row in 0..4 {
            for col in 0..4 {
                self[(row, col)] ^= other[(row, col)];
            }
        }
    }
}
