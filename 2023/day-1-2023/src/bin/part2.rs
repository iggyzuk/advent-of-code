use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(input);
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let letters = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // process lines
    let mut total = 0;

    for line in input.lines() {
        let mut map = HashMap::new();

        // find letters
        for (idx, letter) in letters.iter().enumerate() {
            for (letter_idx, _) in line.match_indices(letter) {
                map.insert(letter_idx, (idx + 1) as i32);
            }
        }

        // find numbers
        for (idx, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                map.insert(idx, char.to_digit(10).unwrap() as i32);
            }
        }

        // sort the keys
        let mut sorted_keys: Vec<_> = map.keys().collect();
        sorted_keys.sort();

        // key sorted items
        let mut sorted_numbers = vec![];
        for key in sorted_keys {
            if let Some(num) = &map.get(key) {
                sorted_numbers.push(**num);
            }
        }

        // dbg!(&sorted_numbers);

        let a = sorted_numbers[0];
        let b = sorted_numbers[sorted_numbers.len() - 1];

        let number: i32 = format!("{a}{b}").parse().expect("has to be a number");

        println!("{line} -> {a}, {b} = ({number})");

        total += number;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_part2() {
        assert_eq!(
            process(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        );
    }

    #[test]
    fn day1_part2_double(){
        assert_eq!(process("vtrbqpv9sevenone1qlvmzkthnnsevenseven"), 97);
    }
}
