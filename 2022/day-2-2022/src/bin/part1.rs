use std::cmp::Ordering;

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("could not parse '{value}' into a hand"),
        }
    }
}

impl PartialOrd for Hand {
    #[rustfmt::skip]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Hand::Rock, Hand::Rock) | (Hand::Paper, Hand::Paper) | (Hand::Scissors, Hand::Scissors) => Some(Ordering::Equal),
            (Hand::Rock, Hand::Scissors) | (Hand::Paper, Hand::Rock) | (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            (Hand::Rock, Hand::Paper) | (Hand::Paper, Hand::Scissors) | (Hand::Scissors, Hand::Rock) => Some(Ordering::Less),
        }
        
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Hand::Rock, Hand::Rock) | (Hand::Paper, Hand::Paper) | (Hand::Scissors, Hand::Scissors) => true,
            _ => false,
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("{output}");
}

// 11873
fn process(input: &str) -> u32 {
    let hands = parse(input);
    let points = hands
        .iter()
        .map(|(opp, me)| {
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

            points
        })
        .sum::<u32>();
    points
}

fn parse(input: &str) -> Vec<(Hand, Hand)> {
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
    fn day2_2022_part1() {
        let input = "A Y
B X
C Z";
        assert_eq!(process(input), 15); // (8 + 1 + 6)
    }
}
