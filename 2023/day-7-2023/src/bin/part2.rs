use std::{cmp::Ordering, collections::BTreeMap};

use itertools::Itertools;

#[derive(Debug)]
struct Round {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    HighCard = 0,
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
        let mut joker_wildcards = 0;

        for card in cards {
            identical_card_counts
                .entry(card)
                .and_modify(|c| *c += 1)
                .or_insert(1);

            if *card == Card::Joker {
                joker_wildcards += 1;
            }
        }

        // if there are any jokers – merge them with the biggest card that has most copies
        if joker_wildcards > 0 {
            // add jokes to the biggest card count already (but not to jokers)
            let sorted_identicals_no_jokers = identical_card_counts
                .iter()
                .filter(|x| **x.0 != Card::Joker)
                .sorted_by(|a, b| {
                    // order by card count
                    let cmp = a.1.cmp(b.1);
                    if cmp != Ordering::Equal {
                        cmp
                    } else {
                        // order by card rank
                        a.0.cmp(b.0)
                    }
                })
                .rev()
                .collect::<Vec<_>>();

            // in case of JJJJJ there wouldn't be any elements in this list
            if let Some(biggest) = sorted_identicals_no_jokers.first() {
                // add jokers to the biggest card
                identical_card_counts
                    .entry(*biggest.0)
                    .and_modify(|c| *c += joker_wildcards);

                // consume all jokers
                identical_card_counts.insert(&Card::Joker, 0);
            }
        }

        // just extract the card counts
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
    Joker = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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
            'J' => Ok(Card::Joker),
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

// jokers as wild-cards
fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 251421071
fn process(input: &str) -> u32 {
    let mut rounds = parse(input);

    // sort rounds by hand or strongest card
    rounds.sort_by(|a, b| {
        let hand_a = Hand::try_from(&a.cards).unwrap();
        let hand_b = Hand::try_from(&b.cards).unwrap();

        // when both hands are the same – sort by first highest card
        if hand_a == hand_b {
            let cmp_cards = a.cards.iter().zip(b.cards.iter()).collect::<Vec<_>>();

            for cmp_card in cmp_cards {
                let cmp = cmp_card.0.cmp(cmp_card.1);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            return Ordering::Equal;
        }

        // when hands are different – compare them directly (e.g. FiveOfAKind > FullHouse)
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
    fn day7_2023_part2() {
        let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

        assert_eq!(process(input), 6839);

        let input = "AAAA4 2
AJAAJ 3
AAAJA 4
KKAAJ 5";

        assert_eq!(process(input), 5 * 1 + 2 * 2 + 3 * 3 + 4 * 4);

        let input = "JAAAA 2
JKKKK 3
QJQQQ 4
JJJJJ 5
QQ555 6";

        assert_eq!(process(input), 6 * 1 + 5 * 2 + 3 * 3 + 2 * 4 + 4 * 5);

        // 2*3 + 3*2 + 4*1
        let input = "AAAAA 2
JJJJJ 3
2345J 4";

        assert_eq!(process(input), 16);

        let input = "AJAAA 2
JKKKJ 3
3JK5K 4";

        assert_eq!(process(input), 16);

        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process(input), 5905);
    }
}
