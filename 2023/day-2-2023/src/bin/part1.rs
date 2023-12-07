use std::collections::HashMap;

struct Game {
    id: i32,
    sets: Vec<HashMap<Color, i32>>,
}

impl Game {
    fn new(id: i32) -> Self {
        Self {
            id,
            sets: Vec::new(),
        }
    }
    fn is_valid(&self) -> bool {
        for set in &self.sets {
            if let Some(red) = set.get(&Color::Red) {
                if *red > 12 {
                    return false;
                }
            }
            if let Some(green) = set.get(&Color::Green) {
                if *green > 13 {
                    return false;
                }
            }
            if let Some(blue) = set.get(&Color::Blue) {
                if *blue > 14 {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let input = include_str!("../../input.txt");
    dbg!(input);
    let output = process(input);
    dbg!(output);
}

// In this approach we will deserialize all string data into concrete types, then we will calculate the sum.
fn process(input: &str) -> i32 {
    // deserialize
    let mut games = vec![];
    for line in input.lines() {
        let game_sets: Vec<&str> = line.split(":").collect();

        dbg!(&game_sets);

        let game_id = game_sets[0].split(" ").collect::<Vec<_>>()[1]
            .parse()
            .unwrap();

        let mut game = Game::new(game_id);

        let sets: Vec<&str> = game_sets[1].split(";").collect();
        for set in sets {
            let cubes: Vec<&str> = set.split(",").collect();

            let mut map = HashMap::new();

            for cube in cubes {
                let count_color: Vec<&str> = cube.trim().split(" ").collect();
                let cube_count: i32 = count_color[0].parse().unwrap();
                let cube_color = count_color[1];
                let color = match cube_color {
                    "red" => Some(Color::Red),
                    "green" => Some(Color::Green),
                    "blue" => Some(Color::Blue),
                    _ => None,
                };
                if let Some(color) = color {
                    map.insert(color, cube_count);
                    // game.sets
                    //     .entry(color)
                    //     .and_modify(|x| *x += cube_count)
                    //     .or_insert(cube_count);
                }
            }

            game.sets.push(map);
        }
        games.push(game);
    }

    for game in &games {
        println!("{0}", game.id);
        for set in &game.sets {
            println!(
                "  r:{0:?}, g:{1:?}, b:{2:?}",
                set.get(&Color::Red),
                set.get(&Color::Green),
                set.get(&Color::Blue)
            );
        }
    }

    count_total_valid_games(games)
}

fn count_total_valid_games(games: Vec<Game>) -> i32 {
    // 12 red cubes
    // 13 green cubes
    // 14 blue cubes
    let mut total = 0;
    for game in games {
        if game.is_valid() {
            total += game.id;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day2_part1() {
        assert_eq!(
            process(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            1 + 2 + 5
        );
    }
}
