use core::panic;
use std::{collections::BTreeMap, fmt::Display};

type BoxIdx = u8;
type FocalLength = u8;

#[derive(Debug)]
struct LenseBox {
    lenses: Vec<Lense>,
}

impl Display for LenseBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lenses.len() > 0 {
            for lense in &self.lenses {
                write!(f, "{}:{}; ", lense.label, lense.focal_len)?;
            }
            writeln!(f, "")?;
        } else {
            writeln!(f, "[]")?;
        }
        Ok(())
    }
}

impl LenseBox {
    fn new() -> Self {
        Self { lenses: vec![] }
    }

    fn get(&self, label: &str) -> Option<u8> {
        let mut result = None;
        if let Some(idx) = self.lenses.iter().position(|lense| lense.label == label) {
            result = Some(idx as u8);
        };
        result
    }

    fn insert(&mut self, label: &str, lense: Lense) {
        if let Some(idx) = self.get(label) {
            self.lenses[idx as usize] = lense;
        } else {
            self.lenses.push(lense);
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(idx) = self.get(label) {
            self.lenses.remove(idx as usize);
        }
    }
}

#[derive(Debug)]
struct Lense {
    label: String,
    focal_len: FocalLength,
}

fn main() {
    println!("Starting Process");
    let now = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let output = process(input);
    println!("Finished in {:?}", now.elapsed());
    println!("Solution: {:?}", output);
}

// 241_094
fn process(input: &str) -> usize {
    // parse instructions
    let instructions = input
        .trim()
        .split(',')
        .filter_map(|chars| {
            if chars.contains("=") {
                let split = chars.split("=").collect::<Vec<_>>();
                Some((split[0].to_owned(), split[1].parse::<u8>().ok()))
            } else if chars.contains("-") {
                let x = chars.replace('-', "");
                Some((x, None))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // create 256 boxes, each box contains a list of lenses with a label and focal length
    // box idx is the hashed label
    // equal instruction inserts/replaces a lense with a label in a matching box_idx
    // dash instruction removes a lense with a label in a matching box_idx
    let mut lense_boxes: BTreeMap<BoxIdx, LenseBox> = BTreeMap::new();
    for box_idx in 0..=u8::MAX {
        lense_boxes.insert(box_idx, LenseBox::new());
    }

    // add and remove lenses from boxes
    for (label, focal_len) in instructions {
        let box_idx: BoxIdx = hash(label.as_ref());
        if let Some(lense_box) = lense_boxes.get_mut(&box_idx) {
            // `=` instruction:
            if focal_len.is_some() {
                lense_box.insert(
                    &label,
                    Lense {
                        label: label.to_owned(),
                        focal_len: focal_len.unwrap(),
                    },
                );
            }
            // `-` instruction:
            else {
                lense_box.remove(&label);
            }
        } else {
            panic!("could not find box: {box_idx}");
        }
    }

    for (box_idx, lense_box) in &lense_boxes {
        println!("{box_idx} -> {lense_box}");
    }

    lense_boxes
        .into_iter()
        .map(|(box_idx, lense_box)| {
            (box_idx as usize + 1) as usize
                * lense_box
                    .lenses
                    .iter()
                    .enumerate()
                    .map(|(lense_idx, lense)| (lense_idx + 1) * lense.focal_len as usize)
                    .sum::<usize>()
        })
        .sum::<usize>()
}

fn hash(input: &str) -> u8 {
    let mut result: usize = 0;
    for item in input.chars() {
        result += item as usize;
        result *= 17;
        result %= 256;
    }
    result as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day15_2023_part2() {
        assert_eq!(
            process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
