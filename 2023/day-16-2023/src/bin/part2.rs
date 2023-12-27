use std::collections::HashSet;
use std::fmt::Display;

use common::Matrix;
use common::Vec2;

#[derive(Clone)]
struct Cell {
    symbol: Symbol,
    energy: HashSet<Vec2<isize>>,
}

impl Cell {
    fn new(symbol: Symbol) -> Cell {
        Self {
            symbol,
            energy: HashSet::new(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[derive(Clone)]
enum Symbol {
    Empty,
    ForwardSlash,
    BackSlash,
    Vertical,
    Horizontal,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl TryFrom<char> for Symbol {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Symbol::Empty),
            '/' => Ok(Symbol::ForwardSlash),
            '\\' => Ok(Symbol::BackSlash),
            '|' => Ok(Symbol::Vertical),
            '-' => Ok(Symbol::Horizontal),
            _ => Err(format!("could not parse symbol from: {value}")),
        }
    }
}

impl From<&Symbol> for char {
    fn from(value: &Symbol) -> Self {
        match value {
            Symbol::Empty => '.',
            Symbol::ForwardSlash => '/',
            Symbol::BackSlash => '\\',
            Symbol::Vertical => '|',
            Symbol::Horizontal => '-',
        }
    }
}

#[derive(Clone, PartialEq)]
struct Beam {
    pos: Vec2<isize>,
    dir: Vec2<isize>,
}

impl Beam {
    fn new(pos: Vec2<isize>, dir: Vec2<isize>, mat: &mut Matrix<Cell>) -> Option<Self> {
        assert_ne!(dir, Vec2::ZERO, "direction should never be zero");
        if let Some(element) = mat.get_mut_element_signed(pos.y, pos.x) {
            element.energy.insert(dir);
            return Some(Self { pos, dir });
        }
        None
    }

    fn step(&mut self, mat: &Matrix<Cell>) -> BeamStep {
        if let Some(cell) = mat.get_element_signed(self.pos.y, self.pos.x) {
            let beam_step = self.step_on_symbol(&cell.symbol);
            if beam_step == BeamStep::Moved || beam_step == BeamStep::Reflected {
                self.advance_forward();
            }
            return beam_step;
        }
        // walked out of bounds
        BeamStep::Faded
    }

    fn advance_forward(&mut self) {
        self.pos = self.pos + self.dir;
    }

    fn step_on_symbol(&mut self, symbol: &Symbol) -> BeamStep {
        match symbol {
            Symbol::Empty => BeamStep::Moved,
            Symbol::ForwardSlash => {
                match self.dir {
                    Vec2::LEFT => self.rotate_left(),  // / <---
                    Vec2::RIGHT => self.rotate_left(), // ---> /
                    Vec2::UP => self.rotate_right(),   // ^ /
                    Vec2::DOWN => self.rotate_right(), // v /
                    _ => panic!("beam direction is invalid: {}", self.dir),
                }
                BeamStep::Reflected
            }
            Symbol::BackSlash => {
                match self.dir {
                    Vec2::LEFT => self.rotate_right(),  // \ <---
                    Vec2::RIGHT => self.rotate_right(), // ---> \
                    Vec2::UP => self.rotate_left(),     // ^ \
                    Vec2::DOWN => self.rotate_left(),   // v \
                    _ => panic!("beam direction is invalid: {}", self.dir),
                }
                BeamStep::Reflected
            }
            Symbol::Vertical => match self.dir {
                Vec2::LEFT | Vec2::RIGHT => BeamStep::Split(self.split()),
                Vec2::UP | Vec2::DOWN => BeamStep::Moved,
                _ => panic!("beam direction is invalid: {}", self.dir),
            },
            Symbol::Horizontal => match self.dir {
                Vec2::LEFT | Vec2::RIGHT => BeamStep::Moved,
                Vec2::UP | Vec2::DOWN => BeamStep::Split(self.split()),
                _ => panic!("beam direction is invalid: {}", self.dir),
            },
        }
    }

    fn rotate_left(&mut self) {
        self.dir.rotate_left();
    }

    fn rotate_right(&mut self) {
        self.dir.rotate_right();
    }

    fn split(&mut self) -> Vec<Beam> {
        let mut beams = vec![];

        let mut left_beam = self.clone();
        left_beam.rotate_left();
        beams.push(left_beam);

        let mut right_beam = self.clone();
        right_beam.rotate_right();
        beams.push(right_beam);

        beams
    }
}

#[derive(PartialEq)]
enum BeamStep {
    Moved,
    Reflected,
    Split(Vec<Beam>),
    Faded,
}

fn main() {
    println!("Starting Process");
    let now = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("Finished in {:?}", now.elapsed());
    println!("Solution: {:?}", output);
}

// 7896
fn process(input: &str) -> usize {
    // parse input into a matrix
    let matrix = Matrix::from_iterator(
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
        input.lines().flat_map(|line| {
            line.chars()
                .map(move |char| Cell::new(Symbol::try_from(char).unwrap()))
        }),
    );
    println!("{matrix}");

    // shoot beams from every edge and return the highest number of energized cells from any one beam
    let mut energies = vec![];

    // left edge – right dir
    let dir = Vec2::RIGHT;
    for row in 0..matrix.nrows() {
        let start_pos = Vec2::new(0, row as isize);
        energies.push(get_total_energy(matrix.clone(), start_pos, dir));
    }

    // right edge – left dir
    let dir: Vec2<isize> = Vec2::LEFT;
    for row in 0..matrix.nrows() {
        let start_pos = Vec2::new(matrix.nrows() as isize, row as isize);
        energies.push(get_total_energy(matrix.clone(), start_pos, dir));
    }

    // top edge – down dir
    let dir: Vec2<isize> = Vec2::DOWN;
    for col in 0..matrix.ncols() {
        let start_pos = Vec2::new(col as isize, 0);
        energies.push(get_total_energy(matrix.clone(), start_pos, dir));
    }

    // bottom edge – down dir
    let dir: Vec2<isize> = Vec2::UP;
    for col in 0..matrix.ncols() {
        let start_pos = Vec2::new(col as isize, matrix.nrows() as isize);
        energies.push(get_total_energy(matrix.clone(), start_pos, dir));
    }

    energies.into_iter().max().unwrap()
}

fn get_total_energy(mut matrix: Matrix<Cell>, pos: Vec2<isize>, dir: Vec2<isize>) -> usize {
    // all active beams
    let mut beams = vec![];

    // add first beam
    if let Some(beam) = Beam::new(pos, dir, &mut matrix) {
        step(beam, &mut beams, &mut matrix);
    }

    // steps all active beams until they all fade –  this can happen when beams go out of bounds or beam's current cell is already energized in the same direction
    while !beams.is_empty() {
        let mut next_beams: Vec<Beam> = vec![];
        for beam in beams {
            step(beam, &mut next_beams, &mut matrix);
        }
        beams = next_beams;
    }

    // print energy matrices
    {
        let energy_matrix = Matrix::from_iterator(
            matrix.nrows(),
            matrix.ncols(),
            matrix.iter().map(|cell| {
                if cell.energy.len() > 0 {
                    '#'
                } else {
                    (&cell.symbol).into()
                }
            }),
        );
        println!("{energy_matrix}");
    }

    // count cells with at least one energy
    matrix
        .iter()
        .filter_map(|cell| (cell.energy.len() > 0).then_some(1))
        .sum::<usize>()
}

fn step(mut beam: Beam, beams: &mut Vec<Beam>, matrix: &mut Matrix<Cell>) {
    match beam.step(matrix) {
        BeamStep::Moved | BeamStep::Reflected => {
            keep_alive_when_first_energy_pass(beam, beams, matrix)
        }
        BeamStep::Split(split_beams) => {
            for split_beam in split_beams {
                keep_alive_when_first_energy_pass(split_beam, beams, matrix);
            }
        }
        BeamStep::Faded => { /* faded – nothing to do */ }
    }
}

fn keep_alive_when_first_energy_pass(beam: Beam, beams: &mut Vec<Beam>, matrix: &mut Matrix<Cell>) {
    if let Some(cell) = matrix.get_mut_element_signed(beam.pos.y, beam.pos.x) {
        if !cell.energy.contains(&beam.dir) {
            cell.energy.insert(beam.dir);
            beams.push(beam);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day16_2023_part2() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        assert_eq!(process(input), 51);
    }
}
