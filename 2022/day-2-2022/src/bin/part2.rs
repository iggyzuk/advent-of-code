use std::cmp::Ordering;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "🪨"),
            Self::Paper => write!(f, "📄"),
            Self::Scissors => write!(f, "✂️"),
        }
    }
}

impl Clone for Hand {
    fn clone(&self) -> Self {
        match self {
            Self::Rock => Self::Rock,
            Self::Paper => Self::Paper,
            Self::Scissors => Self::Scissors,
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("could not parse '{value}' into a hand"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.beats(), self.loses(), other) {
            (beats, _, hand) if beats == *hand => Some(Ordering::Greater),
            (_, loses, hand) if loses == *hand => Some(Ordering::Less),
            _ => Some(Ordering::Equal),
        }
    }
}

impl PartialEq for Hand {
    #[rustfmt::skip]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Hand::Rock, Hand::Rock) | (Hand::Paper, Hand::Paper) | (Hand::Scissors, Hand::Scissors) => true,
            _ => false,
        }
    }
}

impl Hand {
    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
    fn loses(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

enum Round {
    Win,
    Lose,
    Draw,
}

impl std::fmt::Debug for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win => write!(f, "⭐️"),
            Self::Lose => write!(f, "💀"),
            Self::Draw => write!(f, "👀"),
        }
    }
}

impl From<&str> for Round {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("could not parse '{value}' into a round"),
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 12014
fn process(input: &str) -> u32 {
    let hands = parse(input);
    let points = hands
        .into_iter()
        .map(|(opp, round)| {
            // round tells me wether I must win, lose, or draw – my hand depends on opponent's hand
            let me = match round {
                Round::Draw => opp.clone(),
                Round::Win => opp.loses(),
                Round::Lose => opp.beats(),
            };

            let mut points = match me {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            };

            if me > opp {
                points += 6;
            } else if me == opp {
                points += 3;
            }

            println!("{me:?} vs {opp:?} = {round:?} ({points})");

            points
        })
        .sum::<u32>();
    points
}

fn parse(input: &str) -> Vec<(Hand, Round)> {
    input
        .lines()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<_>>();
            (split[0].into(), split[1].into())
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day2_2022_part2() {
        let input = "A Y
B X
C Z";
        assert_eq!(process(input), 12);
    }
}
