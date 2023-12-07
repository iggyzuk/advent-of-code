use nom::{
    character::complete::newline,
    multi::separated_list1,
    sequence::{terminated, tuple},
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
    /// 2*l*w + 2*w*h + 2*h*l
    fn get_wrapping_feet(&self) -> u32 {
        let sides = vec![
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        sides.iter().map(|x| x * 2).sum::<u32>() + sides.iter().min().unwrap()
    }
}

// 1586300
fn process(input: &str) -> u32 {
    let cubes = parse(input).unwrap().1;
    dbg!(&cubes);
    cubes.into_iter().map(|c| c.get_wrapping_feet()).sum()
}

fn parse(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(newline, parse_cube)(input)
}

/// 29x13x26
fn parse_cube(input: &str) -> IResult<&str, Cube> {
    use nom::character::complete::char;
    use nom::character::complete::u32;

    let (input, (length, width, height)) =
        tuple((terminated(u32, char('x')), terminated(u32, char('x')), u32))(input)?;

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
    fn day2_2015_part1() {
        assert_eq!(process("2x3x4"), 58);
    }
}
