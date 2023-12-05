use std::ops::Range;

use indicatif::ProgressBar;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, newline, space1},
    combinator::recognize,
    multi::{many1, separated_list1},
    sequence::{delimited, pair},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug)]
struct Data {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    name: String,
    details: Vec<Details>,
}

#[derive(Debug)]
struct Details {
    source: Range<u32>,
    destination: Range<u32>,
}

impl Data {
    fn best_location_for_all_seeds(&self) -> u32 {
        // construct seed ranges
        let seed_ranges: Vec<_> = self
            .seeds
            .chunks(2)
            .map(|seed| Range {
                start: seed[0],
                end: seed[0] + seed[1],
            })
            .collect();

        // order maps by name
        let maps_order = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];

        let mut maps_iter = self.maps.iter();
        let ordered_maps = maps_order
            .iter()
            .map(|name| {
                maps_iter
                    .find(|map| map.name == *name)
                    .expect(&format!("map '{}' not found", name))
            })
            .collect::<Vec<_>>();

        // count all seeds so we could optimize progress bar with skips
        let progress_seeds: u64 = seed_ranges.len() as u64
            + seed_ranges
                .iter()
                .fold(0, |count, seed_range| count + seed_range.len() as u64);

        // report progress 100 (or less) times over all seeds
        let progress_skips: u64 = if progress_seeds > 100 {
            progress_seeds / 100
        } else {
            progress_seeds
        };

        let pb = ProgressBar::new(progress_seeds);

        // find the mind location using parallel iterator
        let min_location = seed_ranges
            .into_par_iter()
            .flat_map(|range| range.into_iter())
            .map(|seed| {
                if seed as u64 % progress_skips == 0 {
                    pb.inc(progress_skips);
                }
                let x = ordered_maps
                    .iter()
                    .fold(seed, |acc, ord_map| ord_map.remap(acc));
                x
            })
            .min()
            .unwrap();

        min_location
    }
}

impl Map {
    fn remap(&self, seed: u32) -> u32 {
        for detail in &self.details {
            if let Some(dst) = detail.try_remap(seed) {
                return dst;
            }
        }
        seed
    }
}

impl Details {
    fn try_remap(&self, value: u32) -> Option<u32> {
        if self.source.contains(&value) {
            let offset = value - self.source.start;
            return Some(self.destination.start + offset);
        }
        None
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    // answer: 2520479
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let (_, data) = parse_data(input).expect("parsing data should succeed");
    dbg!(&data);
    data.best_location_for_all_seeds()
}

fn parse_data(input: &str) -> IResult<&str, Data> {
    use nom::character::complete::u32;
    let (input, seeds) =
        delimited(tag("seeds: "), separated_list1(space1, u32), multispace1)(input)?;
    let (input, maps) = separated_list1(newline, parse_map)(input)?;

    Ok((input, Data { seeds, maps }))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, _) = multispace0(input)?;
    let (input, map_name) = recognize(many1(alt((alphanumeric1, tag("-")))))(input)?;
    let (input, _) = pair(tag(" map:"), newline)(input)?;
    let (input, details) = separated_list1(newline, parse_details)(input)?;

    Ok((
        input,
        Map {
            name: map_name.to_owned(),
            details,
        },
    ))
}

fn parse_details(input: &str) -> IResult<&str, Details> {
    use nom::character::complete::u32;
    let (input, digits) = separated_list1(space1, u32)(input)?;
    Ok((
        input,
        Details {
            source: Range {
                start: digits[1],
                end: digits[1] + digits[2],
            },
            destination: Range {
                start: digits[0],
                end: digits[0] + digits[2],
            },
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day5_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process(input), 46);
    }
}
