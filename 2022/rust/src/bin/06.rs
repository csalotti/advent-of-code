use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut buffer: HashMap<char, u32> = HashMap::new();
    let (mut left, mut right) = (0, 0);

    for c in input.chars() {
        if buffer.contains_key(&c) {
            let tmp = *buffer.get(&c)?;
            left = if tmp > left { tmp } else { left };
        }
        buffer.insert(c, right);
        if right - left == 4 {
            return Some(right + 1);
        }
        right += 1;
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut buffer: HashMap<char, u32> = HashMap::new();
    let (mut left, mut right) = (0, 0);

    for c in input.chars() {
        if buffer.contains_key(&c) {
            let tmp = *buffer.get(&c)?;
            left = if tmp > left { tmp } else { left };
        }
        buffer.insert(c, right);
        if right - left == 14 {
            return Some(right + 1);
        }
        right += 1;
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
