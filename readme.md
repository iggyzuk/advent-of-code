# Advent of Code 2023

`cargo run -p day-1-2023 --bin part1`
`cargo test`

```rust
fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_2023_part1() {
        let input = "";
        assert_eq!(process(input), 123);
    }
}
```
