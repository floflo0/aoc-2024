use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();
    reader.lines().for_each(|line| {
        let line_ = line.unwrap();
        let mut iter = line_.split_whitespace();
        left_column.push(iter.next().unwrap().to_string().parse().unwrap());
        right_column.push(iter.next().unwrap().to_string().parse().unwrap());
    });
    (left_column, right_column)
}

fn part1(path: &str) -> i32 {
    let (mut left_column, mut right_column) = parse(path);
    left_column.sort();
    right_column.sort();
    let mut total_distance = 0;
    for (left, right) in left_column.iter().zip(right_column.iter()) {
        total_distance += i32::abs(left - right);
    }
    total_distance
}

fn part2(path: &str) -> i32 {
    let (left_column, right_column) = parse(path);
    let mut cache: BTreeMap<i32, i32> = BTreeMap::new();
    let mut total = 0;
    for left in left_column.iter() {
        if !cache.contains_key(left) {
            let mut count = 0;
            for right in right_column.iter() {
                if right == left {
                    count += 1;
                }
            }
            cache.insert(*left, count * left);
        }

        total += cache.get(left).unwrap();
    }
    total
}

fn main() {
    println!("Part 1: {}", part1("assets/day01/input.txt"));
    println!("Part 2: {}", part2("assets/day01/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day01/example.txt"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day01/example.txt"), 31);
    }
}
