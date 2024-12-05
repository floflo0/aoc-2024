use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn parse_rules(lines_iter: &mut Lines<BufReader<File>>) -> BTreeMap<i32, Vec<i32>> {
    let mut rules: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    while let Some(Ok(line)) = lines_iter.next() {
        if line.is_empty() {
            break;
        }

        let (nbr1, nbr2) = line.split_once("|").unwrap();
        rules
            .entry(nbr2.parse().unwrap())
            .or_default()
            .push(nbr1.parse().unwrap());
    }
    rules
}
fn part1(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();
    let rules = parse_rules(&mut lines_iter);

    let mut total = 0;

    while let Some(Ok(line)) = lines_iter.next() {
        let update: Vec<i32> = line.split(",").map(|page| page.parse().unwrap()).collect();
        let mut ordered = true;
        for (i, page) in update.iter().enumerate() {
            for p in update[i..].iter() {
                if rules.contains_key(page) && rules[page].contains(p) {
                    ordered = false;
                    break;
                }
            }
            if !ordered {
                break
            }
        }
        if !ordered {
            continue;
        }

        total += update[update.len() / 2];
    }
    total
}

fn part2(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();
    let rules = parse_rules(&mut lines_iter);

    let mut total = 0;

    while let Some(Ok(line)) = lines_iter.next() {
        let mut update: Vec<i32> = line.split(",").map(|page| page.parse().unwrap()).collect();
        let mut ordered = true;
        for (i, page) in update.iter().enumerate() {
            for p in update[i..].iter() {
                if rules.contains_key(page) && rules[page].contains(p) {
                    ordered = false;
                    break;
                }
            }
            if !ordered {
                break
            }
        }
        if ordered {
            continue;
        }

        let mut i = 0;
        while i < update.len() {
            let mut swaped = false;
            for j in i..update.len() {
                if rules.contains_key(&update[i]) && rules[&update[i]].contains(&update[j]) {
                    update.swap(i, j);
                    swaped = true;
                    break;
                }
            }
            if !swaped {
                i += 1;
            }
        }

        total += update[update.len() / 2];
    }
    total
}

fn main() {
    println!("Part 1: {}", part1("assets/day5/input.txt"));
    println!("Part 2: {}", part2("assets/day5/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day5/example.txt"), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day5/example.txt"), 123);
    }
}
