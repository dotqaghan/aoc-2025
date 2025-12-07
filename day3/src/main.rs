use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/day3.txt").unwrap();

    println!("result: {}", part1(&file));
    println!("result: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let digits = line.as_bytes();

        let mut l_index = 0;
        for i in 0..digits.len()-1 {
            if digits[l_index] < digits[i] {
                l_index = i;
            }
        }

        let mut r_index = l_index + 1;
        for i in l_index +1..digits.len() {
            if digits[r_index] < digits[i] {
                r_index = i;
            }
        }
        let l_value = (digits[l_index] - b'0') as i64;
        let r_value = (digits[r_index] - b'0') as i64;
        let num: i64 = l_value * 10 + r_value;
        sum += num;
    }
    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = line.as_bytes().to_vec();
        let mut l = 0;
        while digits.len() > 12 {
            if l + 2 > digits.len() { break }
            if digits[l] < digits[l + 1] {
                digits.remove(l);
                l = 0;
            } else {
                l += 1;
            }
        }
        let num: i64 = unsafe { str::from_utf8_unchecked(&digits[..12]).parse().unwrap() };
        sum += num;
    }
    sum
}
