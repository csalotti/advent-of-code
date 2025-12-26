use std::str::FromStr;

struct Range(u64, u64);

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").ok_or(format!("No - found in {}", s))?;
        Ok(Range(
            start
                .parse()
                .map_err(|_| format!("{} is bad number", start))?,
            end.parse().map_err(|_| format!("{} is bad number", end))?,
        ))
    }
}

fn get_invalid(range: Range) -> Vec<u64> {
    let mut invalid: Vec<u64> = Vec::new();
    for val in range.0..=range.1 {
        let val_str = val.to_string();
        let val_length = val_str.chars().count();
        let (pre, post) = val_str.split_at(val_length / 2);
        if pre == post {
            invalid.push(val);
        }
    }
    invalid
}

pub fn process(input: &'static str) -> u64 {
    let invalid: Vec<u64> = input
        .trim()
        .split(",")
        .map(|r_str| r_str.trim().parse::<Range>().unwrap())
        .map(|range| get_invalid(range))
        .flatten()
        .collect();

    let invalid_sum = invalid.iter().fold(0, |acc, x| acc + x);

    invalid_sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_02_part_one() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224, 1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(process(input), 1227775554);
    }
}
