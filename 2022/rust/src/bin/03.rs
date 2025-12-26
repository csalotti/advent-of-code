use std::{cmp::Ordering, collections::HashSet};

const L_ORIGIN: u32 = 'a' as u32;
const U_ORIGIN: u32 = 'A' as u32;

fn find_duplicate(content: &str) -> Option<u32> {
    let (first, second) = content.split_at(content.chars().count() / 2);
    let hashset: HashSet<char> = HashSet::from_iter(first.chars().into_iter());
    for c in second.chars() {
        if hashset.contains(&c) {
            let c_ord = c as u32;
            let prio = match (c_ord).cmp(&L_ORIGIN) {
                Ordering::Less => Some(c_ord - U_ORIGIN + 27),
                Ordering::Greater => Some(c_ord - L_ORIGIN + 1),
                Ordering::Equal => Some(1),
            };
            return prio;
        }
    }
    None
}
pub fn part_one(input: &str) -> Option<u32> {
    let total_duplicate_prio = input
        .lines()
        .map(find_duplicate)
        .map(|c| match c {
            Some(n) => n,
            None => 0,
        })
        .sum();

    Some(total_duplicate_prio)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|c| {
                c[0].chars()
                    .into_iter()
                    .filter(|x| c[1].contains(*x) && c[2].contains(*x))
                    .next()
                    .unwrap()
            })
            .map(|v| {
                if (v as u32) >= L_ORIGIN {
                    (v as u32) - L_ORIGIN + 1
                } else {
                    (v as u32) - U_ORIGIN + 27
                }
            })
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
