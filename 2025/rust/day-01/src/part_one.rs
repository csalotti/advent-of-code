pub fn process(input: &'static str) -> u32 {
    let instructions = input.trim().split('\n').map(|l| l.trim()).map(|i| {
        if i.starts_with("L") {
            i.strip_prefix("L")
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
        } else {
            -1 * i
                .strip_prefix("R")
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
        }
    });

    let mut acc: i32 = 50;
    let mut count: u32 = 0;
    for i in instructions {
        acc = (acc + i) % 100;
        if acc == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_01_part_one() {
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
        assert_eq!(process(input), 3);
    }
}
