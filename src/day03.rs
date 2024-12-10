use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn part1(path: &str) -> i32 {
    let mut file = File::open(path).unwrap();
    let mut memory = String::new();
    file.read_to_string(&mut memory).unwrap();
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut result = 0;
    for (_, [n1, n2]) in re.captures_iter(memory.as_str()).map(|caps| caps.extract()) {
        result += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    }
    result
}

fn part2(path: &str) -> i32 {
    let mut file = File::open(path).unwrap();
    let mut memory = String::new();
    file.read_to_string(&mut memory).unwrap();
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let mul_re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for (_, [instruction]) in re.captures_iter(memory.as_str()).map(|caps| caps.extract()) {
        if instruction == "do()" {
            enabled = true;
        } else if instruction == "don't()" {
            enabled = false;
        } else if enabled {
            let caps = mul_re.captures(instruction).unwrap();
            result += caps.get(1).unwrap().as_str().parse::<i32>().unwrap() * caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        }
    }
    result
}

fn main() {
    println!("Part 1: {}", part1("assets/day03/input.txt"));
    println!("Part 2: {}", part2("assets/day03/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day03/example1.txt"), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day03/example2.txt"), 48);
    }
}
