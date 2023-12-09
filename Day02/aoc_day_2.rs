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

impl CubeSet {
    fn new() -> CubeSet {
        CubeSet { red: 0, green: 0, blue: 0 }
    }
}

impl Game {
    fn parse_input(input: &str) -> Vec<Game> {
        input.lines().map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let id = parts[0].replace("Game ", "").parse::<usize>().unwrap();
            let sets = parts[1].split("; ")
                .map(|set_str| {
                    let mut cubes = CubeSet::new();
                    for cube in set_str.split(", ") {
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
        }).collect()
    }

    fn is_game_possible(&self, bag: &std::collections::HashMap<&str, usize>) -> bool {
        self.sets.iter().all(|set| {
            set.red <= *bag.get("red").unwrap() &&
            set.green <= *bag.get("green").unwrap() &&
            set.blue <= *bag.get("blue").unwrap()
        })
    }

    fn find_minimum_cubes(&self) -> (usize, usize, usize) {
        let min_red = self.sets.iter().map(|set| set.red).max().unwrap_or(0);
        let min_green = self.sets.iter().map(|set| set.green).max().unwrap_or(0);
        let min_blue = self.sets.iter().map(|set| set.blue).max().unwrap_or(0);
        (min_red, min_green, min_blue)
    }
}

fn calculate_power(red: usize, green: usize, blue: usize) -> usize {
    red * green * blue
}

fn main() {
    let data = fs::read_to_string("./day_02_input.txt").expect("Unable to read file");
    
    let games = Game::parse_input(&data);
    
    let mut sum_ids = 0;
    let mut total_power = 0;

    let bag = std::collections::HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    for game in games {
        let (min_red, min_green, min_blue) = game.find_minimum_cubes();
        let power = calculate_power(min_red, min_green, min_blue);
        total_power += power;
        println!("Game {}: Minimum cubes - Red: {}, Green: {}, Blue: {}, Power: {}", game.id, min_red, min_green, min_blue, power);
        if game.is_game_possible(&bag) {
            sum_ids += game.id;
        }
    }

    println!("Sum of possible game IDs: {}", sum_ids);
    println!("Total power of minimum sets: {}", total_power);
}
