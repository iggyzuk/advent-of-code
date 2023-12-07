use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u64,     // time for race
    distance: u64, // best record
}

impl Race {
    fn count_record_beats(&self) -> u64 {
        let mut beats = 0;
        for charge_time in 0..self.time {
            let dist = self.get_dist_for_charge_time(charge_time);
            if dist > self.distance {
                beats += 1;
            }
        }
        beats
    }

    fn get_dist_for_charge_time(&self, charge_time: u64) -> u64 {
        let speed = charge_time;
        let time = self.time - charge_time;
        speed * time
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// determine the number of ways you can beat the record in one race
// answer: 36530883
fn process(input: &str) -> u64 {
    let output = parse(input);
    let race = output.unwrap().1;
    race.count_record_beats()
}

fn parse(input: &str) -> IResult<&str, Race> {
    use nom::character::complete::u32;

    // Time: 1 2 3
    let time_tag = preceded(tag("Time:"), multispace1);
    let time_vec = separated_list1(multispace1, u32);
    let (input, times) = preceded(time_tag, time_vec)(input)?;

    // \n
    let (input, _) = newline(input)?;

    // Distance: 1 2 3
    let dist_tag = preceded(tag("Distance:"), multispace1);
    let dist_vec = separated_list1(multispace1, u32);
    let (input, dists) = preceded(dist_tag, dist_vec)(input)?;

    // concat values [7, 15, 30] into "71530" and then parse it into a number: 71530
    let concat = |v: Vec<u32>| {
        v.into_iter()
            .map(|num| num.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    };

    let time = concat(times);
    let distance = concat(dists);
    let race = Race { time, distance };

    Ok((input, race))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input), 71503);
    }
}
