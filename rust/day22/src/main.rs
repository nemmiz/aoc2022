use std::collections::HashMap;

#[derive(Debug)]
struct Board {
    lines: Vec<Vec<u8>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn rev(&self) -> Dir {
        match self {
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Up => Dir::Down,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Pos {
    fn rev(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y,
            dir: self.dir.rev(),
        }
    }

    fn score(&self) -> usize {
        let s = 1000 * (self.y + 1) + 4 * (self.x + 1);
        match self.dir {
            Dir::Right => return s,
            Dir::Down => return s + 1,
            Dir::Left => return s + 2,
            Dir::Up => return s + 3,
        };
    }

    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Left,
        }
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }
}

impl Board {
    fn new(lines: &[&[u8]]) -> Board {
        let mut board_lines = Vec::new();
        for line in lines {
            board_lines.push(line.to_vec());
        }
        Board { lines: board_lines }
    }

    fn find_start(&self) -> Pos {
        let x = self.lines[0].iter().position(|&x| x == b'.').unwrap();
        Pos {
            x,
            y: 0,
            dir: Dir::Right,
        }
    }

    fn move_wrapped(&self, pos: Pos, amount: usize) -> Pos {
        let mut x = pos.x;
        let mut y = pos.y;
        let mut last_x = x;
        let mut last_y = y;
        let mut amount = amount;

        let max = match pos.dir {
            Dir::Right | Dir::Left => self.lines[y].len() - 1,
            Dir::Down | Dir::Up => self.lines.len() - 1,
        };

        match pos.dir {
            Dir::Right => loop {
                x = match x {
                    xx if xx == max => 0,
                    _ => x + 1,
                };

                let tile = self.lines[y][x];
                if tile == b'.' {
                    amount -= 1;
                    last_x = x;
                    if amount == 0 {
                        break;
                    }
                } else if tile == b'#' {
                    break;
                }
            },
            Dir::Down => loop {
                y = match y {
                    yy if yy == max => 0,
                    _ => y + 1,
                };

                let tile = *self.lines[y].get(x).unwrap_or(&b' ');
                if tile == b'.' {
                    amount -= 1;
                    last_y = y;
                    if amount == 0 {
                        break;
                    }
                } else if tile == b'#' {
                    break;
                }
            },

            Dir::Left => loop {
                x = match x {
                    0 => max,
                    _ => x - 1,
                };

                let tile = self.lines[y][x];
                if tile == b'.' {
                    amount -= 1;
                    last_x = x;
                    if amount == 0 {
                        break;
                    }
                } else if tile == b'#' {
                    break;
                }
            },
            Dir::Up => loop {
                y = match y {
                    0 => max,
                    _ => y - 1,
                };

                let tile = *self.lines[y].get(x).unwrap_or(&b' ');
                if tile == b'.' {
                    amount -= 1;
                    last_y = y;
                    if amount == 0 {
                        break;
                    }
                } else if tile == b'#' {
                    break;
                }
            },
        }
        Pos {
            x: last_x,
            y: last_y,
            dir: pos.dir,
        }
    }

    fn try_cubic_move(&self, pos: &Pos, wraparounds: &HashMap<Pos, Pos>) -> Option<Pos> {
        let next = match wraparounds.get(&pos) {
            Some(p) => *p,
            None => match pos.dir {
                Dir::Right => Pos {
                    x: pos.x + 1,
                    y: pos.y,
                    dir: pos.dir,
                },
                Dir::Down => Pos {
                    x: pos.x,
                    y: pos.y + 1,
                    dir: pos.dir,
                },
                Dir::Left => Pos {
                    x: pos.x - 1,
                    y: pos.y,
                    dir: pos.dir,
                },
                Dir::Up => Pos {
                    x: pos.x,
                    y: pos.y - 1,
                    dir: pos.dir,
                },
            },
        };

        return match self.lines[next.y][next.x] {
            b'.' => Some(next),
            b'#' => None,
            _ => panic!(),
        };
    }
}

#[derive(Debug)]
enum Move {
    Go(usize),
    Left,
    Right,
}

