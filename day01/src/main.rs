use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(_f: &str) -> Vec<String> {
    let f: File = File::open(_f).unwrap();
    let f: BufReader<File> = BufReader::new(f);

    f.lines()
        .map(|line| line.unwrap()).collect()
}

fn break_into_chars(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|s|
            s.chars()
                .filter(|&c| c.is_numeric())
                .collect::<Vec<char>>()
        )
        .collect()
}

fn first_and_last_num_as_int(lines_of_nums: Vec<Vec<char>>) -> Vec<(u8, u8)> {
    lines_of_nums
        .iter()
        .map(|v| (v.first().unwrap(), v.last().unwrap()))
        .map(|(&a, &b)| (a as u8 - 48, b as u8 - 48))
        .collect()
}

fn convert_number_words(lines: Vec<String>) -> Vec<String> {
    /// For day 1 part 2: convert each line, by transforming
    /// - numbers by words to numbers,
    /// - keep numbers only,
    /// - in original order
    const NUMBERS: [(&str, char); 9] = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    fn convert_word(word: &str) -> String {
        let mut acc: Vec<(char, usize)> = vec![];

        for (number_word, number_value) in NUMBERS {
            let f: Vec<(char, usize)> = word
                .match_indices(number_word)
                .map(|(idx, _)| (number_value, idx))
                .collect();

            acc.extend(f);
        }

        for (idx, ch) in word.chars().enumerate() {
            if ch.is_numeric() {
                acc.push((ch, idx));
            }
        }

        acc.sort_by(|(_, idx_1), (_, idx_2)| idx_1.cmp(idx_2));

        let vec_of_chars = acc
            .iter()
            .map(|tup| tup.0 );

        String::from_iter(vec_of_chars)
    }

    lines
        .iter()
        .map(|s| convert_word(s.as_ref()))
        .collect()
}

fn main() {
    let filename = "./puzzle_1.txt";
    // let filename = "./test_input.txt";

    let lines = read_input(filename);
    // part 2
    let lines = convert_number_words(lines);

    let lines_of_nums: Vec<Vec<char>> = break_into_chars(lines);
    let first_and_last_num: Vec<(u8, u8)> = first_and_last_num_as_int(lines_of_nums);
    let nums: Vec<u64> = first_and_last_num
        .iter()
        .map(|(a, b)| (a * 10 + b) as u64)
        .collect();

    // println!("{:?}", nums);

    let sum_of_nums: u64 = nums.iter().sum();

    println!("----\nSUM: {sum_of_nums}");
}
