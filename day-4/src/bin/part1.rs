use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline, u32},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    match cards(input) {
        Ok((_, cards)) => {
            let cards_points = cards
                .iter()
                .flat_map(|card| {
                    card.numbers
                        .iter()
                        .filter(|num| card.winning_numbers.contains(num))
                        .fold(None, |acc: Option<u32>, _| {
                            if let Some(last) = acc {
                                Some(last * 2)
                            } else {
                                Some(1)
                            }
                        })
                })
                .sum::<u32>();

            return cards_points;
        }
        Err(err) => panic!("{err}"),
    }
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, card)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(pair(tag("Card"), multispace1), u32)(input)?;
    let (input, numbers) = preceded(pair(tag(":"), multispace1), number_sections)(input)?;
    Ok((
        input,
        Card {
            id: id,
            winning_numbers: numbers.0,
            numbers: numbers.1,
        },
    ))
}

fn number_sections(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(
        numbers,
        tuple((multispace0, tag("|"), multispace0)),
        numbers,
    )(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(multispace1, u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day4_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), 13);
    }
}
