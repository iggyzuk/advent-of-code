use std::{collections::BTreeMap, ops::Range};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, newline, space1},
    combinator::recognize,
    multi::{many1, separated_list1},
    sequence::{delimited, pair},
    IResult,
};

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
        let mut named_maps = BTreeMap::new();
        for map in self.maps.iter() {
            named_maps.insert(map.name.to_owned(), map);
        }

        let mut locations = vec![];
        for seed in &self.seeds {
            let soil = named_maps.get("seed-to-soil").unwrap().src_to_dst(*seed);
            let fertilizer = named_maps
                .get("soil-to-fertilizer")
                .unwrap()
                .src_to_dst(soil);
            let water = named_maps
                .get("fertilizer-to-water")
                .unwrap()
                .src_to_dst(fertilizer);
            let light = named_maps.get("water-to-light").unwrap().src_to_dst(water);
            let temperature = named_maps
                .get("light-to-temperature")
                .unwrap()
                .src_to_dst(light);
            let humidity = named_maps
                .get("temperature-to-humidity")
                .unwrap()
                .src_to_dst(temperature);
            let location = named_maps
                .get("humidity-to-location")
                .unwrap()
                .src_to_dst(humidity);

            println!("seed: {seed}, soil: {soil}, fertilizer: {fertilizer}, water: {water}, light: {light}, temperature: {temperature}, humidity: {humidity}, location: {location}");

            locations.push(location);
        }

        *locations.iter().min().unwrap()
    }
}

impl Map {
    fn src_to_dst(&self, seed: u32) -> u32 {
        for detail in &self.details {
            if let Some(dst) = detail.src_to_dst(seed) {
                return dst;
            }
        }
        seed
    }
}

impl Details {
    fn src_to_dst(&self, value: u32) -> Option<u32> {
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
    fn day5_part1() {
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
        assert_eq!(process(input), 35);
    }
}
