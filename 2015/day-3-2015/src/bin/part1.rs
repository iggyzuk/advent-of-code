use std::collections::BTreeSet;

#[derive(Debug)]
enum Move {
    North,
    East,
    South,
    West,
}

impl From<Move> for Cell {
    fn from(value: Move) -> Self {
        match value {
            Move::North => Cell { x: 0, y: 1 },
            Move::East => Cell { x: 1, y: 0 },
            Move::South => Cell { x: 0, y: -1 },
            Move::West => Cell { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign for Cell {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug)]
struct Santa {
    map: BTreeSet<Cell>,
    cell: Cell,
}

impl Santa {
    fn do_move(&mut self, mv: Move) {
        self.cell += mv.into();
        self.map.insert(self.cell.clone());
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 2565
fn process(input: &str) -> u32 {
    let moves = parse(input);

    dbg!(&moves);

    let mut santa = Santa {
        map: BTreeSet::new(),
        cell: Cell { x: 0, y: 0 },
    };

    for mv in moves {
        santa.do_move(mv);
    }

    santa.map.len() as u32
}

fn parse(input: &str) -> Vec<Move> {
    input
        .chars()
        .map(|c| match c {
            '^' => Move::North,
            '>' => Move::East,
            'v' => Move::South,
            '<' => Move::West,
            _ => Move::North,
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day3_2015_part1() {
        assert_eq!(process("^v^v^v^v^v"), 2);
    }
}
