use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs;

struct Machine {
    target_indicators: i64,
    buttons: Vec<i64>,
    joltage_requirements: Vec<i64>,
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Target indicators: |{:010b}|\nButtons: ", self.target_indicators)?;
        for button in &self.buttons {
            write!(f, "{button:010b} ")?;
        }
        write!(f, "\nJoltage requirements: {:?}", self.joltage_requirements)?;
        Ok(())
    }
}

fn main() {
    let file = fs::read_to_string("inputs//day10.txt").unwrap();
    let machines = parse_file(file);

    println!("result 1: {:?}", part1(&machines));
}

fn parse_file(file: String) -> Vec<Machine> {
    let lines = file
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut machines = vec![];
    for line in lines {
        let targeted_indicators = &line[0][1..line[0].len() - 1]
            .chars()
            .enumerate()
            .fold(0, |acc, (i, c)| match c {
                '#' => acc | (1 << i),
                _ => acc
            });
        let joltage_requirements = &line[line.len() - 1][1..line[line.len() - 1].len() - 1]
            .split(",")
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let buttons = &line[1..line.len() - 1]
            .iter()
            .map(|&x| {
                (&x[1..x.len() - 1])
                    .split(',')
                    .map(|num| num.parse::<i64>().unwrap())
                    .fold(0, |acc, num| acc | (1 << num))
            })
            .collect::<Vec<_>>();

        machines.push(Machine {
            target_indicators: targeted_indicators.clone(),
            joltage_requirements: joltage_requirements.clone(),
            buttons: buttons.clone(),
        });
    }
    machines
}

fn part1(machines: &Vec<Machine>) -> usize {
    let mut total_presses: usize = 0;

    machines.iter().for_each(|machine| {
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        let mut visited = HashSet::new();


        while let Some((state, dist)) = queue.pop_front() {
            if state == machine.target_indicators {
                total_presses += dist;
            }

            for button in &machine.buttons {
                let next = state ^ button;
                if visited.insert(next) {
                    queue.push_back((next, dist + 1));
                }
            }
        }
        println!("{}", machine);
        println!()
    });
    total_presses
}