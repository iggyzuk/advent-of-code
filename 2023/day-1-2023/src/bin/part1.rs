fn main() {
    let input = include_str!("../../input.txt");
    dbg!(input);
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let mut line_numbers = vec![];

    for line in input.lines().into_iter() {
        let numbers: Vec<_> = line.chars().filter(|x| x.is_numeric()).collect();

        let a = numbers[0];
        let b = numbers[numbers.len() - 1];

        let number: i32 = format!("{a}{b}").parse().expect("has to be a number");
        line_numbers.push(number);

        println!("{a}, {b} = {number}");
    }
    line_numbers.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_part1() {
        assert_eq!(
            process(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            ),
            142
        );
    }
}
