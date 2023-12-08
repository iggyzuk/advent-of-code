use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace1, newline},
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
    id: String,
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 14257
fn process(input: &str) -> u32 {
    let (_, data) = parse(input).unwrap();
    dbg!(&data);

    // create a map from id -> node for quick access
    let map = data
        .nodes
        .iter()
        .map(|x| (x.id.clone(), x))
        .collect::<BTreeMap<String, &Node>>();

    let mut steps = 0;
    let mut node = map
        .get("AAA")
        .expect("must have a starting node called AAA");

    for instruction in data.instructions.iter().cycle() {
        steps += 1;

        let node_id = match instruction {
            Instruction::Left => &node.left,
            Instruction::Right => &node.right,
        };

        node = map.get(node_id).expect("should have the node");

        if node.id == "ZZZ" {
            break;
        }
    }

    steps
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

    let (input, id) = alpha1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (left, right)) = delimited(
        char('('),
        separated_pair(alpha1, tag(", "), alpha1),
        char(')'),
    )(input)?;

    Ok((
        input,
        Node {
            id: id.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_2023_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(process(input), 2);
    }
}
