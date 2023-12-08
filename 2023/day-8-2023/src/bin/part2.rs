use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
struct Data {
    instructions: Vec<Instruction>,
    nodes: Vec<Node>,
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("could not parse instruction from: {value}"),
        }
    }
}

#[derive(Debug)]
struct Node {
    id: Tag,
    left: Tag,
    right: Tag,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Tag(String);

impl Tag {
    // note: there's no need to create a wrapper over a string as it already contains String::ends_with(...), but I'll keep this as a lesson for the future.
    fn ends(&self, value: char) -> bool {
        self.0.chars().nth(2).unwrap() == value
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 16_187_743_689_077
fn process(input: &str) -> usize {
    let (_, data) = parse(input).unwrap();
    dbg!(&data);

    // create a map from id -> node for quick access
    let map = data
        .nodes
        .iter()
        .map(|x| (x.id.clone(), x))
        .collect::<BTreeMap<Tag, &Node>>();

    // find loop end for all nodes that start with A and end with E
    let loop_ends = map
        .values()
        .filter(|x| x.id.ends('A'))
        .map(|node| {
            // process all instructions for a single instance of a loop
            let mut current_node = *node;
            data.instructions
                .iter()
                .cycle()
                .position(|instruction| {
                    // get next node by matching on instruction
                    let next_node = map
                        .get(match instruction {
                            Instruction::Left => &current_node.left,
                            Instruction::Right => &current_node.right,
                        })
                        .expect("node must exist");
                    // when found a match – return it
                    let done = next_node.id.ends('Z');
                    if !done {
                        // or keep going – next!
                        current_node = next_node;
                    }
                    done
                })
                .map(|x| x + 1)
                .expect("should find an end to the loop")
        })
        .collect::<Vec<usize>>();

    // find least common multiple for all loop ends
    lcm_multiple(&loop_ends)
}

/// LCM (Least Common Multiple) of multiple numbers is the smallest number that is evenly divisible by all numbers in the set.
fn lcm_multiple(numbers: &[usize]) -> usize {
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        result = lcm(numbers[i], result);
    }
    result
}

/// LCM (Least Common Multiple) of two numbers is the smallest number that is evenly divisible by all numbers in the set.
///
/// For example common multiples of 4 and 6 are 12, 24 and 36, but the lowest of those is 12
///
/// `lcm(4, 6) = 12`
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

/// GCD (Greatest Common Divisor) of two or more numbers is the greatest common factor number that divides them, exactly.
///
/// `gcd(8, 12) = 4`
fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, instructions) = terminated(alphanumeric1, multispace1)(input)?;

    let instructions = instructions
        .chars()
        .map(|c| c.into())
        .collect::<Vec<Instruction>>();

    let (input, nodes) = separated_list1(newline, parse_node)(input)?;

    Ok((
        input,
        Data {
            instructions,
            nodes,
        },
    ))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    use nom::character::complete::char;

    let (input, id) = alphanumeric1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (left, right)) = delimited(
        char('('),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        char(')'),
    )(input)?;

    Ok((
        input,
        Node {
            id: id.into(),
            left: left.into(),
            right: right.into(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_2023_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(lcm(4, 6), 12);
        assert_eq!(gcd(8, 12), 4);

        assert_eq!(process(input), 6);
    }
}
