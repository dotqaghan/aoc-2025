use day9::{IRect, IVec2};
use std::fs;
use std::process::Command;
const FILE_NAME: &str = "inputs/day9.txt";

fn main() {
    let file = fs::read_to_string(FILE_NAME).unwrap();
    let coords = parse_file(file);

    part1(&coords);
    part2();
}

fn parse_file(file: String) -> Vec<IVec2> {
    let mut coords: Vec<IVec2> = Vec::with_capacity(file.lines().count());
    file.lines().for_each(|x| {
        let (x, y) = x.split_once(',').unwrap();
        coords.push(IVec2::new(x.parse().unwrap(), y.parse().unwrap()));
    });
    coords
}

fn part1(coords: &Vec<IVec2>) {
    let mut areas: Vec<(IRect, i64)> = vec![];
    for i in coords {
        for j in coords {
            let rect = IRect {
                a: *i,
                b: IVec2 { x: j.x, y: i.y },
                c: *j,
                d: IVec2 { x: i.x, y: j.y },
            };
            let area = rect.area();
            areas.push((rect, area));
        }
    }

    areas.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("result 1: {:?}", areas[areas.len() - 1].1);
}

fn part2() {
    // I am sorry
    if Command::new(".venv/bin/python")
        .arg("day9/src/part2.py")
        .arg(FILE_NAME)
        .output()
        .is_err()
    {
        panic!("python venv not found");
    }
}
