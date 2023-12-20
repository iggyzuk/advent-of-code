use std::fmt::Display;

use mx::Matrix;

mod mx {
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
            let index = row * self.ncols + col;
            return if self.is_index_valid(index as isize) {
                Some(&self.data[index])
            } else {
                None
            };
        }

        pub fn get_element_signed(&self, row: isize, col: isize) -> Option<&T> {
            let index = row * self.ncols as isize + col;
            return if self.is_index_valid(index as isize) {
                Some(&self.data[index as usize])
            } else {
                None
            };
        }

        pub fn get_mut_element(&mut self, row: usize, col: usize) -> Option<&mut T> {
            let index = row * self.ncols + col;
            return if self.is_index_valid(index as isize) {
                Some(&mut self.data[index])
            } else {
                None
            };
        }

        pub fn get_mut_element_signed(&mut self, row: isize, col: isize) -> Option<&mut T> {
            let index = row * self.ncols as isize + col;
            return if self.is_index_valid(index as isize) {
                Some(&mut self.data[index as usize])
            } else {
                None
            };
        }

        fn is_index_valid(&self, index: isize) -> bool {
            index >= 0 && index < self.nrows as isize * self.ncols as isize
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
}

#[derive(Clone)]
struct Item {
    row: usize,
    col: usize,
    kind: char,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Item {
    fn new(row: usize, col: usize, kind: char) -> Self {
        Self { row, col, kind }
    }
}

fn main() {
    println!("Starting Process");
    let now = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("Finished in {:?}", now.elapsed());
    println!("Solution: {:?}", output);
}

// 105_461
fn process(input: &str) -> usize {
    let mut matrix = parse(input);

    let mut frontier: Vec<_> = matrix
        .iter()
        .filter(|i| i.kind == 'O')
        .map(|i| (i.row, i.col))
        .collect();

    println!("{matrix}");

    let translate = |mat: &mut Matrix<Item>, from: (usize, usize), to: (usize, usize)| {
        mat.get_mut_element(from.0, from.1).unwrap().kind = '.';
        mat.get_mut_element(to.0, to.1).unwrap().kind = 'O';
    };

    while !frontier.is_empty() {
        let mut next_matrix = matrix.clone();
        let coords = frontier.pop().unwrap();

        if let Some(curr_item) = matrix.get_element(coords.0, coords.1) {
            assert_eq!(curr_item.kind, 'O');
            if let Some(next_item) =
                matrix.get_element_signed(coords.0 as isize - 1, coords.1 as isize)
            {
                match next_item.kind {
                    // next item is an empty space – move to it
                    '.' => {
                        translate(
                            &mut next_matrix,
                            (curr_item.row, curr_item.col),
                            (next_item.row, next_item.col),
                        );

                        frontier.push((next_item.row, next_item.col));
                    }
                    // next item is a movable rock – leap over it
                    'O' => {
                        let mut leap = 2;
                        while let Some(leap_item) =
                            matrix.get_element_signed(coords.0 as isize - leap, coords.1 as isize)
                        {
                            // this movable rock is already next to an immovable rock – no leap able
                            if leap_item.kind == '#' {
                                break;
                            }
                            // there's a spot to leap over!
                            if leap_item.kind == '.' {
                                translate(
                                    &mut next_matrix,
                                    (curr_item.row, curr_item.col),
                                    (leap_item.row, leap_item.col),
                                );
                                frontier.push((leap_item.row, leap_item.col));
                                break;
                            }

                            // keep increasing the leap distance
                            leap += 1;
                        }
                    }
                    // we hit an immovable rock – stop
                    '#' => {}
                    // do nothing
                    _ => {}
                }
            }
        }
        matrix = next_matrix;
    }

    println!("{matrix}");

    matrix
        .iter()
        .filter(|i| i.kind == 'O')
        .map(|i| matrix.nrows() - i.row)
        // .inspect(|x| println!("{x}"))
        .sum()
}

fn parse(input: &str) -> Matrix<Item> {
    Matrix::from_iterator(
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
        input.lines().enumerate().flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| Item::new(row, col, char))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::mx::*;
    use super::*;

    #[test]
    fn day14_2023_part1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(process(input), 136);
    }

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
