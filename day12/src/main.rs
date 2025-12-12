use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/day12.txt").unwrap();
    let input = parse_file(&file);
    let result = part1(&input);

    println!("result 1: {result}");
}

fn parse_file(file: &str) -> Vec<bool> {
    file.lines()
        .skip(30)
        .map(|line| {
            line.split_once(':')
                .map(|(size, count)| {
                    let (l, r) = size.split_once('x').unwrap();
                    let area = l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap();
                    let sum = count
                        .trim()
                        .split_whitespace()
                        .map(|count| count.parse::<i32>().unwrap())
                        .sum::<i32>();
                    area > sum * 7
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &Vec<bool>) -> i32 {
    let count = input
        .iter()
        .fold(0, |acc, &x| if x { acc + 1 } else { acc });
    count
}
