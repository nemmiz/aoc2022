use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    width: usize,
    height: usize,
    start: (usize, usize),
    goal: (usize, usize),
    data: Vec<Vec<u8>>,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut start: Option<(usize, usize)> = None;
        let mut goal: Option<(usize, usize)> = None;
        let mut data: Vec<Vec<u8>> = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let mut tmp = Vec::new();
            for (x, &byte) in line.as_bytes().iter().enumerate() {
                if byte == b'S' {
                    start = Some((x, y));
                    tmp.push(b'a');
                } else if byte == b'E' {
                    goal = Some((x, y));
                    tmp.push(b'z');
                } else {
                    tmp.push(byte);
                }
            }
            data.push(tmp);
        }

        Grid {
            width: data[0].len(),
            height: data.len(),
            start: start.unwrap(),
            goal: goal.unwrap(),
            data,
        }
    }

    fn elevation(&self, position: &(usize, usize)) -> u8 {
        self.data[position.1][position.0]
    }

    fn find_all(&self, byte: u8) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (y, row) in self.data.iter().enumerate() {
            for (x, &b) in row.iter().enumerate() {
                if b == byte {
                    result.push((x, y));
                }
            }
        }
        result
    }

    fn neighbors(&self, position: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let height = self.elevation(position);

        if position.0 > 0 {
            let pos = (position.0 - 1, position.1);
            if self.elevation(&pos) <= height + 1 {
                result.push(pos);
            }
        }

        if position.1 > 0 {
            let pos = (position.0, position.1 - 1);
            if self.elevation(&pos) <= height + 1 {
                result.push(pos);
            }
        }

        if position.0 < self.width - 1 {
            let pos = (position.0 + 1, position.1);
            if self.elevation(&pos) <= height + 1 {
                result.push(pos);
            }
        }

        if position.1 < self.height - 1 {
            let pos = (position.0, position.1 + 1);
            if self.elevation(&pos) <= height + 1 {
                result.push(pos);
            }
        }

        result
    }

    fn shortest_path(&self, start: (usize, usize), goal: (usize, usize)) -> Option<i32> {
        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                return Some(cost);
            }

            if let Some(&best) = dist.get(&position) {
                if cost > best {
                    continue;
                }
            }

            for neighbor in self.neighbors(&position) {
                let next_cost = cost + 1;
                let &prev_cost = dist.get(&neighbor).unwrap_or(&i32::MAX);

                if next_cost < prev_cost {
                    heap.push(State {
                        cost: next_cost,
                        position: neighbor,
                    });
                    dist.insert(neighbor, next_cost);
                }
            }
        }

        None
    }
}

fn main() {
    let input = include_str!("../../../input/12.txt");
    let grid = Grid::new(input);

    let part1 = grid.shortest_path(grid.start, grid.goal).unwrap();

    let part2 = grid
        .find_all(b'a')
        .iter()
        .map(|&start| grid.shortest_path(start, grid.goal))
        .filter(|cost| cost.is_some())
        .map(|cost| cost.unwrap())
        .min()
        .unwrap();

    println!("{} {}", part1, part2);
}
