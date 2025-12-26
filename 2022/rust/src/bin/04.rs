struct Section {
    left: u32,
    right: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|pair| {
                pair.split(",")
                    .map(|section| section.split("-").map(|s| s.parse::<u32>().unwrap()))
                    .map(|mut section| Section {
                        left: section.next().unwrap(),
                        right: section.next().unwrap(),
                    })
                    .collect::<Vec<_>>()
            })
            .map(|p| {
                if (p[0].left >= p[1].left) && (p[0].right <= p[1].right) {
                    1
                } else if (p[0].left <= p[1].left) && (p[0].right >= p[1].right) {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (l, r) = l.split_once(",").unwrap();
                let ((a, b), (c, d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
                (
                    a.parse::<u32>().unwrap(),
                    b.parse::<u32>().unwrap(),
                    c.parse::<u32>().unwrap(),
                    d.parse::<u32>().unwrap(),
                )
            })
            .filter(|(l1, r1, l2, r2)| {
                (l1 <= l2 && r1 >= l2)
                    || (l1 >= l2 && l1 <= r2)
                    || (l1 <= l2 && r1 >= r2)
                    || (l1 >= l2 && r1 <= r2)
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
