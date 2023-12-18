use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use nalgebra::{DMatrix, DMatrixView};

/* ------------------ */
/* Idea for Algorithm */
/* ----- -------------*/

// vertical
// for each row of the matrix
//   for each scalar in row 0..n-1
//     check if current is the same as the next
//       create a matrix slice [scalar_index, matrix.rows]
//       flip all rows (1,2,3) => (3,2,1)
//       create another matrix (current_index + 1..)
//       check if all scalars are the same in both matrices

// horizontal – if no vertical reflection was found
// repeat but first rotate matrix values left, then find index
// multiply this result by 100

// sum all results

/* ----- */
/* Types */
/* ----- */

#[derive(Clone, PartialEq, Debug)]
pub enum State {
    Mirror,
    Ground,
}

impl State {
    fn swap(&mut self) {
        *self = match self {
            State::Mirror => State::Ground,
            State::Ground => State::Mirror,
        };
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            State::Mirror => '#',
            State::Ground => '.',
        };
        write!(f, "{symbol}")
    }
}

/* ----- */
/* Setup */
/* ----- */

fn main() {
    println!("Starting Process");
    let now = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("Finished in {:?}", now.elapsed());
    println!("Solution: {:?}", output);
}

/* ----- */
/* Logic */
/* ----- */

// 28_367 (too low)
// 28_398 (too low)
// 30_279 (too low)
fn process(input: &str) -> usize {
    let (_, mut matrices) = parsing::parse_matrices(input).unwrap();
    matrices.iter_mut().map(permutate).sum::<usize>()
}

#[derive(Hash, PartialEq, Debug, Clone)]
struct Reflection {
    index: usize,
    axis: Axis,
}

