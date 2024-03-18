use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_puzzle(file_name: &str) -> Vec<String> {
    let f: File = File::open(file_name).unwrap();
    let f: BufReader<File> = BufReader::new(f);

    f.lines()
        .map(|line|line.unwrap())
        .collect::<Vec<String>>()
}

fn add_padding(lines: &[String]) -> Vec<Vec<char>> {
    let padding_line = ".".repeat(lines[0].len())
        .chars()
        .collect::<Vec<char>>();

    let mut chars = lines.iter().map(|line| {
        let mut l = line.chars().collect::<Vec<char>>();
        l.insert(0, '.');
        l.push('.');
        l
    }).collect::<Vec<Vec<char>>>();

    chars.push(padding_line.clone());
    chars.insert(0, padding_line);

    chars
}

fn add_all_part_numbers(lines_of_chars: &Vec<Vec<char>>) -> u32 {
    let mut num_buff: Option<u32> = None;
    let mut cols: Vec<usize> = vec![];
    let mut acc = 0;

    for (row, line) in lines_of_chars.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if let Some(x) = ch.to_digit(10) {
                num_buff = num_buff
                    .map(|a| a * 10 + x)
                    .or(Some(x));
                cols.push(col);
            } else {
                if let Some(x) = num_buff {
                    let start = cols.first().unwrap() - 1;
                    let end = cols.last().unwrap() + 1;
                    let stamp: Vec<&char> = lines_of_chars[row - 1][start..=end].iter()
                        .chain(lines_of_chars[row][start..=end].iter())
                        .chain(lines_of_chars[row + 1][start..=end].iter())
                        .filter(|&&c| c != '.' && !c.is_numeric())
                        .collect();

                    if !stamp.is_empty() {
                        //println!("{x}");
                        acc += x;
                    }
                }
                num_buff = None;
                cols = vec![];
            }
        }
    }
    acc
}

fn main() {
    let file_name = "test_input.txt";
    //let file_name = "puzzle_input.txt";

    let lines = load_puzzle(file_name);
    let lines_of_chars = add_padding(&lines);

    //println!("{:?}", lines_of_chars);

    // part 1
    let acc = add_all_part_numbers(&lines_of_chars);
    println!("Sum of all of the part numbers in the engine schematic: {acc}");

    // part 2
    todo!();
}

