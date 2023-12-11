use itertools::Itertools;
use nom::{
    character::complete::{newline, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 1798691765
fn process(input: &str) -> i32 {
    let (_, sequences) = parse(input).unwrap();

    sequences
        .iter()
        .map(|seq| predict_next_element(seq))
        .sum::<i32>()
}

/// # Example:
/// ```rust
/// 1   3   6  10  15  21 //<- seq (original)
///   2   3   4   5   6   //<- deltas
///     1   1   1   1
///       0   0   0
/// ```
fn predict_next_element(seq: &Vec<i32>) -> i32 {
    if seq.iter().all(|x| *x == 0) {
        return 0;
    }

    let last = seq
        .last()
        .expect("should have at least one element in starting sequence");

    let differences = seq.windows(2).map(|x| x[1] - x[0]).collect_vec();

    last + predict_next_element(&differences)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    use nom::character::complete::i32;
    separated_list1(newline, separated_list1(space1, i32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day9_2023_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(process(input), 114);
    }
}
