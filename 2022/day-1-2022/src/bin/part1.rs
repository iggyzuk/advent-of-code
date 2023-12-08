use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 72602
fn process(input: &str) -> u32 {
    let elf_calories = parse(input);
    let most_calories = elf_calories
        .into_iter()
        .map(|x| x.iter().sum::<u32>())
        .max()
        .unwrap();
    most_calories
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    // note: here's an approach that uses for loops, but I'm trying to get better at using iterators

    // let mut result = Vec::new();
    // let mut current_calories = Vec::new();
    // for line in input.lines() {
    //     if line == "" {
    //         result.push(current_calories);
    //         current_calories = Vec::new();
    //     } else {
    //         let calories = line.parse::<u32>().expect("should be a number");
    //         current_calories.push(calories);
    //     }
    // }

    let result: Vec<Vec<u32>> = input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(is_empty, _)| !is_empty)
        .map(|(_, group)| {
            group
                .map(|line| line.parse::<u32>().expect("should be a number"))
                .collect()
        })
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_2022_part1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(process(input), 24000);
    }
}
