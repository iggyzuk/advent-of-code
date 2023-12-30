// https://www.youtube.com/watch?v=bGWK76_e-LM

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    steps: usize,
    color: String,
}

impl Instruction {
    fn from(color: String) -> Self {
        let dir = match color.chars().last().expect("should have a last char") {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("could not parse dir from last char in color"),
        };

        let step_hex = color.chars().take(5).collect::<String>();
        let steps =
            usize::from_str_radix(step_hex.as_str(), 16).expect("should parse steps from hex");

        Self { dir, steps, color }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    // Row major.
    fn coords(&self) -> (isize, isize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
        }
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

// 72_811_019_847_283
fn process(input: &str) -> usize {
    let (_, instructions) = parsing::parse(input).unwrap();

    let mut points = vec![(0, 0)];
    let mut boundary = 0;

    for ins in instructions {
        let steps = ins.steps as isize;
        let prev = points.last().expect("must have atleast one");
        let curr = ins.dir.coords();
        let next = (prev.0 + curr.0 * steps, prev.1 + curr.1 * steps);
        points.push(next);

        boundary += ins.steps;
    }

    println!("{points:?}");
    println!("{boundary:?}");

    // https://en.wikipedia.org/wiki/Shoelace_formula
    // A = 1/2 * Σ{i=1,n} => yi * (xi-1 - xi+1)
    let area = (points
        .iter()
        .cycle()
        .take(points.len() + 1)
        .collect::<Vec<_>>()
        .windows(3)
        .inspect(|p| println!("{p:?}"))
        .map(|p| p[1].0 * (p[0].1 - p[2].1))
        .sum::<isize>()
        .abs()
        / 2) as usize;

    println!("area: {area}");

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    // A = i + b/2 - 1
    // i = A - b/2 + 1 (reorder to find interior points)
    let interior = area - boundary / 2 + 1;

    let total = interior + boundary;
    total
}

mod parsing {

    use crate::{Direction, Instruction};

    use nom::{
        character::complete::{alphanumeric1, anychar, newline, space1},
        combinator::map,
        multi::separated_list1,
        sequence::{delimited, preceded, terminated, tuple},
        IResult,
    };

    pub(crate) fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(newline, parse_instruction)(input)
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        use nom::character::complete::u64;
        map(
            tuple((
                terminated(parse_direction, space1),
                terminated(map(u64, |n| n as usize), space1),
                parse_color,
            )),
            |(_, _, color)| Instruction::from(color),
        )(input)
    }

    fn parse_direction(input: &str) -> IResult<&str, Direction> {
        map(anychar, |c| match c {
            'R' => Direction::Right,
            'U' => Direction::Up,
            'L' => Direction::Left,
            'D' => Direction::Down,
            _ => panic!("can't parse {c} to direction"),
        })(input)
    }

    fn parse_color(input: &str) -> IResult<&str, String> {
        use nom::character::complete::char;
        delimited(
            char('('),
            preceded(char('#'), map(alphanumeric1, |a: &str| a.to_owned())),
            char(')'),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day18_2023_part2() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(process(input), 952408144115);
    }
}
