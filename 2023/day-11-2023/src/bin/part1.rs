use std::fmt::Display;
use std::ops::Range;

#[derive(Debug)]
struct Map<T: Clone> {
    width: u32,
    height: u32,
    data: Vec<Option<T>>,
}

impl<T: Clone> Map<T> {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![None; (width * height) as usize],
        }
    }

    fn add(&mut self, cell: Cell, item: T) {
        let index = (cell.y * self.width as i32 + cell.x) as usize;
        self.data[index] = Some(item);
    }

    fn get(&self, cell: Cell) -> Option<&T> {
        let index = (cell.y * self.width as i32 + cell.x) as usize;
        if let Some(item) = self.data.get(index) {
            item.as_ref()
        } else {
            None
        }
    }
}

impl Map<Node> {
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = Cell::new(x as i32, y as i32);
                if let Some(node) = self.get(cell) {
                    if node.thing == Thing::Galaxy {
                        print!("#");
                    } else {
                        print!(".");
                    }
                } else {
                    print!("?");
                }
            }
            println!();
        }
    }
    fn flood(&mut self, thing: Thing) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = Cell::new(x as i32, y as i32);
                if self.get(cell).is_none() {
                    self.add(cell, Node::new(cell, thing.clone()));
                }
            }
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Cell {
    x: i32,
    y: i32,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Cell {
    fn new(x: i32, y: i32) -> Self {
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
    let output = process(input);
    println!("{output}");
}

// 9214785
fn process(input: &str) -> usize {
    let mut map = parse(input);

    // println!("{map:?}");

    println!("------------- expanding space -------------");
    map = expand_space(map);

    let galaxies = find_galaxies(&map);

    println!();
    println!("------------- galaxies ({}) -------------", galaxies.len());
    println!();

    galaxies
        .iter()
        .map(|n| n.cell)
        .for_each(|p| println!("Galaxy: {p}"));

    println!();
    println!("------------- sum -------------");
    println!();

    // The outer loop iterates over each element in the list, and the inner loop iterates over the remaining elements in the list starting from the next element after the current one. This ensures that you don't get duplicate pairs, and the order of elements in the pair doesn't matter.
    let galaxies_paths = galaxies
        .iter()
        .map(|n| n.cell)
        .enumerate()
        .map(|(index, a)| {
            let sum = galaxies
                .iter()
                .skip(index + 1)
                .map(|n| n.cell)
                .map(|b| line(a, b).len())
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
            let cell = Cell::new(x as i32, y as i32);
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

    let mut map = Map::new(max.x as u32, max.y as u32);
    for node in nodes {
        map.add(node.cell, node);
    }

    map
}

/// Any rows or columns that contain no galaxies should all actually be twice as big.
fn expand_space(map: Map<Node>) -> Map<Node> {
    let mut expand_cols = vec![];
    let mut expand_rows = vec![];

    for y in 0..map.height {
        if is_all_space(&map, 0..map.width, y..y + 1) {
            expand_rows.push(y);
        }
    }
    for x in 0..map.width {
        if is_all_space(&map, x..x + 1, 0..map.height) {
            expand_cols.push(x);
        }
    }

    let expand_width = expand_cols.len() as u32;
    let expand_height = expand_rows.len() as u32;

    let map_expand_width = map.width + expand_width;
    let map_expand_height = map.height + expand_height;

    let mut expand_map = Map::new(map_expand_width, map_expand_height);

    let mut ex = 0;
    let mut ey = 0;

    for y in 0..map.height {
        if expand_rows.contains(&y) {
            ey += 1;
        }
        for x in 0..map.width {
            if expand_cols.contains(&x) {
                ex += 1;
            }
            let cell = Cell::new(x as i32, y as i32);

            if let Some(node) = map.get(cell) {
                if node.thing == Thing::Galaxy {
                    let mut node = node.clone();
                    node.cell = cell + Cell::new(ex, ey);
                    expand_map.add(node.cell, node)
                }
            }
        }
        ex = 0;
    }

    // flood all of the empty space with 'Space'
    expand_map.flood(Thing::Space);

    expand_map.print();

    expand_map
}

fn is_all_space(map: &Map<Node>, x_range: Range<u32>, y_range: Range<u32>) -> bool {
    for y in y_range {
        for x in x_range.clone() {
            if let Some(node) = map.get(Cell::new(x as i32, y as i32)) {
                if node.thing != Thing::Space {
                    return false;
                }
            }
        }
    }
    true
}

fn find_galaxies(map: &Map<Node>) -> Vec<&Node> {
    map.data
        .iter()
        .filter_map(|node| {
            if let Some(node) = node {
                if node.thing == Thing::Galaxy {
                    return Some(node);
                }
            };
            None
        })
        .collect::<Vec<_>>()
}

fn line(from: Cell, to: Cell) -> Vec<Cell> {
    let mut points = Vec::new();
    let mut plot_callback = |x, y| {
        points.push(Cell::new(x, y));
    };

    plot_line(from.x, from.y, to.x, to.y, &mut plot_callback);

    // --- line print ---
    // println!("line: from: {} to: {}, len: {}", from, to, points.len());
    // for p in &points {
    //     print!("{p}|");
    // }
    // println!();
    // --- end ---

    points
}

// Note: Line algorithm is unnecessary for this challenge it's just fun to learn about it. To solve the challenge all you need to do is get the sum delta of x and y e.g. (6-4=2)+(2-1=1)=3.
// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
// https://stackoverflow.com/questions/8936183/bresenham-lines-w-o-diagonal-movement
fn plot_line<F>(x0: i32, y0: i32, x1: i32, y1: i32, mut plot: F)
where
    F: FnMut(i32, i32),
{
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    // Should you wish to keep the starting point.
    // plot(x0, y0);

    while x != x1 || y != y1 {
        if 2 * err - dy > dx - 2 * err {
            // horizontal step
            err += dy;
            x += sx;
        } else {
            // vertical step
            err += dx;
            y += sy;
        }

        plot(x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day11_2023_part1() {
        let input = "#....
.....
....#";
        assert_eq!(process(input), 10);

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

        assert_eq!(process(input), 374);
    }

    #[test]
    fn day11_2023_lines() {
        let points = line(Cell::new(0, 0), Cell::new(-5, 5));
        assert_eq!(points.len(), 10);
    }
}
