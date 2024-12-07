use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right
}

impl Orientation {
    fn rotate(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    orientation: Orientation
}

impl Position {
    fn next_position(&self) -> Self {
        match self.orientation {
            Orientation::Down  => Self { x: self.x,     y: self.y + 1, orientation: self.orientation.clone() },
            Orientation::Up    => Self { x: self.x,     y: self.y - 1, orientation: self.orientation.clone() },
            Orientation::Left  => Self { x: self.x - 1, y: self.y,     orientation: self.orientation.clone() },
            Orientation::Right => Self { x: self.x + 1, y: self.y,     orientation: self.orientation.clone() }
        }
    }

    fn is_in_grid(&self, width: usize , height: usize) -> bool {
        (0..width as i32).contains(&self.x) && (0..height as i32).contains(&self.y)
    }

    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn parse_map(path: &str) -> Vec<Vec<u8>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap().as_bytes().to_vec()).collect()
}

fn get_guard_initial_position(map: &[Vec<u8>]) -> Option<Position> {

    for (y, line) in map.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            if *cell == b'^' {
                return Some(Position { x: x as i32, y: y as i32, orientation: Orientation::Up });
            }
        }
    }
    None
}

fn get_viewed_positions(map: &[Vec<u8>]) -> Vec<(i32, i32)> {
    let width = map[0].len();
    let height = map.len();
    let mut position = get_guard_initial_position(map).unwrap();
    let mut viewed_positions: Vec<(i32, i32)> = Vec::new();

    while position.is_in_grid(width, height) {
        if !viewed_positions.contains(&position.to_tuple()) {
            viewed_positions.push(position.to_tuple());
        }

        let mut next_position = position.next_position();
        while next_position.is_in_grid(width, height) &&
            map[next_position.y as usize][next_position.x as usize] == b'#' {
            position.orientation.rotate();
            next_position = position.next_position();
        }

        position = next_position
    }
    viewed_positions
}

fn part1(path: &str) -> i32 {
    let map = parse_map(path);
    let viewed_positions = get_viewed_positions(&map);
    viewed_positions.len() as i32
}

fn is_looping(map: &[Vec<u8>], guard_position: &Position) -> bool {
    let width = map[0].len();
    let height = map.len();
    let mut position = guard_position.clone();
    let mut viewed_positions: Vec<Position> = Vec::new();

    while position.is_in_grid(width, height) {
        if viewed_positions.contains(&position) {
            return true;
        }

        viewed_positions.push(position.clone());

        let mut next_position = position.next_position();
        while next_position.is_in_grid(width, height) &&
            map[next_position.y as usize][next_position.x as usize] == b'#' {
            position.orientation.rotate();
            next_position = position.next_position();
        }

        position = next_position
    }
    false
}

fn part2(path: &str) -> i32 {
    let mut map = parse_map(path);
    let guard_position = get_guard_initial_position(&map).unwrap();
    let viewed_positions = get_viewed_positions(&map);

    let mut count = 0;
    for (x, y) in viewed_positions {

        if x == guard_position.x && y == guard_position.y {
            continue;
        }

        if map[y as usize][x as usize] == b'#' {
            continue;
        }

        map[y as usize][x as usize] = b'#';
        if is_looping(&map, &guard_position) {
            count += 1;
        }
        map[y as usize][x as usize] = b'.';
    }
    count
}

fn main() {
    println!("Part 1: {}", part1("assets/day6/input.txt"));
    println!("Part 2: {}", part2("assets/day6/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day6/example.txt"), 41);
        assert_eq!(part1("assets/day6/input.txt"), 4967);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day6/example.txt"), 6);
        assert_eq!(part2("assets/day6/input.txt"), 1789);
    }
}
