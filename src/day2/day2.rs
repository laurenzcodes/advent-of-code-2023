use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Cube {
    color: String,
    count: u32,
}

struct Game {
    id: u32,
    cubes: Vec<Vec<Cube>>,
}

fn parse_cube(s: &str) -> Cube {
    let parts: Vec<&str> = s.split_whitespace().collect();
    Cube {
        count: parts[0].parse().unwrap(),
        color: parts[1].to_string(),
    }
}

fn parse_game(s: &str) -> Game {
    let parts: Vec<&str> = s.split(": ").collect();
    let id: u32 = parts[0][5..].parse().unwrap();
    let cube_sets: Vec<Vec<Cube>> = parts[1]
        .split("; ")
        .map(|set| set.split(", ").map(parse_cube).collect())
        .collect();
    Game {
        id,
        cubes: cube_sets,
    }
}

fn load_games<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Game>> {
    let file = File::open(&filename)?;
    let games: Vec<Game> = io::BufReader::new(file)
        .lines()
        .map(|line| parse_game(&line.unwrap()))
        .collect();
    Ok(games)
}

fn is_possible(game: &Game, red: u32, green: u32, blue: u32) -> bool {
    for set in &game.cubes {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for cube in set {
            match cube.color.as_str() {
                "red" => r += cube.count,
                "green" => g += cube.count,
                "blue" => b += cube.count,
                _ => (),
            }
        }
        if r > red || g > green || b > blue {
            return false;
        }
    }
    true
}

fn min_cubes(game: &Game) -> (u32, u32, u32) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for set in &game.cubes {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for cube in set {
            match cube.color.as_str() {
                "red" => red = cube.count,
                "green" => green = cube.count,
                "blue" => blue = cube.count,
                _ => (),
            }
        }
        min_red = min_red.max(red);
        min_green = min_green.max(green);
        min_blue = min_blue.max(blue);
    }
    (min_red, min_green, min_blue)
}

fn power(cubes: (u32, u32, u32)) -> u32 {
    cubes.0 * cubes.1 * cubes.2
}

fn main() -> io::Result<()> {
    let games = load_games("day2.txt")?;
    let sum_ids: u32 = games
        .iter()
        .filter(|game| is_possible(game, 12, 13, 14))
        .map(|game| game.id)
        .sum();
    println!("{}", sum_ids);
    let sum_powers: u32 = games.iter().map(|game| power(min_cubes(game))).sum();
    println!("{}", sum_powers);
    Ok(())
}
