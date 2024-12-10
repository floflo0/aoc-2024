use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_safe(levels: &[i32]) -> bool {
    let decreasing = levels[0] > levels[1];
    for i in 0..(levels.len() - 1) {
        let diff = levels[i] - levels[i + 1];
        if decreasing {
            if !(1..=3).contains(&diff) {
                return false;
            }
        } else if !(-3..=-1).contains(&diff) {
            return false;
        }
    }
    true
}

fn part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut safe_count = 0;
    for line in reader.lines() {
        let line_ = line.unwrap();
        let levels: Vec<i32> = line_
            .split_whitespace()
            .map(|level| level.to_string().parse().unwrap())
            .collect();
        if is_safe(&levels) {
            safe_count += 1;
        }
    }
    safe_count
}

fn part2(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut safe_count = 0;
    for line in reader.lines() {
        let line_ = line.unwrap();
        let levels: Vec<i32> = line_
            .split_whitespace()
            .map(|level| level.to_string().parse().unwrap())
            .collect();
        if is_safe(&levels) {
            safe_count += 1;
        } else {
            for i in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if is_safe(&new_levels) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    safe_count
}

fn main() {
    println!("Part 1: {}", part1("assets/day02/input.txt"));
    println!("Part 2: {}", part2("assets/day02/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day02/example.txt"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day02/example.txt"), 4);
    }
}
