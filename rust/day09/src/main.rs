use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope<const LEN: usize> {
    points: [Point; LEN],
}

impl<const LEN: usize> Rope<LEN> {
    pub fn new() -> Self {
        Rope {
            points: [Point { x: 0, y: 0 }; LEN],
        }
    }

    fn move_head(&mut self, dir: char) {
        match dir {
            'U' => self.points[0].y -= 1,
            'D' => self.points[0].y += 1,
            'L' => self.points[0].x -= 1,
            'R' => self.points[0].x += 1,
            _ => panic!(),
        }
        for i in 1..LEN {
            let dx = self.points[i].x - self.points[i - 1].x;
            let dy = self.points[i].y - self.points[i - 1].y;

            if dx < -1 || dx > 1 || dy < -1 || dy > 1 {
                if dx < 0 {
                    self.points[i].x += 1;
                } else if dx > 0 {
                    self.points[i].x -= 1;
                }
                if dy < 0 {
                    self.points[i].y += 1;
                } else if dy > 0 {
                    self.points[i].y -= 1;
                }
            }
        }
    }

    fn get(&self, i: usize) -> Point {
        self.points[i].clone()
    }
}

fn main() {
    let input = include_str!("../../../input/09.txt");

    let mut rope: Rope<10> = Rope::new();
    let mut positions1: HashSet<Point> = HashSet::new();
    let mut positions2: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();
        let dir = a.chars().next().unwrap();
        let amt = b.parse::<i32>().unwrap();

        for _ in 0..amt {
            rope.move_head(dir);
            positions1.insert(rope.get(1));
            positions2.insert(rope.get(9));
        }
    }

    println!("{} {}", positions1.len(), positions2.len());
}
