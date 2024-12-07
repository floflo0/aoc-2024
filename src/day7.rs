use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_equation_part1(x: u64, expected: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return x == expected;
    }
    solve_equation_part1(x + numbers[0], expected, &numbers[1..])
        || solve_equation_part1(x * numbers[0], expected, &numbers[1..])
}

fn part1(path: &str) -> u64 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (expected, numbers) = line.split_once(": ").unwrap();
        let expected: u64 = expected.parse().unwrap();
        let numbers: Vec<u64> = numbers
            .split_whitespace()
            .map(|nbr| nbr.parse().unwrap())
            .collect();
        if solve_equation_part1(numbers[0], expected, &numbers[1..]) {
            total += expected;
        }
    }
    total
}

fn concat(x1: u64, x2: u64) -> u64 {
    let mut p = 10;
    while x2 >= p {
        p *= 10;
    }
    x1 * p + x2
}

fn solve_equation_part2(x: u64, expected: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return x == expected;
    }
    solve_equation_part2(x + numbers[0], expected, &numbers[1..])
        || solve_equation_part2(x * numbers[0], expected, &numbers[1..])
        || solve_equation_part2(concat(x, numbers[0]), expected, &numbers[1..])
}

fn part2(path: &str) -> u64 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (expected, numbers) = line.split_once(": ").unwrap();
        let expected: u64 = expected.parse().unwrap();
        let numbers: Vec<u64> = numbers
            .split_whitespace()
            .map(|nbr| nbr.parse().unwrap())
            .collect();
        if solve_equation_part2(numbers[0], expected, &numbers[1..]) {
            total += expected;
        }
    }
    total
}

fn main() {
    println!("Part 1: {}", part1("assets/day7/input.txt"));
    println!("Part 2: {}", part2("assets/day7/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day7/example.txt"), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day7/example.txt"), 11387);
    }
}
