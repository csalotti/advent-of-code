fn get_joltages(banks: &str, n: usize) -> u64 {
    let batteries: Vec<u32> = banks.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut left = 0;
    let mut right = batteries.len() - n;
    let mut joltages: Vec<u32> = Vec::new();

    for _ in 0..n {
        let mut max_val = 0;
        let mut max_pos = 0;
        for i in left..=right {
            let val = batteries[i];
            if val > max_val {
                max_val = val;
                max_pos = i;
            }
        }
        joltages.push(max_val);
        left = max_pos + 1;
        right += 1;
    }
    joltages
        .iter()
        .map(|j| j.to_string())
        .collect::<String>()
        .parse::<u64>()
        .expect("Not a number")
}

pub fn process(input: &'static str, n: usize) -> u64 {
    input
        .trim()
        .split("\n")
        .map(|l| get_joltages(l.trim(), n))
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_03_part_one_bis() {
        let input = "987654321111111
                                   811111111111119
                                   234234234234278
                                   818181911112111";
        assert_eq!(process(input, 2), 357);
    }

    #[test]
    fn day_03_part_two() {
        let input = "987654321111111
                                   811111111111119
                                   234234234234278
                                   818181911112111";
        assert_eq!(process(input, 12), 3121910778619);
    }
}
