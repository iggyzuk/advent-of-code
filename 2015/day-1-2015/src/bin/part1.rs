fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> i32 {
    input
        .chars()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_2015_part1() {
        let input = ")())())";
        assert_eq!(process(input), -3);
    }
}
