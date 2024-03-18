use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Grab {
    red: u16,
    green: u16,
    blue: u16,
}

enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("no such color")
        }
    }
}

impl From<&str> for Grab {
    fn from(value: &str) -> Self {
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;

        let splits = value.split(",");
        for split in splits {
            match split
                .trim()
                .split_once(" ")
                .map(|(num, color)| (i32::from_str(num).unwrap(), Color::from(color))) {
                Some((x, Color::Red)) => r = x,
                Some((x, Color::Green)) => g = x,
                Some((x, Color::Blue)) => b = x,
                None => panic!("split failed..."),
            }
        }

        Grab { red: r as u16, green: g as u16, blue: b as u16 }
    }
}

#[derive(Debug)]
struct Game {
    grabs: Vec<Grab>,
    id: usize,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, grabs) = value.split_once(":").unwrap();
        let id = usize::from_str(game.trim_start_matches("Game ")).unwrap();
        let grabs: Vec<Grab> = grabs
            .split(";")
            .map(str::trim)
            .map(Grab::from)
            .collect();

        Game { id, grabs }
    }
}

impl From<&String> for Game {
    fn from(value: &String) -> Self {
        Game::from(value.as_str())
    }
}

fn load_puzzle(name: &str) -> Vec<String> {
    let f: File = File::open(name).unwrap();
    let f: BufReader<File> = BufReader::new(f);

    f.lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
}

fn filter_game(game: &Game) -> bool {
    const MAX_GRAB: Grab = Grab { red: 12, green: 13, blue: 14 };

    game.grabs.iter().all(|grab: &Grab|
        grab.red <= MAX_GRAB.red
            && grab.green <= MAX_GRAB.green
            && grab.blue <= MAX_GRAB.blue
    )
}

fn make_minimal_grab(grabs: &Vec<Grab>) -> Grab {
    grabs.iter().fold(
        Grab{
            red: u16::MIN,
            green: u16::MIN,
            blue: u16::MIN
        },
        |min_grab, this_grab| Grab {
            red: min_grab.red.max(this_grab.red),
            green: min_grab.green.max(this_grab.green),
            blue: min_grab.blue.max(this_grab.blue),
        }
    )
}

fn main() {
    // let file_name = "test_input.txt";
    let file_name = "puzzle_input.txt";

    let lines = load_puzzle(file_name);

    let games: Vec<Game> = lines
        .iter()
        .map(Game::from)
        .collect();

    //println!("{:?}", games);

    let valid_games: Vec<usize> = games.iter()
        .filter(|&game| filter_game(game))
        .map(|game| game.id)
        .collect();

    let valid_id_sum: usize = valid_games.iter().sum();

    // println!("{:?}", games);
    println!("---\nSUM = {valid_id_sum}");

    let minimal_grabs_product_sum: u64 = games
        .iter()
        .map(|game| make_minimal_grab(game.grabs.as_ref()))
        .map(|Grab {red: r, green: g, blue: b}| (r * g * b) as u64)
        .sum();

    println!("---\nminimal grabs product sum: {:?}", minimal_grabs_product_sum);
}
