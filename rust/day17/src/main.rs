use std::collections::{hash_map::Entry, HashMap};

struct Rock {
    //y: usize,
    rows: [u8; 4],
}

struct Chamber {
    rows: Vec<u8>,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber { rows: Vec::new() }
    }

    fn get_rows(&self, y: usize) -> [u8; 4] {
        let mut result = [0; 4];
        for (i, yy) in (y..y + 4).rev().enumerate() {
            match self.rows.get(usize::try_from(yy).unwrap()) {
                Some(row) => result[i] = *row,
                None => (),
            }
        }
        result
    }

    fn rest(&mut self, rock: &Rock, y: usize) {
        let h = self.rows.len();
        for (i, y) in (y..y + 4).enumerate() {
            if y < h {
                self.rows[y] |= rock.rows[3 - i];
            } else {
                let row = rock.rows[3 - i];
                if row != 0 {
                    self.rows.push(rock.rows[3 - i]);
                }
            }
        }
    }

    fn drop_rock(&mut self, rock_type: usize, wind: &[u8], wind_index: usize) -> usize {
        let mut wind_index = wind_index;
        let mut rock = Rock::new(rock_type);
        let mut y = self.rows.len() + 3;
        loop {
            match wind[wind_index] {
                b'<' => rock.blow_left(&self, y),
                b'>' => rock.blow_right(&self, y),
                _ => panic!(),
            }

            wind_index += 1;
            if wind_index >= wind.len() {
                wind_index = 0;
            }

            if rock.can_move_down(&self, y) {
                y -= 1;
            } else {
                self.rest(&rock, y);
                break;
            }
        }
        wind_index
    }
}

impl Rock {
    fn new(rock_type: usize) -> Rock {
        match rock_type {
            0 => Rock {
                rows: [0, 0, 0, 0b0011110],
            },
            1 => Rock {
                rows: [0, 0b0001000, 0b0011100, 0b0001000],
            },
            2 => Rock {
                rows: [0, 0b0000100, 0b0000100, 0b0011100],
            },
            3 => Rock {
                rows: [0b0010000, 0b0010000, 0b0010000, 0b0010000],
            },
            4 => Rock {
                rows: [0, 0, 0b0011000, 0b0011000],
            },
            _ => panic!(),
        }
    }

    fn blow_left(&mut self, chamber: &Chamber, y: usize) {
        let chamber_rows = chamber.get_rows(y);
        let bit: u8 = 1 << 6;
        for (i, row) in self.rows.iter().enumerate() {
            if row & bit != 0 {
                return;
            }
            if chamber_rows[i] & (row << 1) != 0 {
                return;
            }
        }
        self.rows[0] <<= 1;
        self.rows[1] <<= 1;
        self.rows[2] <<= 1;
        self.rows[3] <<= 1;
    }

    fn blow_right(&mut self, chamber: &Chamber, y: usize) {
        let chamber_rows = chamber.get_rows(y);
        let bit: u8 = 1;
        for (i, row) in self.rows.iter().enumerate() {
            if row & bit != 0 {
                return;
            }
            if chamber_rows[i] & (row >> 1) != 0 {
                return;
            }
        }
        self.rows[0] >>= 1;
        self.rows[1] >>= 1;
        self.rows[2] >>= 1;
        self.rows[3] >>= 1;
    }

    fn can_move_down(&mut self, chamber: &Chamber, y: usize) -> bool {
        if y == 0 {
            return false;
        }
        let chamber_rows = chamber.get_rows(y - 1);
        for (i, row) in self.rows.iter().enumerate() {
            if (chamber_rows[i] & row) != 0 {
                return false;
            }
        }
        return true;
    }
}

fn part1(wind: &[u8]) {
    let mut wind_index = 0;
    let mut chamber = Chamber::new();

    for i in 0..2022 {
        let rock_type = i % 5;
        wind_index = chamber.drop_rock(rock_type, wind, wind_index);
    }

    println!("{}", chamber.rows.len());
}

fn part2(wind: &[u8]) {
    let num_rocks = 1000000000000;
    let mut wind_index = 0;
    let mut states = HashMap::new();

    let mut chamber = Chamber::new();
    let mut cycle_rows = 0;
    let mut i = 0;

    while i < num_rocks {
        let rock_type = i % 5;

        wind_index = chamber.drop_rock(rock_type, wind, wind_index);

        i += 1;

        if chamber.rows.len() >= 20 {
            let last_rows: [u8; 20] = chamber.rows[chamber.rows.len() - 20..].try_into().unwrap();
            let state = (last_rows, rock_type, wind_index);

            match states.entry(state) {
                Entry::Vacant(entry) => {
                    entry.insert((i, chamber.rows.len()));
                }
                Entry::Occupied(entry) => {
                    let (prev_i, prev_rows) = entry.get();
                    let rocks_in_cycle = i - prev_i;
                    let num_cycles = (num_rocks - i) / rocks_in_cycle;
                    i += rocks_in_cycle * num_cycles;
                    cycle_rows += num_cycles * (chamber.rows.len() - prev_rows);
                    states.clear();
                }
            }
        }
    }
    println!("{}", chamber.rows.len() + cycle_rows);
}

fn main() {
    let input = include_bytes!("../../../input/17.txt");

    part1(input);
    part2(input);
}
