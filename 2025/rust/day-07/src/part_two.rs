use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Node {
    position: usize,
    line: usize,
}

pub fn process(input: &'static str) -> u64 {
    let lines: Vec<&str> = input.trim().split("\n").map(|l| l.trim()).collect();
    let (start, tail) = lines.split_first().unwrap();
    let mut nodes: Vec<Node> = Vec::new();
    let mut registry: HashMap<Node, u64> = HashMap::new();

    // Find first position
    let mut head: Node = Node {
        position: 0,
        line: 0,
    };
    for (i, &c) in start.as_bytes().iter().enumerate() {
        if c == b'S' {
            head = Node {
                position: i,
                line: 0,
            };
            nodes.push(head)
        }
    }

    // DFS
    while let Some(node) = nodes.pop() {
        if node.line == tail.len() - 1 {
            registry.insert(node, 1);
            continue;
        }
        if tail[node.line].as_bytes()[node.position] == b'^' {
            let right = Node {
                line: node.line + 1,
                position: node.position + 1,
            };
            let left = Node {
                line: node.line + 1,
                position: node.position - 1,
            };

            if registry.contains_key(&left) && registry.contains_key(&right) {
                registry.insert(node, registry[&left] + registry[&right]);
                continue;
            }
            nodes.push(node);
            if !registry.contains_key(&left) {
                nodes.push(left);
            }
            if !registry.contains_key(&right) {
                nodes.push(right);
            }
        } else {
            let next = Node {
                line: node.line + 1,
                position: node.position,
            };
            if registry.contains_key(&next) {
                registry.insert(node, registry[&next]);
                continue;
            }
            nodes.push(node);
            nodes.push(next);
        }
    }

    registry[&head]
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_07_part_two() {
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
        assert_eq!(process(input), 40);
    }
}
