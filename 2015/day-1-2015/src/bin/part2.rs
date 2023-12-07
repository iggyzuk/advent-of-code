fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 1795
fn process(input: &str) -> i32 {
    let offsets = input.chars().map(|ch| match ch {
        '(' => 1,
        ')' => -1,
        _ => 0,
    });

    let mut offset = 0;
    for (idx, value) in offsets.enumerate() {
        offset += value;
        if offset <= -1 {
            return idx as i32 + 1;
        }
    }
    offset
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_2015_part2() {
        let input = "()())";
        assert_eq!(process(input), 5);
    }
}