impl Reflection {
    fn value(&self) -> usize {
        match self.axis {
            Axis::Vertical => self.index,
            Axis::Horizontal => self.index * 100,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Axis {
    Vertical,
    Horizontal,
}

fn permutate(mat: &mut DMatrix<State>) -> usize {
    let mut original_reflections = HashMap::new();
    if let Some(reflection) = find_reflection(&mat) {
        original_reflections.insert(reflection.axis.clone(), reflection);
    }
    // now try all permutations by swapping each # to . and vice versa, until we find a different reflection
    for row in 0..mat.nrows() {
        for col in 0..mat.ncols() {
            // swap smudge
            mat.get_mut((row, col)).unwrap().swap();

            println!("{mat}");

            // try to find a different reflection in this new matrix
            for reflection in find_reflections_both_axis(&mat) {
                println!("{reflection:?}");
                // we have a reflection on this axis – check that index is different
                if let Some(original_reflection) = original_reflections.get(&reflection.axis) {
                    if original_reflection.index != reflection.index {
                        return reflection.value();
                    }
                }
                // we don't have a reflection on this axis – it's safe to use it
                else {
                    return reflection.value();
                }
            }

            // swap smudge back to what it was
            mat.get_mut((row, col)).unwrap().swap();
        }
    }

    // since we haven't found any other reflections let's return the original reflection if there was one
    return if original_reflections.len() > 0 {
        *original_reflections
            .into_iter()
            .map(|x| x.1.value())
            .collect_vec()
            .first()
            .unwrap()
    } else {
        0
    };
}

fn find_reflection(mat: &DMatrix<State>) -> Option<Reflection> {
    if let Some(index) = find_reflection_index(mat) {
        Some(Reflection {
            index,
            axis: Axis::Vertical,
        })
    } else if let Some(index) = find_reflection_index(&matrix_rotate_left(&mat)) {
        Some(Reflection {
            index,
            axis: Axis::Horizontal,
        })
    } else {
        None
    }
}

fn find_reflections_both_axis(mat: &DMatrix<State>) -> Vec<Reflection> {
    let mut reflections = vec![];
    if let Some(index) = find_reflection_index(mat) {
        reflections.push(Reflection {
            index,
            axis: Axis::Vertical,
        });
    }
    if let Some(index) = find_reflection_index(&matrix_rotate_left(&mat)) {
        reflections.push(Reflection {
            index,
            axis: Axis::Horizontal,
        });
    }
    reflections
}

fn find_reflection_index(mat: &DMatrix<State>) -> Option<usize> {
    let first_row = mat.row(0);
    for (i, curr) in first_row.iter().enumerate() {
        if let Some(next) = first_row.get(i + 1) {
            if curr == next {
                // slice first half of the matrix and reverse all rows
                let mat1_view = mat.view((0, 0), (mat.nrows(), i + 1));
                let mat1 = matrix_flip_rows(&mat1_view);

                // slice the second half of the matrix
                let mat2_view = mat.view((0, i + 1), (mat.nrows(), mat.ncols() - i - 1));

                // check if matrices are identical by using the min size of either matrix
                if are_matrices_identical_min_size(&mat1, &mat2_view) {
                    return Some(i + 1);
                }
            }
        }
    }
    None
}

fn are_matrices_identical_min_size(mat1: &DMatrix<State>, mat2: &DMatrixView<State>) -> bool {
    let min_rows = mat1.nrows();
    let min_cols = mat1.ncols().min(mat2.ncols());

    // println!("Check Identical: rows: {min_rows}, cols: {min_cols}");

    for row in 0..min_rows {
        for col in 0..min_cols {
            let index = (row, col);
            let a = mat1.get(index).unwrap();
            let b = mat2.get(index).unwrap();
            if a != b {
                return false;
            }
        }
    }
    true
}

fn matrix_flip_rows(mat: &DMatrixView<State>) -> DMatrix<State> {
    let mut data = vec![];
    for row in mat.row_iter() {
        for scalar in row.iter().rev() {
            data.push(scalar.clone())
        }
    }
    DMatrix::from_row_slice(mat.nrows(), mat.ncols(), data.as_slice())
}

fn matrix_rotate_left(mat: &DMatrix<State>) -> DMatrix<State> {
    // row-major: 1,2,3; 4,5,6; 7,8,9;
    // col-major: 3,2,1; 6,5,4; 9,8,7

    // switch rows and columns
    let iter = mat
        .row_iter()
        .flat_map(|row| row.iter().rev().cloned().collect_vec());

    DMatrix::from_iterator(mat.ncols(), mat.nrows(), iter)
}

/* ------- */
/* Parsing */
/* ------- */

mod parsing {
    use super::State;
    use nalgebra::{DMatrix, RowDVector};
    use nom::{
        branch::alt,
        character::complete::newline,
        combinator::{map, opt, value},
        multi::{many0, many1},
        sequence::terminated,
        IResult,
    };

    pub fn parse_matrices(input: &str) -> IResult<&str, Vec<DMatrix<State>>> {
        many0(parse_matrix)(input)
    }

    fn parse_matrix(input: &str) -> IResult<&str, DMatrix<State>> {
        terminated(
            map(many1(parse_row), |rows| DMatrix::from_rows(rows.as_slice())),
            opt(newline),
        )(input)
    }

    fn parse_row(input: &str) -> IResult<&str, RowDVector<State>> {
        map(terminated(states, opt(newline)), |states| {
            RowDVector::from_vec(states)
        })(input)
    }

    fn states(input: &str) -> IResult<&str, Vec<State>> {
        use nom::character::complete::char;
        many1(alt((
            value(State::Mirror, char('#')),
            value(State::Ground, char('.')),
        )))(input)
    }
}

/* ------- */
/* Testing */
/* ------- */

#[cfg(test)]
mod tests {
    use nalgebra::matrix;

    use super::*;

    #[test]
    fn day13_2023_part2() {
        // row 45 col 56
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(process(input), 400);
    }

    #[test]
    fn rotation_matrix() {
        let mat = matrix![1,2,3;4,5,6;7,8,9];
        let mut data = vec![];
        for row in mat.row_iter() {
            for scalar in row.iter().rev() {
                data.push(scalar.clone())
            }
        }
        let rot = DMatrix::from_vec(mat.nrows(), mat.ncols(), data);

        println!("{rot}");

        assert_eq!(rot[(0, 0)], 3);
        assert_eq!(rot[(1, 0)], 2);
        assert_eq!(rot[(2, 0)], 1);
        assert_eq!(rot[(0, 1)], 6);
        assert_eq!(rot[(1, 1)], 5);
        assert_eq!(rot[(2, 1)], 4);
        assert_eq!(rot[(0, 2)], 9);
        assert_eq!(rot[(1, 2)], 8);
        assert_eq!(rot[(2, 2)], 7);
    }
}
