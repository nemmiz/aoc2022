use core::str::Lines;
use std::collections::HashMap;

fn check(lines: &mut Lines, cwd: &str, sizes: &mut HashMap<String, i32>) -> i32 {
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd ..") {
            break;
        } else if line.starts_with("$ cd ") {
            let dir: String = line.chars().skip(5).collect();
            let mut next_dir = String::new();

            if dir == "/" {
                next_dir = dir.clone();
            } else if cwd == "/" {
                next_dir.push_str(cwd);
                next_dir.push_str(&dir);
            } else {
                next_dir.push_str(cwd);
                next_dir.push('/');
                next_dir.push_str(&dir);
            }

            let subsize = check(lines, &next_dir, sizes);
            if !cwd.is_empty() {
                *sizes.entry(cwd.to_string()).or_insert(0) += subsize;
            }
        } else {
            let (tmp, _) = line.split_once(' ').unwrap();
            if let Ok(num) = tmp.parse::<i32>() {
                *sizes.entry(cwd.to_string()).or_insert(0) += num;
            }
        }
    }
    match sizes.get(cwd) {
        Some(x) => *x,
        None => 0,
    }
}

fn main() {
    let input = include_str!("../../../input/07.txt");
    let mut sizes = HashMap::new();
    let mut lines = input.lines();

    check(&mut lines, "", &mut sizes);

    let part1: i32 = sizes.values().filter(|&x| *x <= 100_000).sum();
    let required = *sizes.get("/").unwrap() - 40_000_000;
    let part2 = sizes.values().filter(|&x| *x >= required).min().unwrap();

    println!("{} {}", part1, part2);
}
