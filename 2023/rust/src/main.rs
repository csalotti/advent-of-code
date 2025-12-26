mod day1;
mod day2;

use std::{fs::File, io::BufRead, io::BufReader};

fn day_1() {
    let file = File::open("inputs/day1.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();
    println!("Part one : {}", day1::part_one(&lines));
    println!("Part two : {}", day1::part_two(&lines));
}

fn day_2() {
    let file = File::open("inputs/day2.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();
    println!("Part one : {}", day2::n_valid_games(&lines));
    // println!("Part two : {}", part_two(&lines));
}

fn main() {
    // day_1();
    day_2();
}
