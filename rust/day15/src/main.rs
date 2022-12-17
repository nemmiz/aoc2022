use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    cbx: i32,
    cby: i32,
    range: i32,
}

impl Sensor {
    fn new(x: i32, y: i32, cbx: i32, cby: i32) -> Sensor {
        Sensor {
            x,
            y,
            cbx,
            cby,
            range: (x - cbx).abs() + (y - cby).abs(),
        }
    }

    fn in_range(&self, x: i32, y: i32) -> bool {
        ((x - self.x).abs() + (y - self.y).abs()) <= self.range
    }

    fn add_edge_points(&self, points: &mut HashSet<(i32, i32)>) {
        let mut x = self.x;
        let mut y = self.y - (self.range + 1);
        for _ in 0..=self.range + 1 {
            if x >= 0 && x <= 4000000 && y >= 0 && y <= 4000000 {
                points.insert((x, y));
            }
            x -= 1;
            y += 1;
        }
        for _ in 0..=self.range + 1 {
            if x >= 0 && x <= 4000000 && y >= 0 && y <= 4000000 {
                points.insert((x, y));
            }
            x += 1;
            y += 1;
        }
        for _ in 0..=self.range + 1 {
            if x >= 0 && x <= 4000000 && y >= 0 && y <= 4000000 {
                points.insert((x, y));
            }
            x += 1;
            y -= 1;
        }
        for _ in 0..=self.range + 1 {
            if x >= 0 && x <= 4000000 && y >= 0 && y <= 4000000 {
                points.insert((x, y));
            }
            x -= 1;
            y -= 1;
        }
    }
}

//fn add_edge()

fn main() {
    let input = include_str!("../../../input/15.txt");

    let sensors: Vec<Sensor> = input
        .chars()
        .filter(|&c| c.is_ascii_digit() || c == '-' || c == ' ')
        .collect::<String>()
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .chunks(4)
        .map(|chunk| Sensor::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect();

    let minx = sensors.iter().map(|s| s.x - s.range).min().unwrap();
    let maxx = sensors.iter().map(|s| s.x + s.range).max().unwrap();

    let mut part1 = 0;
    for x in minx..=maxx {
        if sensors.iter().any(|s| s.in_range(x, 2000000)) {
            if !sensors.iter().any(|s| s.cbx == x && s.cby == 2000000) {
                part1 += 1;
            }
        }
    }
    println!("{}", part1);

    let mut points: HashSet<(i32, i32)> = HashSet::new();
    for sensor in sensors.iter() {
        sensor.add_edge_points(&mut points);
    }

    let mut part2 = 0;
    for (x, y) in points.iter() {
        if !sensors.iter().any(|s| s.in_range(*x, *y)) {
            let xx: i64 = (*x).into();
            let yy: i64 = (*y).into();
            part2 = xx * 4000000 + yy;
            break;
        }
    }

    println!("{}", part2);
}
