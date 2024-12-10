use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_in_map(map: &[Vec<u32>], x: i32, y: i32) -> bool {
    (0..map[0].len() as i32).contains(&x) && (0..map.len() as i32).contains(&y)
}

fn get_reachable_positions(
    map: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    search: u32,
) -> HashSet<(usize, usize)> {
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    if search == 9 {
        if is_in_map(map, x as i32, y as i32 - 1) && map[y - 1][x] == 9 {
            positions.insert((x, y - 1));
        }
        if is_in_map(map, x as i32, y as i32 + 1) && map[y + 1][x] == 9 {
            positions.insert((x, y + 1));
        }
        if is_in_map(map, x as i32 - 1, y as i32) && map[y][x - 1] == 9 {
            positions.insert((x - 1, y));
        }
        if is_in_map(map, x as i32 + 1, y as i32) && map[y][x + 1] == 9 {
            positions.insert((x + 1, y));
        }
        return positions;
    }
    if is_in_map(map, x as i32, y as i32 - 1) && map[y - 1][x] == search {
        get_reachable_positions(map, x, y - 1, search + 1)
            .into_iter()
            .for_each(|position| {
                positions.insert(position);
            });
    }
    if is_in_map(map, x as i32, y as i32 + 1) && map[y + 1][x] == search {
        get_reachable_positions(map, x, y + 1, search + 1)
            .into_iter()
            .for_each(|position| {
                positions.insert(position);
            });
    }
    if is_in_map(map, x as i32 - 1, y as i32) && map[y][x - 1] == search {
        get_reachable_positions(map, x - 1, y, search + 1)
            .into_iter()
            .for_each(|position| {
                positions.insert(position);
            });
    }
    if is_in_map(map, x as i32 + 1, y as i32) && map[y][x + 1] == search {
        get_reachable_positions(map, x + 1, y, search + 1)
            .into_iter()
            .for_each(|position| {
                positions.insert(position);
            });
    }
    positions
}

fn part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let map: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut scores = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != 0 {
                continue;
            }

            scores += get_reachable_positions(&map, x, y, 1).len();
        }
    }
    scores as i32
}

fn count_path(map: &Vec<Vec<u32>>, x: usize, y: usize, search: u32) -> i32 {
    let mut scores = 0;
    if search == 9 {
        if is_in_map(map, x as i32, y as i32 - 1) && map[y - 1][x] == 9 {
            scores += 1;
        }
        if is_in_map(map, x as i32, y as i32 + 1) && map[y + 1][x] == 9 {
            scores += 1;
        }
        if is_in_map(map, x as i32 - 1, y as i32) && map[y][x - 1] == 9 {
            scores += 1;
        }
        if is_in_map(map, x as i32 + 1, y as i32) && map[y][x + 1] == 9 {
            scores += 1;
        }
        return scores;
    }
    if is_in_map(map, x as i32, y as i32 - 1) && map[y - 1][x] == search {
        scores += count_path(map, x, y - 1, search + 1);
    }
    if is_in_map(map, x as i32, y as i32 + 1) && map[y + 1][x] == search {
        scores += count_path(map, x, y + 1, search + 1);
    }
    if is_in_map(map, x as i32 - 1, y as i32) && map[y][x - 1] == search {
        scores += count_path(map, x - 1, y, search + 1);
    }
    if is_in_map(map, x as i32 + 1, y as i32) && map[y][x + 1] == search {
        scores += count_path(map, x + 1, y, search + 1);
    }
    scores
}

fn part2(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let map: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut scores = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != 0 {
                continue;
            }

            scores += count_path(&map, x, y, 1);
        }
    }
    scores
}

fn main() {
    println!("Part 1: {}", part1("assets/day10/input.txt"));
    println!("Part 2: {}", part2("assets/day10/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day10/example.txt"), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day10/example.txt"), 81);
    }
}
