use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_puzzle(file_name: &str) -> Vec<String> {
    let f: File = File::open(file_name).unwrap();
    let f: BufReader<File> = BufReader::new(f);

    f.lines()
        .map(|line|line.unwrap())
        .collect::<Vec<String>>()
}

fn main() {
    let file_name = "test_input.txt";
    // let file_name = "puzzle_input.txt";

    let lines = load_puzzle(file_name);

    for &line in lines.iter() {

    }
    println!("{:?}", lines);
}
