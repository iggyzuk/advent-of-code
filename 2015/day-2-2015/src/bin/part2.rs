use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1, sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

#[derive(Debug)]
struct Cube {
    length: u32,
    width: u32,
    height: u32,
}

impl Cube {
    fn get_ribbon_feet(&self) -> u32 {
        let mut sides = vec![self.length, self.width, self.height];

        sides.sort();

        let ribbon_for_present = sides.iter().take(2).map(|x| x * 2).sum::<u32>();
        let ribbon_for_bow = sides.iter().product::<u32>();

        ribbon_for_present + ribbon_for_bow
    }
}

// 3737498
fn process(input: &str) -> u32 {
    let cubes = parse(input).unwrap().1;
    dbg!(&cubes);
    cubes
        .into_iter()
        .map(|c| c.get_ribbon_feet())
        .sum()
}

fn parse(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(newline, parse_cube)(input)
}

/// 29x13x26
fn parse_cube(input: &str) -> IResult<&str, Cube> {
    use nom::character::complete::u32;

    let (input, (length, _, width, _, height)) = tuple((u32, tag("x"), u32, tag("x"), u32))(input)?;

    Ok((
        input,
        Cube {
            length,
            width,
            height,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day2_2015_part2() {
        assert_eq!(process("2x3x4"), 34);
        assert_eq!(process("1x1x10"), 14);
    }
}
