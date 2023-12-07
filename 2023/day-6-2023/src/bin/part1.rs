use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: u32,     // time for race
    distance: u32, // best record
}

impl Race {
    fn count_record_beats(&self) -> u32 {
        let mut beats = 0;
        for charge_time in 0..self.time {
            let dist = self.get_dist_for_charge_time(charge_time);
            if dist > self.distance {
                beats += 1;
            }
        }
        beats
    }

    fn get_dist_for_charge_time(&self, charge_time: u32) -> u32 {
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

// determine the number of ways you can beat the record in each race
// answer: 512295
fn process(input: &str) -> u32 {
    let output = parse(input);
    let races = output.unwrap().1;

    dbg!(&races);

    races
        .into_iter()
        .fold(1, |acc, race| acc * race.count_record_beats())
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
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

    let races = times
        .into_iter()
        .zip(dists.into_iter())
        .map(|x| Race {
            time: x.0,
            distance: x.1,
        })
        .collect::<Vec<_>>();

    Ok((input, races))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day1_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input), 288); // (4 * 8 * 9)
    }
}
