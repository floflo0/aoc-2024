use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Blocks {
    Files { id: i64, size: i64 },
    Free(i64),
}

fn part1(path: &str) -> i64 {
    let mut file = File::open(path).unwrap();
    let mut disk_str = String::new();
    file.read_to_string(&mut disk_str).unwrap();
    let mut disk: Vec<Blocks> = Vec::new();
    for (i, size) in disk_str.trim_end().chars().enumerate() {
        let size = size.to_string().parse().unwrap();
        if size == 0 {
            continue;
        }
        if (i % 2) == 0 {
            disk.push(Blocks::Files {
                id: i as i64 / 2,
                size,
            });
            continue;
        }

        disk.push(Blocks::Free(size));
    }
    if let Blocks::Files { .. } = disk.last().unwrap() {
        disk.push(Blocks::Free(0));
    }
    let mut i = 0;
    while i < disk.len() {
        match disk[i] {
            Blocks::Files { .. } => {}
            Blocks::Free(size) => {
                let j = disk.len() - 2;
                match disk[j] {
                    Blocks::Files { id, size: size2 } => {
                        if size2 > size {
                            disk[i] = Blocks::Files { id, size };
                            disk[j] = Blocks::Files {
                                id,
                                size: size2 - size,
                            };
                            if let Blocks::Free(end_size) = disk.last().unwrap() {
                                let k = disk.len() - 1;
                                disk[k] = Blocks::Free(end_size + size);
                            }
                        } else {
                            disk[i] = Blocks::Files { id, size: size2 };
                            if size != size2 {
                                disk.insert(i + 1, Blocks::Free(size - size2));
                            }
                            let k = disk.len() - 3;
                            let mut merge_size = 0;
                            if let Blocks::Free(s) = disk[k] {
                                merge_size = s;
                                disk.remove(k);
                            }
                            let j = disk.len() - 2;
                            if let Blocks::Free(end_size) = disk.last().unwrap() {
                                disk[j] = Blocks::Free(end_size + size2 + merge_size);
                            }
                            disk.pop();
                        }
                    }
                    Blocks::Free(size2) => {
                        disk[j] = Blocks::Free(size + size2);
                        disk.pop();
                    }
                }
            }
        }
        i += 1;
    }
    let mut checksum = 0;
    let mut i = 0;
    for block in disk {
        if let Blocks::Files { id, size } = block {
            for _ in 0..size {
                checksum += id * i;
                i += 1;
            }
        }
    }
    checksum
}

fn part2(path: &str) -> i64 {
    let mut file = File::open(path).unwrap();
    let mut disk_str = String::new();
    file.read_to_string(&mut disk_str).unwrap();
    let mut disk: Vec<Blocks> = Vec::new();
    for (i, size) in disk_str.trim_end().chars().enumerate() {
        let size = size.to_string().parse().unwrap();
        if size == 0 {
            continue;
        }
        if (i % 2) == 0 {
            disk.push(Blocks::Files {
                id: i as i64 / 2,
                size,
            });
            continue;
        }

        disk.push(Blocks::Free(size));
    }
    let mut j = disk.len() - 1;
    while 1 < j {
        if let Blocks::Files { id, size } = disk[j] {
            let mut move_file = false;
            for i in 0..j {
                if let Blocks::Free(free_size) = disk[i] {
                    if free_size == size {
                        disk[i] = Blocks::Files { id, size };
                        move_file = true;
                        disk[j] = Blocks::Free(size);
                        break;
                    }
                    if free_size > size {
                        disk[i] = Blocks::Files { id, size };
                        move_file = true;
                        disk[j] = Blocks::Free(size);
                        disk.insert(i + 1, Blocks::Free(free_size - size));
                        break;
                    }
                }
            }
            if !move_file {
                j -= 1;
            }
        } else {
            j -= 1;
        }
    }
    let mut checksum = 0;
    let mut i = 0;
    for block in disk {
        match block {
            Blocks::Files { id, size } => {
                for _ in 0..size {
                    checksum += id * i;
                    i += 1;
                }
            }
            Blocks::Free(size) => i += size,
        }
    }
    checksum
}

fn main() {
    println!("Part 1: {}", part1("assets/day09/input.txt"));
    println!("Part 2: {}", part2("assets/day09/input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("assets/day09/example.txt"), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("assets/day09/example.txt"), 2858);
    }
}
