fn main() {
    println!("Starting Process");
    let now = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("Finished in {:?}", now.elapsed());
    println!("Solution: {:?}", output);
}

// 501_680
fn process(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn hash(input: &str) -> usize {
    let mut result: usize = 0;
    for item in input.chars() {
        result += item as usize;
        result *= 17;
        result %= 256;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day15_2023_part1() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(
            process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }
}