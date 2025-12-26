use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
    fn dist(self: Self, other: JBox) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct KDTNode {
    value: JBox,
    axis: Axis,
    left: Option<usize>,
    right: Option<usize>,
}

impl KDTNode {
    fn distance(self: &Self, jbox: &JBox) -> i64 {
        let dist = match self.axis {
            Axis::X => ((self.value.x as i64) - (jbox.x as i64)).pow(2),
            Axis::Y => ((self.value.y as i64) - (jbox.y as i64)).pow(2),
            Axis::Z => ((self.value.z as i64) - (jbox.z as i64)).pow(2),
        };
        dist
    }

    fn jbox_cmp(&self, jbox: &JBox) -> std::cmp::Ordering {
        match self.axis {
            Axis::X => i64::cmp(&self.value.x, &jbox.x),
            Axis::Y => i64::cmp(&self.value.y, &jbox.y),
            Axis::Z => i64::cmp(&self.value.z, &jbox.z),
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, Eq, Hash, Ord)]
struct Connection(JBox, JBox);

impl Connection {
    fn new(jbox: JBox, jbox1: JBox) -> Self {
        if jbox > jbox1 {
            return Self(jbox, jbox1);
        } else {
            return Self(jbox1, jbox);
        }
    }
}

#[derive(Ord, PartialOrd, PartialEq, Debug, Copy, Clone, Eq)]
struct QNode {
    dist: i64,
    connection: Connection,
}

type Graph = HashMap<JBox, Vec<JBox>>;

fn build_tree(boxes: &mut [JBox], axis: Axis, nodes: &mut Vec<KDTNode>) -> Option<usize> {
    if boxes.len() == 0 {
        return None;
    }
    let next_axis = match axis {
        Axis::X => {
            boxes.sort_by(|a, b| a.x.cmp(&b.x));
            Axis::Y
        }
        Axis::Y => {
            boxes.sort_by(|a, b| a.y.cmp(&b.y));
            Axis::Z
        }
        Axis::Z => {
            boxes.sort_by(|a, b| a.z.cmp(&b.z));
            Axis::X
        }
    };

    let middle_index = boxes.len() / 2;
    let node_box = boxes[middle_index];
    let node_left = build_tree(&mut boxes[..middle_index], next_axis, nodes);
    let node_right = build_tree(&mut boxes[middle_index + 1..], next_axis, nodes);

    nodes.push(KDTNode {
        value: node_box,
        left: node_left,
        right: node_right,
        axis,
    });

    Some(nodes.len() - 1)
}

fn search(
    nodes: &[KDTNode],
    node_index: Option<usize>,
    jbox: JBox,
    heap: &mut BinaryHeap<QNode>,
    visited: &mut HashSet<Connection>,
    max_capacity: usize,
) {
    let node = match node_index {
        Some(index) => &nodes[index],
        None => return,
    };

    match node.jbox_cmp(&jbox) {
        Ordering::Less | Ordering::Equal => {
            search(nodes, node.right, jbox, heap, visited, max_capacity)
        }
        Ordering::Greater => search(nodes, node.left, jbox, heap, visited, max_capacity),
    };

    let distance = jbox.dist(node.value);
    if distance > 0 {
        let connection = Connection::new(jbox, node.value);
        if !visited.contains(&connection) {
            heap.push(QNode {
                connection,
                dist: distance,
            });
            visited.insert(connection);
        }
    }

    if heap.len() > max_capacity {
        heap.pop();
    }

    // Check wether other branch is worth visiting
    if heap.len() < max_capacity || node.distance(&jbox) < heap.peek().unwrap().dist {
        match node.jbox_cmp(&jbox) {
            Ordering::Less | Ordering::Equal => {
                search(nodes, node.left, jbox, heap, visited, max_capacity)
            }
            Ordering::Greater => search(nodes, node.right, jbox, heap, visited, max_capacity),
        };
    }
}

fn get_circuits(closest_connections: &mut BinaryHeap<QNode>) -> Vec<HashSet<JBox>> {
    let mut graph: Graph = HashMap::new();
    while let Some(node) = closest_connections.pop() {
        let Connection(a, b) = node.connection;
        graph.entry(a).or_default().push(b);
        graph.entry(b).or_default().push(a);
    }

    let mut circuits: Vec<HashSet<JBox>> = Vec::new();
    let mut visited: HashSet<JBox> = HashSet::new();
    for node in graph.keys() {
        if visited.contains(node) {
            continue;
        }
        let mut circuit: HashSet<JBox> = HashSet::new();
        let mut stack = vec![node];

        while let Some(&curr) = stack.pop() {
            if !visited.contains(&curr) {
                circuit.insert(curr);
                visited.insert(curr);
                for adj in &graph[&curr] {
                    stack.push(adj);
                }
            }
        }
        circuits.push(circuit);
    }
    println!("Total nodes {}", visited.len());
    circuits
}
fn get_top_3(circuits: impl IntoIterator<Item = HashSet<JBox>>) -> u64 {
    let mut circuits_sizes: Vec<u64> = circuits.into_iter().map(|c| c.len() as u64).collect();
    circuits_sizes.sort();
    println!("Circuit sizes {circuits_sizes:?}");
    circuits_sizes
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(i, _)| *i < 3)
        .map(|(_, s)| s)
        .product()
}

pub fn process(input: &'static str, max_capacity: usize) -> u64 {
    let mut jboxes: Vec<JBox> = input
        .trim()
        .split("\n")
        .map(|l| l.trim().parse().unwrap())
        .collect();

    let mut nodes: Vec<KDTNode> = Vec::new();
    let root_node_index = build_tree(jboxes.as_mut_slice(), Axis::X, &mut nodes);
    let mut max_heap: BinaryHeap<QNode> = BinaryHeap::new();
    let mut visited: HashSet<Connection> = HashSet::new();
    for node in &nodes {
        search(
            nodes.as_slice(),
            root_node_index,
            node.value,
            &mut max_heap,
            &mut visited,
            max_capacity,
        );
    }
    let circuits = get_circuits(&mut max_heap);
    let top_3_sizes = get_top_3(circuits);
    top_3_sizes
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn day_08_part_one() {
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
        assert_eq!(process(input, 10), 40);
    }
}
