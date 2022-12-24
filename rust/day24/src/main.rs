use std::collections::HashSet;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Valley {
    maxx: i32,
    maxy: i32,
    walls: HashSet<(i32, i32)>,
    bpos: Vec<(i32, i32)>,
    bdir: Vec<Dir>,
}

impl Valley {
    fn new(input: &str) -> Valley {
        let mut maxx: i32 = 0;
        let mut maxy: i32 = 0;
        let mut bpos = Vec::new();
        let mut bdir = Vec::new();
        let mut walls = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            maxy = std::cmp::max(maxy, y as i32);
            for (x, c) in line.chars().enumerate() {
                maxx = std::cmp::max(maxx, x as i32);
                match c {
                    '#' => {
                        walls.insert((x as i32, y as i32));
                    }
                    '^' => {
                        bpos.push((x as i32, y as i32));
                        bdir.push(Dir::Up);
                    }
                    'v' => {
                        bpos.push((x as i32, y as i32));
                        bdir.push(Dir::Down);
                    }
                    '<' => {
                        bpos.push((x as i32, y as i32));
                        bdir.push(Dir::Left);
                    }
                    '>' => {
                        bpos.push((x as i32, y as i32));
                        bdir.push(Dir::Right);
                    }
                    _ => (),
                }
            }
        }
        walls.insert((1, -1));
        walls.insert((maxx - 1, maxy + 1));
        Valley {
            maxx: maxx - 1,
            maxy: maxy - 1,
            walls,
            bpos,
            bdir,
        }
    }

    fn tick(&mut self) {
        for (pos, dir) in self.bpos.iter_mut().zip(self.bdir.iter()) {
            match *dir {
                Dir::Up => {
                    pos.1 -= 1;
                    if pos.1 <= 0 {
                        pos.1 = self.maxy;
                    }
                }
                Dir::Down => {
                    pos.1 += 1;
                    if pos.1 > self.maxy {
                        pos.1 = 1;
                    }
                }
                Dir::Left => {
                    pos.0 -= 1;
                    if pos.0 <= 0 {
                        pos.0 = self.maxx;
                    }
                }
                Dir::Right => {
                    pos.0 += 1;
                    if pos.0 > self.maxx {
                        pos.0 = 1;
                    }
                }
            }
        }
    }

    fn occupied_positions(&self) -> HashSet<(i32, i32)> {
        let mut ret = self.walls.clone();
        for pos in self.bpos.iter() {
            ret.insert(*pos);
        }
        ret
    }
}

fn walk(valley: &mut Valley, start: (i32, i32), goal: (i32, i32)) -> i32 {
    let mut visited = HashSet::new();
    visited.insert(start);

    for step in 0.. {
        if visited.contains(&goal) {
            return step;
        }

        valley.tick();
        let occupied = valley.occupied_positions();

        let mut next_visited = HashSet::new();
        for pos in visited.iter() {
            let up = (pos.0, pos.1 - 1);
            if !occupied.contains(&up) {
                next_visited.insert(up);
            }
            let down = (pos.0, pos.1 + 1);
            if !occupied.contains(&down) {
                next_visited.insert(down);
            }
            let left = (pos.0 - 1, pos.1);
            if !occupied.contains(&left) {
                next_visited.insert(left);
            }
            let right = (pos.0 + 1, pos.1);
            if !occupied.contains(&right) {
                next_visited.insert(right);
            }
            if !occupied.contains(&pos) {
                next_visited.insert(*pos);
            }
        }
        visited = next_visited;
    }

    panic!();
}

fn main() {
    let input = include_str!("../../../input/24.txt");
    let mut valley = Valley::new(input);

    let start = (1, 0);
    let goal = (valley.maxx, valley.maxy + 1);

    let a = walk(&mut valley, start, goal);
    let b = walk(&mut valley, goal, start);
    let c = walk(&mut valley, start, goal);

    println!("{} {}", a, a + b + c);
}
