use std::{collections::HashMap, ops::Range};

use nom::{
    character::complete::{digit1, newline, one_of},
    combinator::{map, map_res, opt},
    multi::{many0, many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

/* ----- */
/* Types */
/* ----- */

type Cache = HashMap<(usize, usize), usize>;

#[derive(Clone, Debug)]
struct SpringProblem {
    states: String,
    groups: Vec<usize>,
}

/* ----- */
/* Setup */
/* ----- */

fn main() {
    println!("Starting Process");
    let now = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("Finished in {:?}", now.elapsed());
    println!("Solution: {:?}", output);
}

// 7402
fn process(input: &str) -> usize {
    let (_, problems) = parse(input).expect("should parse");

    /*
    // should you wish to see all results
    println!("{}", problems.len());
    for sp in &problems {
        let result = count_arrangements(&sp.states, &sp.groups, &mut HashMap::new(), 0);
        println!("{result:?}\t{sp:?}");
    }
    */

    problems
        .into_iter()
        .map(|p| count_arrangements(&p.states, &p.groups, &mut HashMap::new(), 0))
        .sum()
}

/* ------- */
/* Parsing */
/* ------- */

fn parse(input: &str) -> IResult<&str, Vec<SpringProblem>> {
    many0(parse_problem)(input)
}

fn parse_problem(input: &str) -> IResult<&str, SpringProblem> {
    use nom::character::complete::char;
    map(
        terminated(
            separated_pair(many1(one_of(".#?")), char(' '), parse_groups),
            opt(newline),
        ),
        |(states, groups)| SpringProblem {
            states: states.into_iter().collect(),
            groups,
        },
    )(input)
}

fn parse_groups(input: &str) -> IResult<&str, Vec<usize>> {
    use nom::character::complete::char;
    separated_list1(char(','), map_res(digit1, |s: &str| s.parse::<usize>()))(input)
}

/* ----- */
/* Logic */
/* ----- */

// https://www.reddit.com/r/adventofcode/comments/18hg99r/2023_day_12_simple_tutorial_with_memoization/
fn count_arrangements(springs: &str, groups: &[usize], cache: &mut Cache, i: usize) -> usize {
    if groups.is_empty() {
        return if i < springs.len() && springs[i..].chars().any(|c| c == '#') {
            // we ran out of groups, but there are more actual springs to fill ('#' not '?')
            0
        } else {
            // we ran out of groups and we used up all of the springs
            1
        };
    }

    // find the next '#' or '?'
    let mut i = i;
    while i < springs.len() {
        match springs.chars().nth(i).unwrap() {
            '#' | '?' => break,
            _ => i += 1,
        }
    }

    // we reached the end, didn't find any '#' or '?'
    if i >= springs.len() {
        return 0;
    }

    // this (index, group) is already the cache of states, no need to recalculate
    if let Some(result) = cache.get(&(i, groups.len())) {
        return *result;
    }

    let mut result = 0;

    // if current group size fits nicely in this location – try to fit next groups recursively
    let groupsize_end_index = i + groups[0];
    if can_fit(springs, i..groupsize_end_index) {
        result += count_arrangements(springs, &groups[1..], cache, groupsize_end_index + 1);
    }

    // if the current spot is '?', recursively call with current groups at the next index to try all permutations
    if springs.chars().nth(i).unwrap() == '?' {
        result += count_arrangements(springs, groups, cache, i + 1);
    }

    cache.insert((i, groups.len()), result);

    result
}

fn can_fit(springs: &str, range: Range<usize>) -> bool {
    let all_chars = springs.chars().collect::<Vec<_>>();
    // make sure the range's end fits into the springs string
    //  XXXXX---] – good
    //  XXX]X !   – bad
    if range.end > springs.len() {
        return false;
    }
    // make sure that all chars in range are either a '?' or '#' – not '.'
    if springs[range.clone()].chars().any(|x| x == '.') {
        return false;
    }
    // make sure the next char is one of: { out_of_bounds, '.', '?' } – not '#'
    let next = range.end;
    if next < springs.len() && all_chars[next] == '#' {
        return false;
    }

    true
}

/* ------- */
/* Testing */
/* ------- */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day12_2023_part1() {
        assert_eq!(process("???.### 1,1,3"), 1);
        assert_eq!(process(".??..??...?##. 1,1,3"), 4);
        assert_eq!(process("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(process("????.#...#... 4,1,1"), 1);
        assert_eq!(process("????.######..#####. 1,6,5"), 4);
        assert_eq!(process("?###???????? 3,2,1"), 10);
        assert_eq!(process("?.? 1,1"), 1);
        assert_eq!(process("? 1"), 1);
        assert_eq!(process("... 1,1,1"), 0);
        assert_eq!(process(".......?.....#? 1,2"), 1);
        assert_eq!(process("????? 1"), 5);
        assert_eq!(process("????? 1,1"), 6);
        assert_eq!(process("??????? 1,5"), 1);
        assert_eq!(process("????? 1,2"), 3);
        assert_eq!(process("??????#??#??#?? 1,2,9"), 4);

        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(process(input), 21);
    }
}
