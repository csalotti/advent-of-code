use regex::Regex;

fn init_stacks(stacks_input: &str) -> Vec<Vec<char>> {
    let stack_size = stacks_input
        .lines()
        .rev()
        .next()
        .unwrap()
        .split("  ")
        .last()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = Default::default();

    for _ in 0..stack_size {
        stacks.push(Default::default())
    }
    stacks_input.lines().rev().skip(1).for_each(|l| {
        l.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, c)| c != &' ')
            .for_each(|(i, c)| stacks[i].push(c))
    });

    stacks
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks_input, moves_input) = input.split_once("\n\n")?;
    let re = Regex::new(r"(\d+) from (\d+) to (\d+)").unwrap();

    let mut stacks = init_stacks(stacks_input);

    for cap in re.captures_iter(moves_input) {
        let (quantity, from, to) = (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap() - 1,
            cap[3].parse::<usize>().unwrap() - 1,
        );
        for _ in 0..quantity {
            let tmp = stacks[from].pop().unwrap();
            stacks[to].push(tmp)
        }
    }

    Some(
        stacks
            .into_iter()
            .map(|s| s.last().unwrap().to_owned())
            .collect::<String>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks_input, moves_input) = input.split_once("\n\n")?;
    let re = Regex::new(r"(\d+) from (\d+) to (\d+)").unwrap();

    let mut stacks = init_stacks(stacks_input);

    for cap in re.captures_iter(moves_input) {
        let (quantity, from, to) = (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap() - 1,
            cap[3].parse::<usize>().unwrap() - 1,
        );
        let mut tmp_vec: Vec<char> = Default::default();
        for _ in 0..quantity {
            tmp_vec.push(stacks[from].pop().unwrap());
        }
        tmp_vec.reverse();
        stacks[to].append(&mut tmp_vec);
    }

    Some(
        stacks
            .into_iter()
            .map(|s| s.last().unwrap().to_owned())
            .collect::<String>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
