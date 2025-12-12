use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs;

struct Machine {
    target_indicators: i32,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<i32>,
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Target indicators: |{:010b}|\nButtons: ",
            self.target_indicators
        )?;
        for button in &self.buttons {
            let button = button.iter().fold(0, |acc, num| acc | (1 << num));
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
    println!("result 2: {:?}", part2(&machines));
}

fn parse_file(file: String) -> Vec<Machine> {
    let lines = file
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut machines = vec![];
    for line in lines {
        let targeted_indicators =
            &line[0][1..line[0].len() - 1]
                .chars()
                .enumerate()
                .fold(0, |acc, (i, c)| match c {
                    '#' => acc | (1 << i),
                    _ => acc,
                });
        let joltage_requirements = &line[line.len() - 1][1..line[line.len() - 1].len() - 1]
            .split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let buttons = &line[1..line.len() - 1]
            .iter()
            .map(|&x| {
                (&x[1..x.len() - 1])
                    .split(',')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
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

fn part1(machines: &Vec<Machine>) -> i32 {
    let mut total_presses = 0;

    machines.iter().for_each(|machine| {
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        let mut visited = HashSet::new();
        let mut press_count = 0;

        while let Some((state, dist)) = queue.pop_front() {
            if state == machine.target_indicators {
                press_count += dist;
            }

            for button in &machine.buttons {
                let button = button.iter().fold(0, |acc, num| acc | (1 << num));
                let next = state ^ button;
                if visited.insert(next) {
                    queue.push_back((next, dist + 1));
                }
            }
        }
        println!("{}", machine);
        println!("Needs at least {} presses", press_count);
        println!();
        total_presses += press_count;
    });
    total_presses
}

fn part2(machines: &Vec<Machine>) -> i32 {
    let mut result = 0;
    for machine in machines {
        let mut problem_vars = variables!();
        let vars: Vec<Variable> = (0..machine.buttons.len())
            .map(|_| problem_vars.add(variable().min(0).integer()))
            .collect();

        let objective: Expression = vars.iter().sum();
        let mut model = problem_vars.minimise(objective).using(default_solver);

        for (i, &target_value) in machine.joltage_requirements.iter().enumerate() {
            let relevant_vars: Vec<Variable> = machine
                .buttons
                .iter()
                .enumerate()
                .filter(|(_, indices)| indices.contains(&i))
                .map(|(button, _)| vars[button])
                .collect();

            let constraint_sum: Expression = relevant_vars.iter().sum();
            model.add_constraint(constraint_sum.eq(target_value));
        }

        let solution = model.solve().unwrap();
        let current_sum: f64 = vars.iter().map(|&var| solution.value(var)).sum();
        result += current_sum.round() as i32;
    }
    result
}
