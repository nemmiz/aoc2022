use std::cmp;
use std::str;

const SPAWN_X: usize = 500;
const SPAWN_Y: usize = 0;

struct Grid {
    w: usize,
    h: usize,
    x_offset: usize,
    y_offset: usize,
    data: Vec<Vec<u8>>,
}

enum State {
    ExitDown,
    ExitLeft,
    ExitRight,
    RestAt { x: usize, y: usize },
}

impl Grid {
    fn new(input: &'static str) -> Grid {
        let mut minx = SPAWN_X;
        let mut maxx = SPAWN_X;
        let mut miny = SPAWN_Y;
        let mut maxy = SPAWN_Y;

        let mut lines: Vec<Vec<(usize, usize)>> = Vec::new();

        for line in input.lines() {
            let mut pts = Vec::new();
            for pair in line.split(" -> ") {
                let (x, y) = pair.split_once(',').unwrap();
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                pts.push((x, y));
                //println!("{} {}", x, y);
                minx = cmp::min(minx, x);
                maxx = cmp::max(maxx, x);
                miny = cmp::min(miny, y);
                maxy = cmp::max(maxy, y);
            }
            lines.push(pts);
        }

        let w = maxx - minx + 1;
        let h = maxy - miny + 1;
        let x_offset = minx;
        let y_offset = miny;

        let mut grid = Vec::new();
        for _ in 0..h {
            grid.push(vec![b'.'; w.try_into().unwrap()]);
        }
        grid[SPAWN_Y - y_offset][SPAWN_X - x_offset] = b'+';

        for line in lines.iter() {
            for window in line.iter().as_slice().windows(2) {
                let from = window[0];
                let to = window[1];
                if from.0 != to.0 {
                    for x in cmp::min(from.0, to.0)..=cmp::max(from.0, to.0) {
                        grid[from.1 - y_offset][x - x_offset] = b'#';
                    }
                } else {
                    for y in cmp::min(from.1, to.1)..=cmp::max(from.1, to.1) {
                        grid[y - y_offset][from.0 - x_offset] = b'#';
                    }
                }
            }
        }

        Grid {
            w,
            h,
            x_offset,
            y_offset,
            data: grid,
        }
    }

    fn expand(&mut self) {
        let new_h = self.h + 2;
        let new_w = new_h + self.w + new_h;

        let mut new_data = Vec::new();
        new_data.reserve(new_h);

        for row in self.data.iter_mut() {
            let mut new_row = Vec::new();
            new_row.reserve(new_w);
            new_row.resize(new_h, b'.');
            new_row.append(row);
            new_row.resize(new_w, b'.');
            new_data.push(new_row);
        }

        new_data.push(vec![b'.'; new_w]);
        new_data.push(vec![b'#'; new_w]);

        self.w = new_w;
        self.h = new_h;
        self.x_offset -= new_h;
        self.data = new_data;
    }

    fn add_sand(&mut self) -> State {
        let mut x = SPAWN_X - self.x_offset;
        let mut y = SPAWN_Y - self.y_offset;

        loop {
            y += 1;
            if y >= self.h {
                return State::ExitDown;
            }

            if self.data[y][x] == b'.' {
                continue;
            }

            if x == 0 {
                return State::ExitLeft;
            } else if self.data[y][x - 1] == b'.' {
                x -= 1;
                continue;
            }

            if x == self.w - 1 {
                return State::ExitRight;
            } else if self.data[y][x + 1] == b'.' {
                x += 1;
                continue;
            }

            y -= 1;
            self.data[y][x] = b'o';
            return State::RestAt {
                x: x + self.x_offset,
                y: y + self.y_offset,
            };
        }
    }
}

fn main() {
    let input = include_str!("../../../input/14.txt");
    let mut grid = Grid::new(input);

    let mut part1 = 0;
    loop {
        match grid.add_sand() {
            State::RestAt { x: _, y: _ } => (),
            _ => break,
        }
        part1 += 1;
    }

    grid.expand();

    let mut part2 = part1;
    loop {
        part2 += 1;
        match grid.add_sand() {
            State::RestAt { x, y } if x != SPAWN_X || y != SPAWN_Y => (),
            _ => break,
        }
    }

    println!("{} {}", part1, part2);
}
