use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug)]
struct Round {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl TryFrom<&Vec<Card>> for Hand {
    type Error = String;

    fn try_from(cards: &Vec<Card>) -> Result<Self, Self::Error> {
        // count how many cards are of the same type (e.g. AAA12 -> Ace:3, One:1, Two:1)
        let mut identical_card_counts = BTreeMap::new();
        for card in cards {
            identical_card_counts
                .entry(card)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        // just extract the counts
        let identicals = identical_card_counts
            .iter()
            .map(|card| *card.1)
            .collect::<Vec<_>>();

        // five of a kind – 1 must have count of 5
        if identicals.contains(&5) {
            return Ok(Hand::FiveOfAKind);
        }
        // four of a kind – 1 must have count of 4
        else if identicals.contains(&4) {
            return Ok(Hand::FourOfAKind);
        }
        // full house – one must have count of 3, the other must have count of 2
        else if identicals.contains(&3) && identicals.contains(&2) {
            return Ok(Hand::FullHouse);
        }
        // four of a kind – 1 must have count of 3
        else if identicals.contains(&3) {
            return Ok(Hand::ThreeOfAKind);
        }
        // two of a kind – 2 must have count of 2
        else if identicals.iter().filter(|x| **x == 2).count() == 2 {
            return Ok(Hand::TwoPair);
        }
        // one pair – 1 must have count of 2
        else if identicals.contains(&2) {
            return Ok(Hand::OnePair);
        }
        // or just return a high card
        return Ok(Hand::HighCard);
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(format!("could not convert '{value}' to a card")),
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 251121738
fn process(input: &str) -> u32 {
    let mut rounds = parse(input);

    // sort by hand
    rounds.sort_by(|a, b| {
        let hand_a = Hand::try_from(&a.cards).unwrap();
        let hand_b = Hand::try_from(&b.cards).unwrap();

        // when both hands are the same – sort by first highest card
        if hand_a == hand_b {
            let cmp_cards = a.cards.iter().zip(b.cards.iter());
            for cmp_card in cmp_cards {
                let cmp = cmp_card.0.cmp(cmp_card.1);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            return Ordering::Equal;
        }

        // when hands are different – compare them
        hand_a.cmp(&hand_b)
    });

    rounds
        .into_iter()
        .enumerate()
        .map(|(i, r)| (i as u32 + 1) * r.bid)
        .sum::<u32>()
}

fn parse(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let line_split = line.split(" ").collect::<Vec<_>>();

            let cards_str = line_split[0];
            let bid_str = line_split[1];

            let cards = cards_str
                .chars()
                .map(|card_char| match card_char.try_into() {
                    Ok(card) => card,
                    Err(err) => panic!("could not parse a card: {err}"),
                })
                .collect::<Vec<_>>();

            Round {
                cards: cards.into(),
                bid: bid_str.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day7_2023_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process(input), 6440);
    }
}
