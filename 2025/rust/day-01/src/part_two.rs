pub fn process(input: &'static str) -> u32 {
    let instructions = input.trim().split('\n').map(|l| l.trim()).map(|i| {
        if i.starts_with("L") {
            -1 * i
                .strip_prefix("L")
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
        } else {
            i.strip_prefix("R")
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
        }
    });

    let mut curr: i32 = 50;
    let mut count: u32 = 0;
    println!("{:?}", instructions.clone().collect::<Vec<i32>>());
    for i in instructions {
        let next = (curr + i).rem_euclid(100);
        // Counted twice otherwise
        if curr == 0 {
            curr = next;
            continue;
        }

        if (i < 0) && (next > curr) {
            count += 1;
        } else if (i > 0) && (next < curr) {
            count += 1;
        } else if next == 0 {
            count += 1;
        } else {
        }
        curr = next;
    }

    count
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_01_part_two() {
        let input = "L68
                                    L30
                                    R48
                                    L5
                                    R60
                                    L55
                                    L1
                                    L99
                                    R14
                                    L82";
        assert_eq!(process(input), 6);
    }
}
