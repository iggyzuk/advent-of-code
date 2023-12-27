use common::{Matrix, Vec2};
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Clone)]
struct Node {
    cell: Vec2<usize>,
    dir: Direction,
    steps: usize,
    cost: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse compare for min-heap.
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We only care about the cost of the node when comparing them,
        // because it's only used for the binary heap.
        self.cost.cmp(&other.cost)
    }
}

impl Node {
    fn new(cell: Vec2<usize>, dir: Direction, steps: usize, cost: usize) -> Self {
        Self {
            cell,
            dir,
            steps,
            cost,
        }
    }

    // Extracts just the state that's necessary to track as we traverse the graph.
    // Arguably, state should be encapsulated in the node.
    fn state(&self) -> State {
        State {
            cell: self.cell,
            dir: self.dir.clone(),
            steps: self.steps,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Self::East => Self::West,
            Self::West => Self::East,
            Self::North => Self::South,
            Self::South => Self::North,
        }
    }
}

impl From<Direction> for Vec2<isize> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::East => Vec2::RIGHT,
            Direction::North => Vec2::UP,
            Direction::West => Vec2::LEFT,
            Direction::South => Vec2::DOWN,
        }
    }
}

impl From<Vec2<isize>> for Direction {
    fn from(value: Vec2<isize>) -> Self {
        match value {
            Vec2::RIGHT => Direction::East,
            Vec2::UP => Direction::North,
            Vec2::LEFT => Direction::West,
            Vec2::DOWN => Direction::South,
            _ => panic!("could not cast '{value}' to direction"),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    cell: Vec2<usize>,
    dir: Direction,
    steps: usize,
}

fn main() {
    println!("Starting Process");
    let now = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("Finished in {:?}", now.elapsed());
    println!("Solution: {:?}", output);
}

// 1260
fn process(input: &str) -> usize {
    // parse input into a matrix
    let matrix = Matrix::from_iterator(
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
        input.lines().flat_map(|line| {
            line.chars()
                .map(move |char| char.to_digit(10).unwrap() as usize)
        }),
    );
    println!("{matrix}");

    let start = Vec2::new(0, 0);
    let end = Vec2::new(matrix.ncols() - 1, matrix.nrows() - 1);

    total_cost_of_shortest_path(start, end, matrix)
}

// Dijkstra algorithm.
// Note: we don't care about the points of the shortest path, just the cost of getting there.
fn total_cost_of_shortest_path(
    start: Vec2<usize>,
    end: Vec2<usize>,
    matrix: Matrix<usize>,
) -> usize {
    // Min costs track node costs, only cell, dir, steps -> ((2,3), (1,0), 2) => 7
    let mut costs: HashMap<State, usize> = HashMap::new();
    let mut frontier: BinaryHeap<Node> = BinaryHeap::new();

    // Set costs to zero in all starting directions,
    // Start the frontier with the two initial directions.
    let node_east = Node::new(start, Direction::East, 0, 0);
    costs.insert(node_east.state(), 0);
    frontier.push(node_east);

    let node_south = Node::new(start, Direction::South, 0, 0);
    costs.insert(node_south.state(), 0);
    frontier.push(node_south);

    // Pop the smallest cost node from the frontier (it's a min-heap)
    while let Some(current) = frontier.pop() {
        if current.cell == end {
            return current.cost;
        }

        for neighbour in get_neighbours(&current, &matrix) {
            let new_cost = current.cost + neighbour.cost;

            // Compare this new total cost to our current min cost (if we have one),
            // If new cost is more than what we already have – don't bother with it.
            if let Some(existing_cost) = costs.get(&neighbour.state()) {
                if new_cost >= *existing_cost {
                    continue;
                }
            }

            // This neighbour was either never visited or its cost is smaller,
            // track the cost and add the node to the frontier.
            let mut new_node = neighbour;
            new_node.cost = new_cost;
            costs.insert(new_node.state(), new_node.cost);
            frontier.push(new_node);
        }
    }

    panic!("could not find the end node");
}

fn get_neighbours(node: &Node, matrix: &Matrix<usize>) -> Vec<Node> {
    let mut neighbours: Vec<Node> = vec![];
    let dirs = vec![Vec2::LEFT, Vec2::RIGHT, Vec2::UP, Vec2::DOWN];
    for dir in dirs {
        // No backtracking – only forward, left, right.
        if node.dir.inverse() == dir.into() {
            continue;
        }

        // Try to get the node in that direction, could be out of bounds.
        let neighbour_cell = Vec2::new(node.cell.x as isize, node.cell.y as isize) + dir;
        if let Some(cost) = matrix.get_element_signed(neighbour_cell.y, neighbour_cell.x) {
            // Generate new neighbours for the next iteration of the frontier.
            if node.dir != dir.into() {
                // Reset direction steps when we change direction.
                let node = Node::new(
                    Vec2::new(neighbour_cell.x as usize, neighbour_cell.y as usize),
                    dir.into(),
                    1,
                    *cost,
                );
                neighbours.push(node);
            } else if node.steps < 3 {
                // We're walking to the same direction, make sure the steps are less than three.
                let node = Node::new(
                    Vec2::new(neighbour_cell.x as usize, neighbour_cell.y as usize),
                    dir.into(),
                    node.steps + 1,
                    *cost,
                );
                neighbours.push(node);
            }
        }
    }
    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day17_2023_part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        assert_eq!(process(input), 102);
    }
}
