use std::cmp::max;
use regex::Regex;
use lazy_static::lazy_static;

const P1_DATUM: Game = Game{
    id: 0,
    blue: 14,
    green: 13,
    red: 12,
};


pub fn run(file: &str) -> (usize,usize)  {
    let games = read_data(file);
    let p1 = pt1(&games);
    let p2 = pt2(&games);
    (p1,p2)
}

#[derive(Debug,Default)]
struct Game {
    id: usize,
    blue: usize,
    green: usize,
    red: usize,
}

fn read_data(file: &str) -> Vec<Game> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) (\w+)").expect("Failed to compile regex");
    }
    std::fs::read_to_string(file)
    .expect("File not found")
    .lines()
    .map(|x| {
        let mut game = Game{..Default::default()};
        if let Some((gid,gnums)) = x.split_once(':') {
            game.id = gid.trim_start_matches(char::is_alphabetic).trim().parse::<usize>().unwrap();
            for round in gnums.split(';') {
                    for sequence in round.split(','){
                    let cap = RE.captures(sequence).expect("Failed to match regex");
                    match &cap[2] {
                        "red" => {
                            game.red = max(game.red, cap[1].parse::<usize>().expect("Failed to parse red"))
                        }
                        "blue" => {
                            game.blue = max(game.blue, cap[1].parse::<usize>().expect("Failed to parse blue"))
                        }
                        "green" => {
                            game.green = max(game.green, cap[1].parse::<usize>().expect("Failed to parse green"))
                        }
                        _ => panic!("Unknown colour")
                    }
                }
            }
        };
        game
    }).collect()
}

fn pt1(games: &Vec<Game>) -> usize{
    let mut res = 0;
    for game in games {
        if game.red <= P1_DATUM.red &&
        game.green <= P1_DATUM.green &&
        game.blue   <=  P1_DATUM.blue {
            res += game.id;
        }
    }
    res
}

fn pt2(games: &[Game]) -> usize{
   games.iter().map(|game| {
    game.red * game.blue * game.green
   }).sum()
}