use std::fmt::Display;

use itertools::Itertools;
use nalgebra::{DMatrix, DMatrixView};

/* ----- */
/* Types */
/* ----- */

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    kind: StateKind,
    smudge: bool,
}

impl State {
    fn swap(&mut self) {
        self.smudge = !self.smudge;
        self.kind.swap()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum StateKind {
    Mirror,
    Ground,
}

impl StateKind {
    fn swap(&mut self) {
        *self = match self {
            Self::Mirror => Self::Ground,
            Self::Ground => Self::Mirror,
        };
    }
}

impl Display for StateKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Self::Mirror => '#',
            Self::Ground => '.',
        };
        write!(f, "{symbol}")
    }
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

// 39037
fn process(input: &str) -> usize {
    let (_, mut matrices) = parsing::parse_matrices(input).unwrap();
    matrices.iter_mut().map(permutate).sum::<usize>()
}

fn permutate(mat: &mut DMatrix<State>) -> usize {
    // try all permutations by swapping each # to . and vice versa, until we find a different reflection
    for row in 0..mat.nrows() {
        for col in 0..mat.ncols() {
            let index = (row, col);
            mat.get_mut(index).unwrap().swap(); // swap smudge
            if let Some(reflection) = find_reflection_with_smudge(&mat) {
                return reflection.value();
            }
            mat.get_mut(index).unwrap().swap(); // swap smudge back to what it was
        }
    }
    0
}

fn find_reflection_with_smudge(mat: &DMatrix<State>) -> Option<Reflection> {
    if let Some(index) = find_reflection_index_with_smudge(mat) {
        return Some(Reflection {
            index,
            axis: Axis::Vertical,
        });
    }
    if let Some(index) = find_reflection_index_with_smudge(&matrix_rotate_left(&mat)) {
        return Some(Reflection {
            index,
            axis: Axis::Horizontal,
        });
    }
    None
}

fn find_reflection_index_with_smudge(mat: &DMatrix<State>) -> Option<usize> {
    let first_row = mat.row(0);
    for (i, curr) in first_row.iter().enumerate() {
        if let Some(next) = first_row.get(i + 1) {
            if curr.kind == next.kind {
                let mat1_view = mat.view((0, 0), (mat.nrows(), i + 1));
                let mat2_view = mat.view((0, i + 1), (mat.nrows(), mat.ncols() - i - 1));

                let mat1 = matrix_flip_rows(mat1_view);

                if are_matrices_identical_min_size_with_smudge(&mat1, &mat2_view) {
                    return Some(i + 1);
                }
            }
        }
    }
    None
}

fn are_matrices_identical_min_size_with_smudge(
    mat1: &DMatrix<State>,
    mat2: &DMatrixView<State>,
) -> bool {
    let min_rows = mat1.nrows();
    let min_cols = mat1.ncols().min(mat2.ncols());

    // println!("Check Identical: rows: {min_rows}, cols: {min_cols}");

    let mut smudged = false;

    for row in 0..min_rows {
        for col in 0..min_cols {
            let index = (row, col);
            let a = mat1.get(index).unwrap();
            let b = mat2.get(index).unwrap();
            if a.kind != b.kind {
                return false;
            }
            if a.smudge || b.smudge {
                smudged = true;
            }
        }
    }
    smudged
}

fn matrix_flip_rows(mat: DMatrixView<State>) -> DMatrix<State> {
    let iter = mat
        .row_iter()
        .flat_map(|row| row.iter().rev().cloned().collect_vec());

    DMatrix::from_row_iterator(mat.nrows(), mat.ncols(), iter)
}

fn matrix_rotate_left(mat: &DMatrix<State>) -> DMatrix<State> {
    // switch rows and columns:
    // row-major: 1,2,3; 4,5,6; 7,8,9;
    // col-major: 3,2,1; 6,5,4; 9,8,7
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
    use super::StateKind;
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
        many1(state)(input)
    }

    fn state(input: &str) -> IResult<&str, State> {
        use nom::character::complete::char;
        map(
            alt((
                value(StateKind::Mirror, char('#')),
                value(StateKind::Ground, char('.')),
            )),
            |kind| State {
                kind,
                smudge: false,
            },
        )(input)
    }
}

/* ------- */
/* Testing */
/* ------- */

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day13_2023_part2() {
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
#....#..#

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(process(input), 1400);
    }
}
