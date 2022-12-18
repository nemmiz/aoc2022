use std::collections::{HashSet, VecDeque};

fn part1(cubes: &HashSet<(i32, i32, i32)>) {
    let mut sides = 0;

    for (x, y, z) in cubes.iter() {
        if !cubes.contains(&(*x - 1, *y, *z)) {
            sides += 1;
        }
        if !cubes.contains(&(*x + 1, *y, *z)) {
            sides += 1;
        }
        if !cubes.contains(&(*x, *y - 1, *z)) {
            sides += 1;
        }
        if !cubes.contains(&(*x, *y + 1, *z)) {
            sides += 1;
        }
        if !cubes.contains(&(*x, *y, *z - 1)) {
            sides += 1;
        }
        if !cubes.contains(&(*x, *y, *z + 1)) {
            sides += 1;
        }
    }

    println!("{:?}", sides);
}

fn part2(cubes: &HashSet<(i32, i32, i32)>) {
    let minx = cubes.iter().map(|cube| cube.0).min().unwrap() - 1;
    let maxx = cubes.iter().map(|cube| cube.0).max().unwrap() + 1;
    let miny = cubes.iter().map(|cube| cube.1).min().unwrap() - 1;
    let maxy = cubes.iter().map(|cube| cube.1).max().unwrap() + 1;
    let minz = cubes.iter().map(|cube| cube.2).min().unwrap() - 1;
    let maxz = cubes.iter().map(|cube| cube.2).max().unwrap() + 1;

    let mut sides = 0;
    let mut checked = HashSet::new();
    let mut queue = VecDeque::from([(minx, miny, minz)]);

    while !queue.is_empty() {
        let cube = queue.pop_front().unwrap();

        if checked.contains(&cube) {
            continue;
        }

        checked.insert(cube);

        if cube.0 > minx {
            let next = (cube.0 - 1, cube.1, cube.2);
            if !checked.contains(&next) {
                if cubes.contains(&next) {
                    sides += 1;
                } else {
                    queue.push_back(next);
                }
            }
        }

        if cube.0 < maxx {
            let next = (cube.0 + 1, cube.1, cube.2);
            if !checked.contains(&next) {
                if cubes.contains(&next) {
                    sides += 1;
                } else {
                    queue.push_back(next);
                }
            }
        }

        if cube.1 > miny {
            let next = (cube.0, cube.1 - 1, cube.2);
            if !checked.contains(&next) {
                if cubes.contains(&next) {
                    sides += 1;
                } else {
                    queue.push_back(next);
                }
            }
        }

        if cube.1 < maxy {
            let next = (cube.0, cube.1 + 1, cube.2);
            if !checked.contains(&next) {
                if cubes.contains(&next) {
                    sides += 1;
                } else {
                    queue.push_back(next);
                }
            }
        }

        if cube.2 > minz {
            let next = (cube.0, cube.1, cube.2 - 1);
            if !checked.contains(&next) {
                if cubes.contains(&next) {
                    sides += 1;
                } else {
                    queue.push_back(next);
                }
            }
        }

        if cube.2 < maxz {
            let next = (cube.0, cube.1, cube.2 + 1);
            if !checked.contains(&next) {
                if cubes.contains(&next) {
                    sides += 1;
                } else {
                    queue.push_back(next);
                }
            }
        }
    }

    println!("{}", sides);
}

fn main() {
    let input = include_str!("../../../input/18.txt");

    let mut cubes = HashSet::new();
    for line in input.lines() {
        let tmp: Vec<_> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        cubes.insert((tmp[0], tmp[1], tmp[2]));
    }

    part1(&cubes);
    part2(&cubes);
}
