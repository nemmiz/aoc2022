use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
    id: u8,
    flow_rate: i32,
    neighbors: Vec<u8>,
}

impl Valve {
    fn new(line: &str, names: &HashMap<String, u8>) -> Valve {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let id = *names.get(parts[1]).unwrap();
        let flow_rate = parts[4]
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let neighbors: Vec<u8> = parts[9..]
            .iter()
            .map(|&part| *names.get(part.trim_end_matches(',')).unwrap())
            .collect();
        Valve {
            id,
            flow_rate,
            neighbors,
        }
    }
}

fn floyd_warshall(valves: &HashMap<u8, Valve>) -> Vec<Vec<u8>> {
    let mut dist = Vec::new();
    for _ in 0..valves.len() {
        dist.push(vec![100; valves.len()]);
    }

    for i in 0..valves.len() {
        dist[i][i] = 0;
    }

    for valve in valves.values() {
        for n in valve.neighbors.iter() {
            dist[usize::from(valve.id)][usize::from(*n)] = 1;
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    dist
}

fn find_path(
    targets: &Vec<u8>,
    valves: &HashMap<u8, Valve>,
    dists: &Vec<Vec<u8>>,
    current: u8,
    minutes: i32,
) -> i32 {
    let targets: Vec<u8> = targets
        .iter()
        .filter(|&t| *t != current)
        .map(|t| *t)
        .collect();

    let mut best_flow = 0;

    for target in targets.iter() {
        let minutes_left =
            minutes - 1 - i32::from(dists[usize::from(current)][usize::from(*target)]);

        if minutes_left > 0 {
            let mut flow = valves.get(target).unwrap().flow_rate * minutes_left;
            let new_flow = find_path(&targets, valves, dists, *target, minutes_left);
            flow += new_flow;

            if flow > best_flow {
                best_flow = flow;
            }
        }
    }
    best_flow
}

fn part1(valves: &HashMap<u8, Valve>, dists: &Vec<Vec<u8>>, start: u8) {
    let targets: Vec<u8> = valves
        .values()
        .filter(|value| value.flow_rate > 0)
        .map(|valve| valve.id)
        .collect();

    let best = find_path(&targets, valves, dists, start, 30);

    println!("{}", best);
}

fn part2(valves: &HashMap<u8, Valve>, dists: &Vec<Vec<u8>>, start: u8) {
    let orig: Vec<u8> = valves
        .values()
        .filter(|value| value.flow_rate > 0)
        .map(|valve| valve.id)
        .collect();

    let len = orig.len();

    let mut incl = Vec::new();
    let mut excl = Vec::new();

    let mut best = 0;

    incl.reserve(len);
    excl.reserve(len);

    for i in 0..(1 << len) {
        for j in 0..len {
            if (i & (1 << j)) != 0 {
                incl.push(orig[j]);
            } else {
                excl.push(orig[j]);
            }
        }

        let best1 = find_path(&incl, valves, dists, start, 26);
        let best2 = find_path(&excl, valves, dists, start, 26);
        let tmp = best1 + best2;
        if tmp > best {
            best = tmp;
        }

        incl.clear();
        excl.clear();
    }

    println!("{}", best);
}

fn main() {
    let input = include_str!("../../../input/16.txt");

    let mut names: HashMap<String, u8> = HashMap::new();
    for line in input.lines() {
        let name: String = line.chars().skip(6).take(2).collect();
        names.insert(name, u8::try_from(names.len()).unwrap());
    }

    let mut valves = HashMap::new();
    for line in input.lines() {
        let valve = Valve::new(line, &names);
        valves.insert(valve.id, valve);
    }

    let dists = floyd_warshall(&valves);
    let start = *names.get("AA").unwrap();

    part1(&valves, &dists, start);
    part2(&valves, &dists, start);
}
