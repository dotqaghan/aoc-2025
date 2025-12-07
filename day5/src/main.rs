use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/day5.txt").unwrap();
    let (ranges, ingredients) = file.split_once("\n\n").unwrap();

    println!("result 1: {}", part1(ranges, ingredients));
    println!("result 2: {}", part2(ranges, ingredients));
}

fn part1(ranges: &str, ingredients: &str) -> u64 {
    let mut count = 0;
    for ingredient in ingredients.lines() {
        let id = ingredient.parse::<u64>().unwrap();
        for range in ranges.lines() {
            let (first, second) = range.split_once('-').unwrap();
            let (lower, upper) = (first.parse::<u64>().unwrap(), second.parse::<u64>().unwrap());
            if lower <= id && id <= upper {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part2(ranges: &str, ingredients: &str) -> u64 {
    let mut lowers = Vec::new();
    let mut uppers = Vec::new();
    for _ in ingredients.lines() {
        for range in ranges.lines() {
            let (first, second) = range.split_once('-').unwrap();
            let (lower, upper): (u64, u64) = (first.parse().unwrap(), second.parse().unwrap());
            lowers.push(lower);
            uppers.push(upper);
        }
    }

    lowers.sort_unstable();
    uppers.sort_unstable();

    let mut merged = Vec::new();
    let mut search = 0..0;

    for i in 0..uppers.len() {
        if lowers[i] < search.end {
            search.end = search.end.max(uppers[i] + 1);
        } else {
            merged.push(search);
            search = lowers[i]..uppers[i] + 1;
        }
    }
    merged.push(search);

    merged.iter().map(|range| range.end - range.start).sum::<u64>()
}