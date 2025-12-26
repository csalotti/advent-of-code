use std::collections::{HashSet, VecDeque};
use std::mem::swap;

pub fn process(input: &'static str) -> u64 {
    let mut nsplit = 0;
    let mut lines: VecDeque<&str> = input.trim().split("\n").map(|l| l.trim()).collect();
    let mut positions: Vec<usize> = Vec::new();
    let mut current: HashSet<usize> = HashSet::new();

    // Find first position
    for (i, &c) in lines.pop_front().unwrap().as_bytes().iter().enumerate() {
        if c == b'S' {
            positions.push(i);
            current.insert(i);
        }
    }

    // BFS
    for l in lines {
        let mut next_pos: Vec<usize> = Vec::new();
        while let Some(i) = positions.pop() {
            if l.as_bytes()[i] == b'^' {
                nsplit += 1;
                current.remove(&i);
                if !current.contains(&(i - 1)) {
                    next_pos.push(i - 1);
                    current.insert(i - 1);
                }
                if !current.contains(&(i + 1)) {
                    positions.push(i + 1);
                    current.insert(i + 1);
                }
            } else {
                next_pos.push(i);
            }
        }
        swap(&mut next_pos, &mut positions);
    }
    nsplit
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_07_part_one() {
        let input = ".......S.......
                                   ...............
                                   .......^.......
                                   ...............
                                   ......^.^......
                                   ...............
                                   .....^.^.^.....
                                   ...............
                                   ....^.^...^....
                                   ...............
                                   ...^.^...^.^...
                                   ...............
                                   ..^...^.....^..
                                   ...............
                                   .^.^.^.^.^...^.
                                   ...............";
        assert_eq!(process(input), 21);
    }
}
