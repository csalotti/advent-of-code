fn get_top_2(banks: &str) -> u64 {
    let mut val_max_left = 0;
    let mut pos_max_left = 0;

    let batteries: Vec<u32> = banks.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for i in 0..batteries.len() - 1 {
        let val = batteries[i];
        if val > val_max_left {
            val_max_left = val;
            pos_max_left = i;
        }
    }

    let mut val_max_right = 0;
    for i in pos_max_left + 1..batteries.len() {
        let val = batteries[i];
        if val > val_max_right {
            val_max_right = val;
        }
    }

    format!("{val_max_left}{val_max_right}").parse().unwrap()
}

pub fn process(input: &'static str) -> u64 {
    input.trim().split("\n").map(|l| get_top_2(l.trim())).sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_03_part_one() {
        let input = "987654321111111
                                   811111111111119
                                   234234234234278
                                   818181911112111";
        assert_eq!(process(input), 357);
    }
}
