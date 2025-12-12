use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/day04.txt").unwrap();
    let mut data: Vec<Vec<i32>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| if ch == '@' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let result1 = part1(&data);
    let result2 = part2(&mut data);
    println!("result 1: {}", result1);
    println!("result 2: {}", result2);
}

fn count_surrounding(data: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    let x_max = data[y].len() - 1;
    let y_max = data.len() - 1;

    count += if x > 0 && y > 0 {
        data[y - 1][x - 1]
    } else {
        0
    };
    count += if x > 0 { data[y][x - 1] } else { 0 };
    count += if x > 0 && y < y_max {
        data[y + 1][x - 1]
    } else {
        0
    };

    count += if y > 0 { data[y - 1][x] } else { 0 };
    count += if y < y_max { data[y + 1][x] } else { 0 };

    count += if x < x_max && y > 0 {
        data[y - 1][x + 1]
    } else {
        0
    };
    count += if x < x_max { data[y][x + 1] } else { 0 };
    count += if x < x_max && y < y_max {
        data[y + 1][x + 1]
    } else {
        0
    };

    count
}

fn part1(data: &Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    for j in 0..data.len() {
        for i in 0..data[j].len() {
            if data[j][i] == 0 {
                print!(".");
                continue;
            }
            if count_surrounding(data, i, j) < 4 {
                print!("x");
                answer += 1;
            } else {
                print!("@");
            }
        }
        println!();
    }
    answer
}

fn part2(data: &mut Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for j in 0..data.len() {
            for i in 0..data[j].len() {
                if data[j][i] == 0 {
                    print!(".");
                    continue;
                }

                if count_surrounding(data, i, j) < 4 {
                    print!("x");
                    changed = true;
                    data[j][i] = 0;
                    answer += 1;
                } else {
                    print!("@");
                }
            }
            println!();
        }
        println!();
    }
    answer
}
