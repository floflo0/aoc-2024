use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let width = lines[0].len();
    let height = lines.len();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if lines[y].chars().nth(x).unwrap() == 'X' {
                if x < width - 3 &&
                    lines[y].chars().nth(x + 1).unwrap() == 'M' &&
                    lines[y].chars().nth(x + 2).unwrap() == 'A' &&
                    lines[y].chars().nth(x + 3).unwrap() == 'S' {
                    count += 1;
                }
                if x > 2 &&
                    lines[y].chars().nth(x - 1).unwrap() == 'M' &&
                    lines[y].chars().nth(x - 2).unwrap() == 'A' &&
                    lines[y].chars().nth(x - 3).unwrap() == 'S' {
                    count += 1;
                }
                if y < height - 3 &&
                    lines[y + 1].chars().nth(x).unwrap() == 'M' &&
                    lines[y + 2].chars().nth(x).unwrap() == 'A' &&
                    lines[y + 3].chars().nth(x).unwrap() == 'S' {
                    count += 1;
                }
                if y > 2 &&
                    lines[y - 1].chars().nth(x).unwrap() == 'M' &&
                    lines[y - 2].chars().nth(x).unwrap() == 'A' &&
                    lines[y - 3].chars().nth(x).unwrap() == 'S' {
                    count += 1;
                }
                if x < width - 3 &&
                    y < height - 3 &&
                    lines[y + 1].chars().nth(x + 1).unwrap() == 'M' &&
                    lines[y + 2].chars().nth(x + 2).unwrap() == 'A' &&
                    lines[y + 3].chars().nth(x + 3).unwrap() == 'S' {
                    count += 1;
                }
                if x < width - 3 &&
                    y > 2 &&
                    lines[y - 1].chars().nth(x + 1).unwrap() == 'M' &&
                    lines[y - 2].chars().nth(x + 2).unwrap() == 'A' &&
                    lines[y - 3].chars().nth(x + 3).unwrap() == 'S' {
                    count += 1;
                }
                if x > 2 &&
                    y > 2 &&
                    lines[y - 1].chars().nth(x - 1).unwrap() == 'M' &&
                    lines[y - 2].chars().nth(x - 2).unwrap() == 'A' &&
                    lines[y - 3].chars().nth(x - 3).unwrap() == 'S' {
                    count += 1;
                }
                if x > 2 &&
                    y < height - 3 &&
                    lines[y + 1].chars().nth(x - 1).unwrap() == 'M' &&
                    lines[y + 2].chars().nth(x - 2).unwrap() == 'A' &&
                    lines[y + 3].chars().nth(x - 3).unwrap() == 'S' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let width = lines[0].len();
    let height = lines.len();
    let mut count = 0;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if lines[y].chars().nth(x).unwrap() == 'A' {
                if  lines[y - 1].chars().nth(x - 1).unwrap() == 'M' &&
                    lines[y + 1].chars().nth(x + 1).unwrap() == 'S' &&
                    (lines[y - 1].chars().nth(x + 1).unwrap() == 'M' &&
                     lines[y + 1].chars().nth(x - 1).unwrap() == 'S' ||
                     lines[y - 1].chars().nth(x + 1).unwrap() == 'S' &&
                     lines[y + 1].chars().nth(x - 1).unwrap() == 'M') {
                    count += 1;
                }
                if  lines[y - 1].chars().nth(x - 1).unwrap() == 'S' &&
                    lines[y + 1].chars().nth(x + 1).unwrap() == 'M' &&
                    (lines[y - 1].chars().nth(x + 1).unwrap() == 'M' &&
                     lines[y + 1].chars().nth(x - 1).unwrap() == 'S' ||
                     lines[y - 1].chars().nth(x + 1).unwrap() == 'S' &&
                     lines[y + 1].chars().nth(x - 1).unwrap() == 'M') {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    println!("Part 1: {}", part1("assets/day4/input.txt"));
    println!("Part 2: {}", part2("assets/day4/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day4/example.txt"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day4/example.txt"), 9);
    }
}
