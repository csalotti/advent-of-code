fn get_digit(line: &str) -> u8 {
    let numbers: Vec<u8> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|d| (d as u8) - ('0' as u8))
        .collect();

    10 * numbers.first().unwrap() + numbers.last().unwrap()
}

fn get_line_number(line: &str) -> u8 {
    let mut positions_vals: Vec<(usize, u8)> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .flat_map(|(i, &word)| {
        vec![
            line.match_indices(&(i + 1).to_string())
                .map(|(mi, _)| (mi, i + 1))
                .collect::<Vec<(usize, usize)>>(),
            line.match_indices(word)
                .map(|(mi, _)| (mi, i + 1))
                .collect(),
        ]
    })
    .flatten()
    .map(|(idx, val)| (idx, val as u8))
    .collect();

    positions_vals.sort_by(|a, b| a.0.cmp(&b.0));

    let numbers: Vec<u8> = positions_vals.into_iter().map(|(_, val)| val).collect();

    10 * numbers.first().unwrap() + numbers.last().unwrap()
}

pub fn part_one(lines: &Vec<String>) -> u32 {
    lines
        .into_iter()
        .map(|l| get_digit(l.as_ref()) as u32)
        .sum::<u32>()
}

pub fn part_two(lines: &Vec<String>) -> u32 {
    lines
        .into_iter()
        .map(|l| get_line_number(l.as_ref()) as u32)
        .sum()
}

#[test]
fn test_calibration() {
    let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let calibration = part_one(&(lines.iter().map(|l| l.to_string()).collect::<Vec<String>>()));
    assert_eq!(calibration, 142);
}

#[test]
fn test_part_two() {
    let lines = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    let calibration = part_two(&(lines.iter().map(|l| l.to_string()).collect::<Vec<String>>()));
    assert_eq!(calibration, 281);
}
