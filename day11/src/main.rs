use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/day11.txt").unwrap();
    let data = parse_file(&file);

    let part1 = follow_path(&data, "you", "out", &mut HashMap::new());
    println!("result 1: {part1}");

    let svr_dac = follow_path(&data, "svr", "dac", &mut HashMap::new());
    let dac_fft = follow_path(&data, "dac", "fft", &mut HashMap::new());
    let fft_out = follow_path(&data, "fft", "out", &mut HashMap::new());

    let svr_fft = follow_path(&data, "svr", "fft", &mut HashMap::new());
    let fft_dac = follow_path(&data, "fft", "dac", &mut HashMap::new());
    let dac_out = follow_path(&data, "dac", "out", &mut HashMap::new());

    let part2 = (svr_dac * dac_fft * fft_out) + (svr_fft * fft_dac * dac_out);
    println!("result 2: {part2}");
}

fn parse_file(file: &'_ str) -> HashMap<&'_ str, Vec<&'_ str>> {
    let mut machines = HashMap::new();
    for line in file.lines() {
        let (source, destinations) = line.split_once(':').unwrap();
        let destinations = destinations.trim().split_whitespace().collect();
        machines.insert(source, destinations);
    }
    machines
}

fn follow_path<'a>(
    machines: &HashMap<&'a str, Vec<&'a str>>,
    source: &'a str,
    dest: &'a str,
    cache: &mut HashMap<&'a str, i64>,
) -> i64 {
    if source == dest {
        return 1;
    }

    if cache.contains_key(source) {
        return cache[source];
    }

    if machines.contains_key(source) {
        let count = machines[source]
            .iter()
            .map(|n_source| follow_path(machines, n_source, dest, cache))
            .sum();
        cache.insert(source, count);
        count
    } else {
        0
    }
}
