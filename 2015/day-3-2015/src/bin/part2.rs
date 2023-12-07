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
    cell: Cell,
}

impl Santa {
    fn new() -> Self {
        let cell = Cell { x: 0, y: 0 };
        Self { cell }
    }
    fn do_move(&mut self, mv: Move) {
        self.cell += mv.into();
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 2639
fn process(input: &str) -> u32 {
    // parse out the moves
    let moves = parse(input);
    dbg!(&moves);

    // create global map of visited cells
    let mut map = BTreeSet::new();
    map.insert(Cell { x: 0, y: 0 });

    // create 2 santas (real and robo)
    let mut santas = vec![Santa::new(), Santa::new()];
    let mut current_santa_idx = 0;

    for mv in moves {
        // move current santa
        let santa = &mut santas[current_santa_idx];
        santa.do_move(mv);

        // insert new cell
        map.insert(santa.cell.clone());

        // switch to the next santa
        current_santa_idx += 1;
        if current_santa_idx >= santas.len() {
            current_santa_idx = 0
        }
    }

    map.len() as u32
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
    fn day3_2015_part2() {
        assert_eq!(process("^v"), 3);
        assert_eq!(process("^>v<"), 3);
        assert_eq!(process("^v^v^v^v^v"), 11);
    }
}
