use std::cmp::Ordering;

fn visible(rows: &Vec<&[u8]>, x: usize, y: usize) -> bool {
    let height = rows[y][x];
    let nrows = rows.len();
    let ncols = rows[0].len();

    if (0..x).all(|xx| rows[y][xx] < height) {
        return true;
    }
    if (x + 1..ncols).all(|xx| rows[y][xx] < height) {
        return true;
    }
    if (0..y).all(|yy| rows[yy][x] < height) {
        return true;
    }
    if (y + 1..nrows).all(|yy| rows[yy][x] < height) {
        return true;
    }

    false
}

fn scenic_score(rows: &Vec<&[u8]>, x: usize, y: usize) -> i32 {
    let height = rows[y][x];
    let last_row = rows.len() - 1;
    let last_col = rows[0].len() - 1;

    let mut left = 0;
    for xx in (0..x).rev() {
        match rows[y][xx].cmp(&height) {
            Ordering::Less => left += 1,
            _ => {
                left += 1;
                break;
            }
        }
    }

    let mut right = 0;
    for xx in (x + 1)..=last_col {
        match rows[y][xx].cmp(&height) {
            Ordering::Less => right += 1,
            _ => {
                right += 1;
                break;
            }
        }
    }

    let mut up = 0;
    for yy in (0..y).rev() {
        match rows[yy][x].cmp(&height) {
            Ordering::Less => up += 1,
            _ => {
                up += 1;
                break;
            }
        }
    }

    let mut down = 0;
    for yy in (y + 1)..=last_row {
        match rows[yy][x].cmp(&height) {
            Ordering::Less => down += 1,
            _ => {
                down += 1;
                break;
            }
        }
    }

    left * right * up * down
}

fn main() {
    let input = include_str!("../../../input/08.txt");
    let rows: Vec<&[u8]> = input.lines().map(|line| line.trim().as_bytes()).collect();

    let nrows = rows.len();
    let ncols = rows[0].len();

    let mut part1 = nrows * 2 + (ncols - 2) * 2;
    let mut part2 = 0;

    for y in 1..(nrows - 1) {
        for x in 1..(ncols - 1) {
            if visible(&rows, x, y) {
                part1 += 1;
            }
            let scenic = scenic_score(&rows, x, y);
            if scenic > part2 {
                part2 = scenic;
            }
        }
    }

    println!("{} {}", part1, part2);
}
