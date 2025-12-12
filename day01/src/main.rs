use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/day01.txt").unwrap();
    let inputs = file
        .lines()
        .map(|line| {
            let n = line[1..].parse::<i64>().unwrap();
            if line.starts_with("R") { n } else { -n }
        })
        .collect::<Vec<i64>>();

    println!("result 1: {}", part1(&inputs));
    println!("result 2: {}", part2(&inputs));
}

fn part1(inputs: &[i64]) -> i64 {
    let mut position = 50;
    let mut count = 0;
    for input in inputs {
        position = (position + input).rem_euclid(100);
        if position == 0 {
            count += 1;
        }
    }
    count
}
fn part2(inputs: &[i64]) -> i64 {
    let mut position = 50;
    let mut count = 0;
    for &input in inputs {
        let rotations = if input < 0 {
            (100 - position) % 100 - input
        } else {
            position + input
        } / 100;
        count += rotations;
        position = (position + input).rem_euclid(100);
    }
    count
}
