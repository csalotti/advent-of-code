mod part_one;
mod part_two;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let part = args.get(1).map(String::as_str).unwrap_or("part_one");
    let input = include_str!("../../data/day_01.txt");

    let result = match part {
        "part_one" => part_one::process(input),
        "part_two" => part_two::process(input),
        _ => panic!("Unknown part"),
    };

    println!("{} {}", part, result);
}
