use std::{fmt::Display, slice::Iter, usize};

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    nrows: usize,
    ncols: usize,
    data: Vec<T>,
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.nrows {
            write!(f, "[")?;
            for col in 0..self.ncols {
                let value = &self.data[row * self.ncols + col];
                write!(f, "{}", value)?;
                if col < self.ncols - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<T> Matrix<T> {
    pub fn nrows(&self) -> usize {
        self.nrows
    }

    pub fn ncols(&self) -> usize {
        self.ncols
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn new(nrows: usize, ncols: usize, data: Vec<T>) -> Self {
        Self { nrows, ncols, data }
    }

    pub fn get_element(&self, row: usize, col: usize) -> Option<&T> {
        return if self.check_coords(row as isize, col as isize) {
            Some(&self.data[row * self.ncols + col])
        } else {
            None
        };
    }

    pub fn get_element_signed(&self, row: isize, col: isize) -> Option<&T> {
        return if self.check_coords(row as isize, col as isize) {
            Some(&self.data[row as usize * self.ncols + col as usize])
        } else {
            None
        };
    }

    pub fn get_mut_element(&mut self, row: usize, col: usize) -> Option<&mut T> {
        return if self.check_coords(row as isize, col as isize) {
            Some(&mut self.data[row * self.ncols + col])
        } else {
            None
        };
    }

    pub fn get_mut_element_signed(&mut self, row: isize, col: isize) -> Option<&mut T> {
        return if self.check_coords(row as isize, col as isize) {
            Some(&mut self.data[row as usize * self.ncols + col as usize])
        } else {
            None
        };
    }

    fn check_coords(&self, row: isize, col: isize) -> bool {
        row >= 0 && col >= 0 && row < self.nrows as isize && col < self.ncols as isize
    }

    pub fn get_row(&self, row: usize) -> Matrix<&T> {
        let mut data = vec![];
        for col in 0..self.ncols {
            data.push(&self.data[row * self.ncols + col]);
        }
        Matrix::new(1, self.nrows, data)
    }

    pub fn get_col(&self, col: usize) -> Matrix<&T> {
        let mut data = vec![];
        for row in 0..self.nrows {
            data.push(&self.data[row * self.ncols + col]);
        }
        Matrix::new(1, self.ncols, data)
    }
}

impl<T: Clone> Matrix<&T> {
    pub fn to_owned(self) -> Matrix<T> {
        Matrix::from_iterator(self.nrows, self.ncols, self.data.into_iter().cloned())
    }
}

impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self {
            nrows: self.nrows.clone(),
            ncols: self.ncols.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T> Matrix<T> {
    pub fn from_iterator<I: Iterator<Item = T>>(nrows: usize, ncols: usize, iter: I) -> Self {
        let mut data = Vec::new();
        let mut iter = iter;
        while let Some(next) = iter.next() {
            data.push(next);
        }
        Self { nrows, ncols, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix() {
        let m: Matrix<usize> =
            Matrix::from_iterator(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().rev());

        assert_eq!(m.get_element(0, 0), Some(&9));
        assert_eq!(m.get_element(1, 1), Some(&5));
        assert_eq!(m.get_element(2, 0), Some(&3));
        assert_eq!(m.get_element(0, 2), Some(&7));
        assert_eq!(m.get_element(2, 2), Some(&1));
        assert_eq!(m.get_element(3, 3), None);

        assert_eq!(m.get_col(0).to_owned(), Matrix::new(1, 3, vec![9, 6, 3]));
        assert_eq!(m.get_col(1).to_owned(), Matrix::new(1, 3, vec![8, 5, 2]));
        assert_eq!(m.get_row(0).to_owned(), Matrix::new(1, 3, vec![9, 8, 7]));
        assert_eq!(m.get_row(1).to_owned(), Matrix::new(1, 3, vec![6, 5, 4]));
    }
}
