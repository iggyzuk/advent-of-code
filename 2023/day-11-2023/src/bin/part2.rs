use std::fmt::Display;
use std::ops::Range;

#[derive(Debug)]
struct Map<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<Option<T>>,
}

impl<T: Clone> Map<T> {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![None; width * height],
        }
    }

    fn add(&mut self, cell: Cell, item: T) {
        let index = (cell.y * self.width as isize + cell.x) as usize;
        self.data[index] = Some(item);
    }

    fn get(&self, cell: Cell) -> Option<&T> {
        // Out of bounds, don't even try to index.
        if cell.x < 0
            || cell.y < 0
            || cell.x >= self.width as isize
            || cell.y >= self.height as isize
        {
            return None;
        }
        // Flatten 2D coordinates into a 1D array – row-major order.
        let index = (cell.y * self.width as isize + cell.x) as usize;
        if let Some(item) = self.data.get(index) {
            item.as_ref()
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Cell {
    x: isize,
    y: isize,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Cell {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Cell {
    type Output = Cell;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Cell {
    type Output = Cell;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct Node {
    cell: Cell,
    thing: Thing,
}
impl Node {
    fn new(cell: Cell, thing: Thing) -> Self {
        Self { cell, thing }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum Thing {
    Space,
    Galaxy,
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input, 1_000_000);
    println!("{output}");
}

// 613_686_987_427
fn process(input: &str, galaxy_expansion: usize) -> usize {
    let map = parse(input);

    // println!("{map:?}");

    let galaxies = galaxies_in_expand_space(map, galaxy_expansion);

    println!();
    println!("------------- galaxies ({}) -------------", galaxies.len());
    println!();

    galaxies.iter().for_each(|p| println!("Galaxy: {p}"));

    println!();
    println!("------------- sum -------------");
    println!();

    // The outer loop iterates over each element in the list, and the inner loop iterates over the remaining elements in the list starting from the next element after the current one. This ensures that you don't get duplicate pairs, and the order of elements in the pair doesn't matter.
    let galaxies_paths = galaxies
        .iter()
        .enumerate()
        .map(|(index, a)| {
            let sum = galaxies
                .iter()
                .skip(index + 1)
                .map(|b| distance(*a, *b))
                .sum::<usize>();

            println!("Path Sum: {a} -> {sum:?}");

            sum
        })
        .sum::<usize>();

    galaxies_paths
}

fn parse(input: &str) -> Map<Node> {
    let mut nodes = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let cell = Cell::new(x as isize, y as isize);
            let node = match char {
                '.' => Node::new(cell, Thing::Space),
                '#' => Node::new(cell, Thing::Galaxy),
                _ => panic!("could not parse '{char}'"),
            };
            nodes.push(node);
        }
    }

    let max = nodes
        .iter()
        .map(|node| node.cell + Cell::new(1, 1))
        .max()
        .expect("should be a max value");

    println!("Map Size: {}", max);

    let mut map = Map::new(max.x as usize, max.y as usize);
    for node in nodes {
        map.add(node.cell, node);
    }

    map
}

/// Any rows or columns that contain no galaxies should all actually be a million times bigger.
fn galaxies_in_expand_space(map: Map<Node>, galaxy_expansion: usize) -> Vec<Cell> {
    let mut expand_rows = vec![];
    for y in 0..map.height as usize {
        if is_all_space(&map, 0..map.width as usize, y..y + 1) {
            expand_rows.push(y);
        }
    }
    let mut expand_cols = vec![];
    for x in 0..map.width as usize {
        if is_all_space(&map, x..x + 1, 0..map.height as usize) {
            expand_cols.push(x);
        }
    }

    let mut galaxies = vec![];

    // 10 times needs to turn into x9, while 1 needs to stay 1
    let step = usize::max(galaxy_expansion - 1, 1);

    let mut ex = 0;
    let mut ey = 0;

    for y in 0..map.height {
        if expand_rows.contains(&y) {
            ey += step;
        }
        for x in 0..map.width {
            if expand_cols.contains(&x) {
                ex += step;
            }

            let cell = Cell::new(x as isize, y as isize);

            if let Some(node) = map.get(cell) {
                if node.thing == Thing::Galaxy {
                    let galaxy = node.cell + Cell::new(ex as isize, ey as isize);
                    galaxies.push(galaxy);
                }
            }
        }
        ex = 0;
    }

    galaxies
}

fn is_all_space(map: &Map<Node>, h_range: Range<usize>, v_range: Range<usize>) -> bool {
    for v in v_range {
        for h in h_range.clone() {
            if let Some(node) = map.get(Cell::new(h as isize, v as isize)) {
                if node.thing == Thing::Galaxy {
                    return false;
                }
            }
        }
    }
    true
}

fn distance(from: Cell, to: Cell) -> usize {
    ((from.x - to.x).abs() + (from.y - to.y).abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day11_2023_part2() {
        let input = "#....
.....
....#";
        assert_eq!(process(input, 1), 10);

        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(process(input, 1), 374);
        assert_eq!(process(input, 10), 1030);
        assert_eq!(process(input, 100), 8410);
    }

    #[test]
    fn day11_2023_distance() {
        let d = distance(Cell::new(0, 0), Cell::new(-5, 5));
        assert_eq!(d, 10);
        let d = distance(Cell::new(0, 0), Cell::new(10, 1));
        assert_eq!(d, 11);
        let d = distance(Cell::new(2, 2), Cell::new(4, 4));
        assert_eq!(d, 4);
        let d = distance(Cell::new(-100, -100), Cell::new(100, 100));
        assert_eq!(d, 400);
    }
}
