use std::collections::{HashMap, HashSet};
use std::str::FromStr;
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct JBox {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for JBox {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match *s.split(',').collect::<Vec<&str>>() {
            [x, y, z] => Ok(JBox {
                x: x.parse().expect("{x} isn't u64"),
                y: y.parse().expect("{y} isn't u64"),
                z: z.parse().expect("{z} isn't u64"),
            }),
            _ => panic!(" {s} isn't a Junction Box"),
        }
    }
}

impl JBox {
    fn distance(self: Self, other: JBox) -> u64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as u64
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, Eq, Hash, Ord)]
struct Connection {
    a: JBox,
    b: JBox,
    distance: u64,
}

impl Connection {
    fn new(a: JBox, b: JBox) -> Self {
        let distance = a.distance(b);
        Self { a, b, distance }
    }
}

struct Graph {
    graph: HashMap<JBox, Vec<JBox>>,
}

impl Graph {
    fn new() -> Self {
        let graph: HashMap<JBox, Vec<JBox>> = HashMap::new();
        Self { graph }
    }

    fn add(self: &mut Self, connection: Connection) {
        self.graph
            .entry(connection.a)
            .or_default()
            .push(connection.b);
        self.graph
            .entry(connection.b)
            .or_default()
            .push(connection.a);
    }

    fn get_n_circuits(self: &Self) -> usize {
        let mut circuits: Vec<HashSet<JBox>> = Vec::new();
        let mut visited: HashSet<JBox> = HashSet::new();
        for node in self.graph.keys() {
            if visited.contains(node) {
                continue;
            }
            let mut circuit: HashSet<JBox> = HashSet::new();
            let mut stack = vec![node];

            while let Some(&curr) = stack.pop() {
                if !visited.contains(&curr) {
                    circuit.insert(curr);
                    visited.insert(curr);
                    for adj in self.graph[&curr].iter() {
                        stack.push(&adj);
                    }
                }
            }
            circuits.push(circuit);
        }
        return circuits.len();
    }
}

pub fn process(input: &'static str) -> u64 {
    let jboxes: Vec<JBox> = input
        .trim()
        .split("\n")
        .map(|l| l.trim().parse().unwrap())
        .collect();
    let mut connections: Vec<Connection> = Vec::new();
    for i in 0..jboxes.len() - 1 {
        for j in i + 1..jboxes.len() {
            connections.push(Connection::new(jboxes[i], jboxes[j]));
        }
    }
    connections.sort_by(|a, b| b.distance.cmp(&a.distance));

    let mut graph = Graph::new();
    while let Some(conn) = connections.pop() {
        graph.add(conn);
        if graph.graph.keys().len() == jboxes.len() && graph.get_n_circuits() == 1 {
            return (conn.a.x * conn.b.x) as u64;
        }
    }
    panic!("No soulution found");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_08_part_two() {
        let input = "162,817,812
                                   57,618,57
                                   906,360,560
                                   592,479,940
                                   352,342,300
                                   466,668,158
                                   542,29,236
                                   431,825,988
                                   739,650,466
                                   52,470,668
                                   216,146,977
                                   819,987,18
                                   117,168,530
                                   805,96,715
                                   346,949,466
                                   970,615,88
                                   941,993,340
                                   862,61,35
                                   984,92,344
                                   425,690,689";
        assert_eq!(process(input), 25272);
    }
}
