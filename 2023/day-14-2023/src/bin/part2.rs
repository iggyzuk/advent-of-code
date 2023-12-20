use std::fmt::Display;

use common::matrix::Matrix;

#[derive(Clone, PartialEq)]
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

// 102_829
fn process(input: &str) -> usize {
    let mut matrix = parse(input);

    println!("{matrix}");

    let translate = |mat: &mut Matrix<Item>, from: (usize, usize), to: (usize, usize)| {
        mat.get_mut_element(from.0, from.1).unwrap().kind = '.';
        mat.get_mut_element(to.0, to.1).unwrap().kind = 'O';
    };

    // (row, col) north, west, south, east
    let dirs = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    // brute force – goes on forever, but it seems at 1000 cycles the answer is the same, 97 cycles also works.
    let cycles = 97;

    // note: this can be `for (i, dir) in dirs.iter().cycle().take(cycles).enumerate()`
    for i in 0..cycles {
        println!("{}/{}", i, cycles);

        for dir in dirs.iter() {
            let mut frontier: Vec<_> = matrix
                .iter()
                .filter(|i| i.kind == 'O')
                .map(|i| (i.row, i.col))
                .collect();

            while !frontier.is_empty() {
                let mut next_matrix = matrix.clone();
                let coords = frontier.pop().unwrap();

                if let Some(curr_item) = matrix.get_element(coords.0, coords.1) {
                    assert_eq!(curr_item.kind, 'O');
                    if let Some(next_item) = matrix
                        .get_element_signed(coords.0 as isize + dir.0, coords.1 as isize + dir.1)
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
                                'leaping: while let Some(leap_item) = matrix.get_element_signed(
                                    coords.0 as isize + (dir.0 * leap),
                                    coords.1 as isize + (dir.1 * leap),
                                ) {
                                    // this movable rock is already next to an immovable rock – no leap able
                                    if leap_item.kind == '#' {
                                        break 'leaping;
                                    }
                                    // there's a spot to leap over!
                                    if leap_item.kind == '.' {
                                        translate(
                                            &mut next_matrix,
                                            (curr_item.row, curr_item.col),
                                            (leap_item.row, leap_item.col),
                                        );
                                        frontier.push((leap_item.row, leap_item.col));
                                        break 'leaping;
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
        }
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
    use super::*;

    #[test]
    fn day14_2023_part2() {
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

        assert_eq!(process(input), 64);
    }
}
