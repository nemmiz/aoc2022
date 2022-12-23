use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

const NORTH_MASK: u8 = 0b10000011;
const EAST_MASK: u8 = 0b00001110;
const SOUTH_MASK: u8 = 0b00111000;
const WEST_MASK: u8 = 0b11100000;

fn adj_mask(positions: &HashSet<Pos>, pos: &Pos) -> u8 {
    let adj = [
        Pos::new(pos.x, pos.y - 1),
        Pos::new(pos.x + 1, pos.y - 1),
        Pos::new(pos.x + 1, pos.y),
        Pos::new(pos.x + 1, pos.y + 1),
        Pos::new(pos.x, pos.y + 1),
        Pos::new(pos.x - 1, pos.y + 1),
        Pos::new(pos.x - 1, pos.y),
        Pos::new(pos.x - 1, pos.y - 1),
    ];

    let mut mask: u8 = 0;
    for (i, apos) in adj.iter().enumerate() {
        if positions.contains(apos) {
            mask |= 1 << i;
        }
    }

    mask
}

fn propose_move(round: usize, elf: &Pos, positions: &HashSet<Pos>) -> Option<Pos> {
    let mask = adj_mask(&positions, &elf);
    if mask == 0 {
        return None;
    }
    if round % 4 == 0 {
        if mask & NORTH_MASK == 0 {
            return Some(Pos::new(elf.x, elf.y - 1));
        }
        if mask & SOUTH_MASK == 0 {
            return Some(Pos::new(elf.x, elf.y + 1));
        }
        if mask & WEST_MASK == 0 {
            return Some(Pos::new(elf.x - 1, elf.y));
        }
        if mask & EAST_MASK == 0 {
            return Some(Pos::new(elf.x + 1, elf.y));
        }
    } else if round % 4 == 1 {
        if mask & SOUTH_MASK == 0 {
            return Some(Pos::new(elf.x, elf.y + 1));
        }
        if mask & WEST_MASK == 0 {
            return Some(Pos::new(elf.x - 1, elf.y));
        }
        if mask & EAST_MASK == 0 {
            return Some(Pos::new(elf.x + 1, elf.y));
        }
        if mask & NORTH_MASK == 0 {
            return Some(Pos::new(elf.x, elf.y - 1));
        }
    } else if round % 4 == 2 {
        if mask & WEST_MASK == 0 {
            return Some(Pos::new(elf.x - 1, elf.y));
        }
        if mask & EAST_MASK == 0 {
            return Some(Pos::new(elf.x + 1, elf.y));
        }
        if mask & NORTH_MASK == 0 {
            return Some(Pos::new(elf.x, elf.y - 1));
        }
        if mask & SOUTH_MASK == 0 {
            return Some(Pos::new(elf.x, elf.y + 1));
        }
    } else if round % 4 == 3 {
        if mask & EAST_MASK == 0 {
            return Some(Pos::new(elf.x + 1, elf.y));
        }
        if mask & NORTH_MASK == 0 {
            return Some(Pos::new(elf.x, elf.y - 1));
        }
        if mask & SOUTH_MASK == 0 {
            return Some(Pos::new(elf.x, elf.y + 1));
        }
        if mask & WEST_MASK == 0 {
            return Some(Pos::new(elf.x - 1, elf.y));
        }
    }
    None
}

fn count_empty_tiles(elves: &Vec<Pos>) -> usize {
    let positions: HashSet<Pos> = elves.iter().map(|elf| *elf).collect();
    let minx = positions.iter().map(|p| p.x).min().unwrap();
    let miny = positions.iter().map(|p| p.y).min().unwrap();
    let maxx = positions.iter().map(|p| p.x).max().unwrap();
    let maxy = positions.iter().map(|p| p.y).max().unwrap();

    let mut n = 0;
    for y in miny..=maxy {
        for x in minx..=maxx {
            let pos = Pos { x, y };
            if !positions.contains(&pos) {
                n += 1;
            }
        }
    }
    n
}

fn main() {
    let input = include_str!("../../../input/23.txt");

    let mut elves = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Pos::new(x as i32, y as i32));
            }
        }
    }

    let mut proposed = Vec::with_capacity(elves.len());

    for round in 0.. {
        let positions: HashSet<Pos> = elves.iter().map(|elf| *elf).collect();

        proposed.clear();
        for elf in elves.iter() {
            proposed.push(propose_move(round, elf, &positions));
        }

        let mut counts = HashMap::new();
        for prop in proposed.iter() {
            if let Some(p) = prop {
                *counts.entry(p).or_insert(0) += 1;
            }
        }

        if counts.is_empty() {
            println!("{}", round + 1);
            break;
        }

        for (elf, prop) in elves.iter_mut().zip(proposed.iter()) {
            if let Some(p) = prop {
                if *counts.get(p).unwrap() == 1 {
                    elf.x = p.x;
                    elf.y = p.y;
                }
            }
        }

        if round == 9 {
            println!("{}", count_empty_tiles(&elves));
        }
    }
}
