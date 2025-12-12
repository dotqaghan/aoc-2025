use dashmap::DashMap;
use std::{fs, sync::Arc};

fn main() {
    let file = fs::read_to_string("inputs/day7.txt").unwrap();
    let input: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    println!("result 1: {}", part1(input.clone()));
    println!("result 2: {}", part2(input, Arc::new(DashMap::new())) + 1);
}

fn part2(mut input: Vec<Vec<char>>, cache: Arc<DashMap<Vec<String>, i64>>) -> i64 {
    let key: Vec<String> = input.iter().map(|row| row.iter().collect()).collect();
    if let Some(count) = cache.get(&key) {
        return *count;
    }

    let mut count = 0;

    for i in 0..input.len() - 1 {
        for j in 0..input[i].len() {
            if input[i][j] == 'S' {
                input[i + 1][j] = '|';
            }
            if input[i][j] == '|' {
                if input[i + 1][j] == '^' {
                    let mut l_copy = input[i + 1..].to_owned();
                    l_copy[0][j + 1] = '|';
                    let mut r_copy = input[i + 1..].to_owned();
                    r_copy[0][j - 1] = '|';
                    let (l_result, r_result) = rayon::join(
                        || part2(l_copy, cache.clone()),
                        || part2(r_copy, cache.clone()),
                    );
                    count += l_result + r_result + 1;
                }
                if input[i + 1][j] == '.' {
                    input[i + 1][j] = '|';
                }
            }
        }
    }
    cache.insert(key, count);
    count
}

fn part1(mut input: Vec<Vec<char>>) -> i64 {
    let length = input.len();
    let mut count = 0;
    for i in 0..length - 1 {
        for j in 0..input[i].len() {
            if input[i][j] == 'S' {
                input[i + 1][j] = '|';
            }
            if input[i][j] == '|' {
                if input[i + 1][j] == '^' {
                    input[i + 1][j + 1] = '|';
                    input[i + 1][j - 1] = '|';
                    count += 1;
                }
                if input[i + 1][j] == '.' {
                    input[i + 1][j] = '|';
                }
            }
        }
    }
    count
}
