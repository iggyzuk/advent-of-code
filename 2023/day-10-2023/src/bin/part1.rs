use std::collections::HashMap;

#[derive(Debug)]
struct Map {
    nodes: HashMap<Cell, Node>, // this could also be a 2d array
}

impl Map {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
    fn add(&mut self, cell: Cell, node: Node) {
        self.nodes.insert(cell, node);
    }

    fn get(&self, cell: Cell) -> Option<&Node> {
        self.nodes.get(&cell)
    }

    /// Returns a list of cardinal neighbours, if they exist and pass conditions e.g. Pipe joins.
    fn neighbours(&self, cell: Cell) -> Vec<&Node> {
        let mut neighbours = vec![];

        let current_node = self.get(cell).expect("should not feed wrong cell here");

        let mut try_add_neighbour = |dir: Cell| {
            let neighbour_node_op = self.get(cell + dir);
            if let Some(neighbour_node) = neighbour_node_op {
                if current_node.can_connect(neighbour_node) {
                    neighbours.push(neighbour_node);
                }
            }
        };

        try_add_neighbour(Cell::new(1, 0));
        try_add_neighbour(Cell::new(-1, 0));
        try_add_neighbour(Cell::new(0, 1));
        try_add_neighbour(Cell::new(0, -1));

        neighbours
    }

    fn flood_fill_from_start(&self) -> HashMap<Cell, u32> {
        let start_cell = self
            .nodes
            .values()
            .find_map(|node| {
                if node.kind == NodeKind::Start {
                    return Some(node.cell);
                }
                None
            })
            .expect("should have an 'S' in dataset");

        let mut flood_map = HashMap::new();
        flood_map.insert(start_cell, 0);

        let mut frontier = vec![start_cell];

        // keep flooding, until all neighbours are smaller
        while frontier.len() > 0 {
            // can be used to visualize progress
            // print_flood_map(&flood_map);

            let current_cell = frontier
                .pop()
                .expect("we just checked that there's more than zero elements in the frontier");

            let current_flood = *flood_map
                .get(&current_cell)
                .expect("current node must exist");

            let neighbours = self.neighbours(current_cell);
            for neighbour in neighbours {
                let flood_entry = flood_map.entry(neighbour.cell);
                use std::collections::hash_map::Entry;
                match flood_entry {
                    Entry::Occupied(mut value) => {
                        if current_flood < *value.get_mut() {
                            *value.get_mut() = current_flood + 1;
                            frontier.push(neighbour.cell);
                        }
                    }
                    Entry::Vacant(_) => {
                        flood_map.insert(neighbour.cell, current_flood + 1);
                        frontier.push(neighbour.cell);
                    }
                }
            }
        }

        flood_map
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Cell {
    x: i32,
    y: i32,
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

#[derive(Hash, PartialEq, Eq, Debug)]
struct Node {
    cell: Cell,
    kind: NodeKind,
}

impl Node {
    fn from_symbol(cell: Cell, symbol: char) -> Node {
        Self {
            cell,
            kind: symbol.into(),
        }
    }

    fn can_connect(&self, other_node: &Node) -> bool {
        self.cell_in_connections(other_node.cell) && other_node.cell_in_connections(self.cell)
    }

    fn cell_in_connections(&self, cell: Cell) -> bool {
        self.connections().iter().any(|conn| *conn == cell)
    }

    fn connections(&self) -> Vec<Cell> {
        match &self.kind {
            NodeKind::Start => {
                vec![
                    self.cell + Cell::new(1, 0),
                    self.cell + Cell::new(-1, 0),
                    self.cell + Cell::new(0, 1),
                    self.cell + Cell::new(0, -1),
                ]
            }
            NodeKind::Ground => {
                vec![]
            }
            NodeKind::Pipe(pipe) => pipe
                .local_connections()
                .into_iter()
                .map(|local_cell| local_cell + self.cell)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum NodeKind {
    Start,
    Ground,
    Pipe(Pipe),
}

impl From<char> for NodeKind {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Ground,
            '|' => Self::Pipe(Pipe::Vertical),
            '-' => Self::Pipe(Pipe::Horizontal),
            'L' => Self::Pipe(Pipe::NorthEast),
            'J' => Self::Pipe(Pipe::NorthWest),
            '7' => Self::Pipe(Pipe::SouthWest),
            'F' => Self::Pipe(Pipe::SouthEast),
            _ => panic!("could not convert {value} to a node kind"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Pipe {
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
}

impl Pipe {
    fn local_connections(&self) -> Vec<Cell> {
        let mut connections = vec![];
        match self {
            Pipe::Vertical => {
                connections.push(Cell::new(0, 1));
                connections.push(Cell::new(0, -1));
            }
            Pipe::Horizontal => {
                connections.push(Cell::new(1, 0));
                connections.push(Cell::new(-1, 0));
            }
            Pipe::NorthEast => {
                connections.push(Cell::new(0, -1));
                connections.push(Cell::new(1, 0));
            }
            Pipe::NorthWest => {
                connections.push(Cell::new(0, -1));
                connections.push(Cell::new(-1, 0));
            }
            Pipe::SouthWest => {
                connections.push(Cell::new(0, 1));
                connections.push(Cell::new(-1, 0));
            }
            Pipe::SouthEast => {
                connections.push(Cell::new(0, 1));
                connections.push(Cell::new(1, 0));
            }
        }
        connections
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 7107
fn process(input: &str) -> u32 {
    let map = parse(input);

    dbg!(&map);

    let flood_fill_map = map.flood_fill_from_start();

    print_flood_map(&flood_fill_map);

    *flood_fill_map.values().max().unwrap()
}

fn print_flood_map(map: &HashMap<Cell, u32>) {
    let rows = map.iter().map(|item| item.0.x).max().unwrap() as usize;
    let cols = map.iter().map(|item| item.0.y).max().unwrap() as usize;

    let mut visual_map = vec![vec![0; cols]; rows];

    for x in 0..cols {
        for y in 0..rows {
            if let Some(item) = map.get(&Cell::new(x as i32, y as i32)) {
                visual_map[y as usize][x as usize] = ((*item as f32) / 10.0) as u32;
            }
        }
    }

    // print visual map
    for row in &visual_map {
        for cell in row {
            print!("{:3} ", cell);
        }
        println!();
    }
}

fn parse(input: &str) -> Map {
    let mut map = Map::new();

    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            let cell = Cell::new(x as i32, y as i32);
            map.add(cell, Node::from_symbol(cell, symbol));
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day10_2023_part1() {
        // simple loop
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(process(input), 4);

        // pipes that you can't connect to at the start
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(process(input), 4);

        // another longer loop
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(process(input), 8);

        // another one
        let input = "F---7
L---7
S---J
L7.-|
LJ---";
        assert_eq!(process(input), 14);
    }
}
