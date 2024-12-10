use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_in_map((x, y): &(i32, i32), width: usize, height: usize) -> bool {
    (0..width as i32).contains(x) && (0..height as i32).contains(y)
}

fn part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let map: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, cell)| {
            if *cell != b'.' {
                antennas.entry(*cell).or_default().push((x, y));
            }
        })
    });
    for frequency in antennas.values() {
        for i in 0..(frequency.len() - 1) {
            for j in (i + 1)..(frequency.len()) {
                let dx: i32 = frequency[j].0 as i32 - frequency[i].0 as i32;
                let dy: i32 = frequency[j].1 as i32 - frequency[i].1 as i32;
                let antinode1 = (frequency[j].0 as i32 + dx, frequency[j].1 as i32 + dy);
                let antinode2 = (frequency[i].0 as i32 - dx, frequency[i].1 as i32 - dy);
                if is_in_map(&antinode1, map[0].len(), map.len()) {
                    antinodes.insert(antinode1);
                }
                if is_in_map(&antinode2, map[0].len(), map.len()) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    antinodes.len() as i32
}

fn part2(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let map: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, cell)| {
            if *cell != b'.' {
                antennas.entry(*cell).or_default().push((x, y));
            }
        })
    });
    for frequency in antennas.values() {
        for i in 0..(frequency.len() - 1) {
            for j in (i + 1)..(frequency.len()) {
                let dx: i32 = frequency[j].0 as i32 - frequency[i].0 as i32;
                let dy: i32 = frequency[j].1 as i32 - frequency[i].1 as i32;
                antinodes.insert((frequency[i].0 as i32, frequency[i].1 as i32));
                antinodes.insert((frequency[j].0 as i32, frequency[j].1 as i32));
                let mut antinode = (frequency[j].0 as i32 + dx, frequency[j].1 as i32 + dy);
                while is_in_map(&antinode, map[0].len(), map.len()) {
                    antinodes.insert(antinode);
                    antinode = (antinode.0 + dx, antinode.1 + dy);
                }
                antinode = (frequency[i].0 as i32 - dx, frequency[i].1 as i32 - dy);
                while is_in_map(&antinode, map[0].len(), map.len()) {
                    antinodes.insert(antinode);
                    antinode = (antinode.0 - dx, antinode.1 - dy);
                }
            }
        }
    }
    antinodes.len() as i32
}

fn main() {
    println!("Part 1: {}", part1("assets/day08/input.txt"));
    println!("Part 2: {}", part2("assets/day08/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day08/example.txt"), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day08/example.txt"), 34);
    }
}
