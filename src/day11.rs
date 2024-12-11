use std::fs::File;
use std::io::prelude::*;

use cached::proc_macro::cached;

fn part1(path: &str) -> usize {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut stones: Vec<u64> = content
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();
    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            if stones[i] == 0 {
                stones[i] = 1;
            } else if stones[i].to_string().len() % 2 == 0 {
                let stone_str = stones[i].to_string();
                let (stone1, stone2) = stone_str.split_at(stone_str.len() / 2);
                stones[i] = stone1.parse().unwrap();
                stones.insert(i + 1, stone2.parse().unwrap());
                i += 1;
            } else {
                stones[i] *= 2024;
            }
            i += 1;
        }
    }
    stones.len()
}

#[cached]
fn get_nbr_stones(stone: u64, blinks_times: u8) -> usize {
    if blinks_times == 0 {
        return 1;
    }

    if stone == 0 {
        return get_nbr_stones(1, blinks_times - 1);
    }

    let stone_str = stone.to_string();
    let stone_str_len = stone_str.len();
    if stone_str_len % 2 == 0 {
        let (stone1, stone2) = stone_str.split_at(stone_str.len() / 2);
        return get_nbr_stones(stone1.parse().unwrap(), blinks_times - 1)
            + get_nbr_stones(stone2.parse().unwrap(), blinks_times - 1);
    }

    return get_nbr_stones(stone * 2024, blinks_times - 1);
}

fn solve(path: &str, blinks_times: u8) -> usize {
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
        .split_whitespace()
        .map(|stone| get_nbr_stones(stone.parse().unwrap(), blinks_times))
        .sum()
}

fn part2(path: &str) -> usize {
    solve(path, 75)
}

fn main() {
    println!("Part 1: {}", part1("assets/day11/input.txt"));
    println!("Part 2: {}", part2("assets/day11/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day11/example.txt"), 55312);
        assert_eq!(part1("assets/day11/input.txt"), 200446);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve("assets/day11/example.txt", 25), 55312);
        assert_eq!(solve("assets/day11/input.txt", 25), 200446);
    }
}
