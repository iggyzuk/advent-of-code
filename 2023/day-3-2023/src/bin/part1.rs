use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
struct Cell {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum CellKind {
    Empty,
    Digit,
    Symbol,
}

#[derive(PartialEq, Debug)]
struct Digit {
    start: Cell,
    end: Cell,
    total: u32,
}

impl Digit {
    fn new(start: Cell, num: u32) -> Self {
        Self {
            start: start,
            end: start,
            total: num,
        }
    }

    fn add_number(&mut self, num: u32) {
        self.total = format!("{0}{1}", &self.total.to_string(), num)
            .parse()
            .unwrap();
    }

    fn symbol_in_bounds(&self, map: &HashMap<Cell, CellKind>) -> bool {
        let bounds = &self.bounds();
        for cell in bounds {
            if let Some(kind) = map.get(cell) {
                if *kind == CellKind::Symbol {
                    return true;
                }
            }
        }
        false
    }

    fn bounds(&self) -> Vec<Cell> {
        let mut bounds = vec![];
        for x in &self.start.x - 1..=&self.end.x + 1 {
            for y in &self.start.y - 1..=&self.end.y + 1 {
                bounds.push(Cell { x: x, y: y })
            }
        }
        bounds
    }
}

struct DigitBuilder {
    current: Option<Digit>,
    digits: Vec<Digit>,
}

impl DigitBuilder {
    fn new() -> Self {
        Self {
            current: None,
            digits: vec![],
        }
    }

    fn extend(&mut self, cell: Cell, number: u32) {
        if self.current.is_none() {
            self.current = Some(Digit::new(cell, number));
        } else {
            let mut x = self.current.take().unwrap();
            x.end = cell;
            x.add_number(number);
            self.current = Some(x);
        }
    }

    fn complete(&mut self) {
        if self.current.is_some() {
            self.digits.push(self.current.take().unwrap());
        }
    }
}

// get digits
// construct a map to check if it has a neighbour
fn process(input: &str) -> u32 {
    let mut builder = DigitBuilder::new();
    for (y, line) in input.lines().enumerate() {
        builder.complete();
        for (x, char) in line.chars().enumerate() {
            let cell = Cell {
                x: x as i32,
                y: y as i32,
            };

            if char.is_digit(10) {
                builder.extend(cell, char.to_digit(10).unwrap())
            } else {
                builder.complete();
            }
        }
    }

    let mut map: HashMap<Cell, CellKind> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let kind = if char.is_digit(10) {
                CellKind::Digit
            } else if char == '.' {
                CellKind::Empty
            } else {
                CellKind::Symbol
            };

            let cell = Cell {
                x: x as i32,
                y: y as i32,
            };

            map.insert(cell, kind);
        }
    }

    let mut sum = 0;
    for digit in &builder.digits {
        println!("{digit:?}");
        if digit.symbol_in_bounds(&map) {
            sum += digit.total;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day3_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process(input), 4361);
    }
}
