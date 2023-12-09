use std::collections::HashMap;
use std::fs;

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

fn read_input_from_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Failed to read file")
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let id = parts[0].replace("Game ", "").parse::<usize>().unwrap();
            let sets = parts[1]
                .split("; ")
                .map(|set| {
                    let mut cubes = CubeSet {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    for cube in set.split(", ") {
                        let parts: Vec<&str> = cube.split_whitespace().collect();
                        let count = parts[0].parse::<usize>().unwrap();
                        match parts[1] {
                            "red" => cubes.red = count,
                            "green" => cubes.green = count,
                            "blue" => cubes.blue = count,
                            _ => {}
                        }
                    }
                    cubes
                })
                .collect();
            Game { id, sets }
        })
        .collect()
}

fn is_game_possible(game: &Game, bag: &HashMap<&str, usize>) -> bool {
    for set in &game.sets {
        if set.red > *bag.get("red").unwrap()
            || set.green > *bag.get("green").unwrap()
            || set.blue > *bag.get("blue").unwrap()
        {
            return false;
        }
    }
    true
}

fn main() {
    // example input
    // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let file_path = "./day_02_input.txt";
    let input_data = read_input_from_file(file_path);
    let games = parse_input(&input_data);
    let mut sum = 0;

    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for game in games {
        if is_game_possible(&game, &bag) {
            sum += game.id;
        }
    }

    println!("Sum of possible game IDs: {}", sum);
}
