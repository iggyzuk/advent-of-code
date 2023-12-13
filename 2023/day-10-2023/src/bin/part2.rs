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

impl NodeKind {
    fn symbol(&self, bold: bool) -> char {
        match self {
            NodeKind::Start => '!', // ●
            NodeKind::Ground => '░',
            NodeKind::Pipe(pipe) => pipe.symbol(bold),
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

    fn symbol(&self, bold: bool) -> char {
        if bold {
            match self {
                Pipe::Vertical => '┃',
                Pipe::Horizontal => '━',
                Pipe::NorthEast => '┗',
                Pipe::NorthWest => '┛',
                Pipe::SouthWest => '┓',
                Pipe::SouthEast => '┏',
            }
        } else {
            match self {
                Pipe::Vertical => '│',
                Pipe::Horizontal => '─',
                Pipe::NorthEast => '└',
                Pipe::NorthWest => '┘',
                Pipe::SouthWest => '┐',
                Pipe::SouthEast => '┌',
            }
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

#[derive(PartialEq)]
enum Collision {
    Start,
    Vertical,
    Horizontal,
    Space,
}

// 281
fn process(input: &str) -> u32 {
    // parsed-map
    let map = parse(input);

    // floor-map
    let flood_fill_map = map.flood_fill_from_start();

    // collision-map
    let mut collision_map = HashMap::new();
    for (cell, _) in flood_fill_map {
        let node = map
            .get(cell)
            .expect("node should exist for cells in flood map");

        let collision = match &node.kind {
            NodeKind::Start => Collision::Start,
            NodeKind::Pipe(pipe) => match pipe {
                Pipe::Vertical | Pipe::SouthEast | Pipe::SouthWest => Collision::Vertical,
                _ => Collision::Horizontal,
            },
            _ => Collision::Space,
        };

        collision_map.insert(cell, collision);
    }

    count_inside_nodes(map, collision_map)
}

fn count_inside_nodes(map: Map, collision_map: HashMap<Cell, Collision>) -> u32 {
    let rows = map.nodes.iter().filter(|item| item.0.x == 0).count() as usize;
    let cols = map.nodes.iter().filter(|item| item.0.y == 0).count() as usize;

    println!("map size: (rows: {}, columns: {})", rows, cols);

    let mut total_inside = 0;

    for y in 0..rows {
        let mut intersections = 0;
        let mut prev_inside = false;

        for x in 0..cols {
            let cell = Cell::new(x as i32, y as i32);

            if let Some(cursor_node) = map.get(cell) {
                // ░░░░░░░░░░░ ░░░░░░░░░░░
                // ░!━━━━━━━┓░ ░X━━━━━━━X░
                // ░┃┏━━━━━┓┃░ ░XX━━━━━XX░
                // ░┃┃░░░░░┃┃░ ░XX░░░░░XX░
                // ░┃┃░░░░░┃┃░ ░XX░░░░░XX░
                // ░┃┗━┓░┏━┛┃░ ░X┗━X░X━┛X░
                // ░┃╳╳┃░┃╳╳┃░ ░X░░X░X░░X░
                // ░┗━━┛░┗━━┛░ ░┗━━┛░┗━━┛░
                // ░░░░░░░░░░░ ░░░░░░░░░░░

                let mut curr_inside = intersections % 2 != 0;

                if let Some(collision) = collision_map.get(&cell) {
                    match collision {
                        Collision::Start => {
                            // Collision with type start can be in place of F, 7, |, which is considered a vertical collision, so we must increase intersections if that's the case.
                            // Arguably, this should be pre-processed before getting here, but it works ok for now.
                            let check_shape = |a, b| {
                                map.get(cursor_node.cell + a)
                                    .map_or(false, |n| cursor_node.can_connect(n))
                                    && map
                                        .get(cursor_node.cell + b)
                                        .map_or(false, |n| cursor_node.can_connect(n))
                            };

                            if check_shape(Cell::new(0, 1), Cell::new(0, -1))
                                || check_shape(Cell::new(0, 1), Cell::new(1, 0))
                                || check_shape(Cell::new(0, 1), Cell::new(-1, 0))
                            {
                                intersections += 1;
                            }

                            // start is never inside
                            curr_inside = false;
                        }
                        Collision::Vertical => {
                            intersections += 1;
                            curr_inside = intersections % 2 != 0;
                        }
                        Collision::Horizontal => curr_inside = false,
                        Collision::Space => {}
                    }
                }

                let is_inside = prev_inside && curr_inside;

                if is_inside {
                    total_inside += 1;
                }

                prev_inside = intersections % 2 != 0;

                // print inside nodes as "X"
                print!(
                    "{}",
                    if is_inside {
                        '╳'
                    } else {
                        cursor_node.kind.symbol(collision_map.get(&cell).is_some())
                    }
                );
            }
        }

        // new line
        println!();
    }

    total_inside
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
    fn day10_2023_part2() {
        let input = "...|F--7...
.FS-J--L-7.
J|F-----7|7
-|L-7F-7||L
FJ|.|L7|||F
||J-|.|LJ||
L7--|.|..|J
JL--J.L--J.
.|.--|-...";
        assert_eq!(process(input), 11);

        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(process(input), 4);

        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(process(input), 4);

        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(process(input), 8);

        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(process(input), 10);
    }
}
