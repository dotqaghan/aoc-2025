use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/day6.txt").unwrap();

    println!("result 1: {}", part1(&file));
    println!("result 2: {}", part2(&file).iter().sum::<i64>());
}

fn part1(input: &str) -> i64 {
    let parsed: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();
    let numbers: Vec<Vec<i64>> = (0..parsed[0].len())
        .map(|col| {
            (0..parsed.len() - 1)
                .map(|row| parsed[row][col].parse().unwrap())
                .collect()
        })
        .collect();

    numbers
        .iter()
        .enumerate()
        .map(|(i, v)| match parsed[parsed.len() - 1][i] {
            "*" => v.iter().product::<i64>(),
            "+" => v.iter().sum::<i64>(),
            _ => unreachable!(),
        })
        .sum()
}
fn part2(input: &str) -> Vec<i64> {
    let parsed: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let operators = &parsed[parsed.len() - 1];
    let numbers: Vec<String> = (0..parsed[0].len())
        .map(|col| unsafe {
            let x = (0..parsed.len() - 1)
                .map(|row| parsed[row][col])
                .collect::<Vec<_>>();
            str::from_utf8_unchecked(&x).trim().to_string()
        })
        .collect();

    let operators: Vec<_> = operators.iter().filter(|&&x| x != 32).collect();
    let numbers = numbers.split(|x| x.is_empty()).collect::<Vec<&[String]>>();

    let mut current_op = operators[0];
    let results: Vec<i64> = numbers
        .iter()
        .enumerate()
        .map(|(i, v)| {
            current_op = match operators[i] {
                b'*' => &b'*',
                b'+' => &b'+',
                _ => current_op,
            };
            match current_op {
                b'*' => v.iter().map(|x| x.parse::<i64>().unwrap()).product(),
                b'+' => v.iter().map(|x| x.parse::<i64>().unwrap()).sum(),
                _ => unreachable!(),
            }
        })
        .collect();
    results
}