fn parse_moves(move_line: &[u8]) -> Vec<Move> {
    let mut n: usize = 0;
    let mut moves = Vec::new();

    for byte in move_line.iter() {
        match byte {
            b'0'..=b'9' => n = n * 10 + (byte - b'0') as usize,
            b'L' => {
                if n != 0 {
                    moves.push(Move::Go(n));
                    n = 0;
                }
                moves.push(Move::Left);
            }
            b'R' => {
                if n != 0 {
                    moves.push(Move::Go(n));
                    n = 0;
                }
                moves.push(Move::Right);
            }
            _ => panic!(),
        }
    }

    if n != 0 {
        moves.push(Move::Go(n));
    }

    moves
}

fn part1(board: &Board, moves: &Vec<Move>) {
    let mut pos = board.find_start();
    for mov in moves {
        match mov {
            Move::Go(steps) => pos = board.move_wrapped(pos, *steps),
            Move::Left => pos.turn_left(),
            Move::Right => pos.turn_right(),
        }
    }
    println!("{}", pos.score());
}

fn hori_edge(from_x: usize, to_x: usize, y: usize, dir: Dir) -> Vec<Pos> {
    if from_x < to_x {
        (from_x..to_x).map(|x| Pos { x, y, dir }).collect()
    } else {
        (to_x..from_x).rev().map(|x| Pos { x, y, dir }).collect()
    }
}

fn vert_edge(x: usize, from_y: usize, to_y: usize, dir: Dir) -> Vec<Pos> {
    if from_y < to_y {
        (from_y..to_y).map(|y| Pos { x, y, dir }).collect()
    } else {
        (to_y..from_y).rev().map(|y| Pos { x, y, dir }).collect()
    }
}

fn add_cube_wrap(mapping: &mut HashMap<Pos, Pos>, a: &Vec<Pos>, b: &Vec<Pos>) {
    for (&aa, &bb) in a.iter().zip(b.iter()) {
        mapping.insert(aa, bb);
        mapping.insert(bb.rev(), aa.rev());
    }
}

fn part2(board: &Board, moves: &Vec<Move>) {
    let mut pos = board.find_start();
    let mut wraparounds = HashMap::new();

    // Hardcoded cubic wraps for my input
    add_cube_wrap(
        &mut wraparounds,
        &hori_edge(50, 100, 0, Dir::Up),
        &vert_edge(0, 150, 200, Dir::Right),
    );
    add_cube_wrap(
        &mut wraparounds,
        &hori_edge(100, 150, 0, Dir::Up),
        &hori_edge(0, 50, 199, Dir::Up),
    );
    add_cube_wrap(
        &mut wraparounds,
        &hori_edge(0, 50, 100, Dir::Up),
        &vert_edge(50, 50, 100, Dir::Right),
    );
    add_cube_wrap(
        &mut wraparounds,
        &hori_edge(100, 150, 49, Dir::Down),
        &vert_edge(99, 50, 100, Dir::Left),
    );
    add_cube_wrap(
        &mut wraparounds,
        &vert_edge(149, 0, 50, Dir::Right),
        &vert_edge(99, 150, 100, Dir::Left),
    );
    add_cube_wrap(
        &mut wraparounds,
        &vert_edge(49, 150, 200, Dir::Right),
        &hori_edge(50, 100, 149, Dir::Up),
    );
    add_cube_wrap(
        &mut wraparounds,
        &vert_edge(0, 100, 150, Dir::Left),
        &vert_edge(50, 50, 0, Dir::Right),
    );

    for mov in moves {
        match mov {
            Move::Go(steps) => {
                for _ in 0..*steps {
                    match board.try_cubic_move(&pos, &wraparounds) {
                        Some(p) => pos = p,
                        None => break,
                    }
                }
            }
            Move::Left => pos.turn_left(),
            Move::Right => pos.turn_right(),
        }
    }
    println!("{}", pos.score());
}

fn main() {
    let input = include_str!("../../../input/22.txt");
    let lines: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let (blines, plines) = lines.split_at(lines.len() - 2);
    let board = Board::new(blines);
    let moves = parse_moves(plines[1]);

    part1(&board, &moves);
    part2(&board, &moves);
}
