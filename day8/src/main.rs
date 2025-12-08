#![feature(array_try_from_fn)]

use day8::{Forest, IVec3, Node, distance_between};
use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/day8.txt").unwrap();
    println!("result 1: {}", part1(&file, 1000));
    println!("result 2: {}", part2(&file));
}

fn parse_file(file: &str) -> (Vec<IVec3>, Vec<((usize, usize), f64)>, Forest) {
    let positions: Vec<IVec3> = file
        .lines()
        .map(|line| line.splitn(3, ","))
        .map(|mut split| std::array::try_from_fn(|_| split.next()).unwrap())
        .map(|[x, y, z]| IVec3 {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        })
        .collect();

    let forest = Forest::with_size(positions.len());

    let mut distances: Vec<((usize, usize), f64)> = Vec::with_capacity(positions.len() * (positions.len() - 1));
    for a in 0..positions.len() {
        for b in a + 1..positions.len() {
            distances.push(((a, b), distance_between(&positions[a], &positions[b])))
        }
    }
    distances.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    (positions, distances, forest)
}

fn part1(file: &str, max_connections: i64) -> usize {
    let (junction_boxes, distances, mut forest) = parse_file(file);

    let mut connected = 0;
    for ((a, b), _) in distances {
        match (forest.get_parent(a), forest.get_parent(b)) {
            (None, None) => {
                forest[a] = Node::Root;
                forest[b] = Node::Parent(a);
            }
            (Some(p), None) => forest[b] = Node::Parent(p),
            (None, Some(p)) => forest[a] = Node::Parent(p),
            (Some(p_a), Some(p_b)) => {
                if p_a != p_b {
                    forest[p_b] = Node::Parent(p_a)
                }
            }
        }
        connected += 1;

        if connected == max_connections {
            break;
        }
    }
    let mut counts = HashMap::new();

    for i in 0..junction_boxes.len() {
        match forest.get_parent(i) {
            Some(i_val) => {
                counts
                    .entry(i_val)
                    .and_modify(|count| *count += 1)
                    .or_insert(1usize);
            }
            None => continue,
        }
    }

    let mut values = counts.values().map(|&count| count).collect::<Vec<_>>();
    values.sort_unstable();

    values.iter().rev().take(3).product()
}

fn part2(file: &str) -> i64 {
    let (junction_boxes, distances, mut forest) = parse_file(file);
    let mut circuit_count = 0;
    let mut disjoint_count = junction_boxes.len();
    for ((a, b), _) in distances {
        let parent_a = forest.get_parent(a);
        let parent_b = forest.get_parent(b);

        match (parent_a, parent_b) {
            (None, None) => {
                forest[a] = Node::Root;
                forest[b] = Node::Parent(a);
                circuit_count += 1;
                disjoint_count -= 2;
            }
            (Some(p), None) => {
                forest[b] = Node::Parent(p);
                disjoint_count -= 1;
            }
            (None, Some(p)) => {
                forest[a] = Node::Parent(p);
                disjoint_count -= 1;
            }
            (Some(p_a), Some(p_b)) => {
                if p_a != p_b {
                    forest[p_b] = Node::Parent(p_a);
                    circuit_count -= 1;
                }
            }
        }

        if disjoint_count == 0 && circuit_count == 1 {
            return junction_boxes[a].x * junction_boxes[b].x;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let file = fs::read_to_string("../inputs/examples/day8.txt").unwrap();
        assert_eq!(part1(&file, 10), 40);
    }
    #[test]
    fn part1_full() {
        let file = fs::read_to_string("../inputs/day8.txt").unwrap();
        assert_eq!(part1(&file, 1000), 57564);
    }

    #[test]
    fn part2_example() {
        let file = fs::read_to_string("../inputs/examples/day8.txt").unwrap();
        assert_eq!(part2(&file), 25272);
    }

    #[test]
    fn part2_full() {
        let file = fs::read_to_string("../inputs/day8.txt").unwrap();
        assert_eq!(part2(&file), 133296744);
    }
}