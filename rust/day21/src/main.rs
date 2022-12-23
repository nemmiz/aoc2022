use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Set(u32, i64),
    Add(u32, u32, u32),
    Sub(u32, u32, u32),
    Mul(u32, u32, u32),
    Div(u32, u32, u32),
    RootEq(u32, u32),
}

fn parse(line: &str) -> Operation {
    let bytes = line.as_bytes();
    if bytes.len() > 12 {
        let a = u32::from_ne_bytes(bytes[..4].try_into().unwrap());
        let b = u32::from_ne_bytes(bytes[6..10].try_into().unwrap());
        let c = u32::from_ne_bytes(bytes[13..17].try_into().unwrap());
        match bytes[11] {
            b'+' => return Operation::Add(a, b, c),
            b'-' => return Operation::Sub(a, b, c),
            b'*' => return Operation::Mul(a, b, c),
            b'/' => return Operation::Div(a, b, c),
            _ => panic!(),
        }
    } else {
        let a = u32::from_ne_bytes(bytes[..4].try_into().unwrap());
        let s: String = line.chars().skip(6).collect();
        Operation::Set(a, s.parse::<i64>().unwrap())
    }
}

fn part1(input: &str) {
    let mut ops: VecDeque<Operation> = input.lines().map(|line| parse(line)).collect();
    let mut vars = HashMap::new();
    let root = u32::from_ne_bytes(*b"root");

    while !ops.is_empty() {
        let op = ops.pop_front().unwrap();

        match op {
            Operation::Set(a, v) => {
                vars.insert(a, v);
            }
            Operation::Add(a, b, c) => {
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                if bv.is_some() && cv.is_some() {
                    vars.insert(a, bv.unwrap() + cv.unwrap());
                } else {
                    ops.push_back(op);
                }
            }
            Operation::Sub(a, b, c) => {
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                if bv.is_some() && cv.is_some() {
                    vars.insert(a, bv.unwrap() - cv.unwrap());
                } else {
                    ops.push_back(op);
                }
            }
            Operation::Mul(a, b, c) => {
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                if bv.is_some() && cv.is_some() {
                    vars.insert(a, bv.unwrap() * cv.unwrap());
                } else {
                    ops.push_back(op);
                }
            }
            Operation::Div(a, b, c) => {
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                if bv.is_some() && cv.is_some() {
                    vars.insert(a, bv.unwrap() / cv.unwrap());
                } else {
                    ops.push_back(op);
                }
            }
            _ => panic!(),
        }
    }

    println!("{:?}", vars.get(&root).unwrap());
}

fn part2(input: &str) {
    let root = u32::from_ne_bytes(*b"root");
    let humn = u32::from_ne_bytes(*b"humn");

    let mut ops: VecDeque<Operation> = input
        .lines()
        .map(|line| parse(line))
        .filter(|op| match op {
            Operation::Set(a, _) if *a == humn => false,
            _ => true,
        })
        .map(|op| match op {
            Operation::Add(a, b, c) if a == root => Operation::RootEq(b, c),
            op => op,
        })
        .collect();

    let mut vars = HashMap::new();

    while !ops.is_empty() {
        let op = ops.pop_front().unwrap();

        match op {
            Operation::Set(a, v) => {
                vars.insert(a, v);
            }
            Operation::Add(a, b, c) => {
                let av = vars.get(&a);
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                if bv.is_some() && cv.is_some() {
                    vars.insert(a, bv.unwrap() + cv.unwrap());
                } else {
                    if av.is_some() && bv.is_some() {
                        vars.insert(c, av.unwrap() - bv.unwrap());
                    } else if av.is_some() && cv.is_some() {
                        vars.insert(b, av.unwrap() - cv.unwrap());
                    } else {
                        ops.push_back(op);
                    }
                }
            }
            Operation::Sub(a, b, c) => {
                let av = vars.get(&a);
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                if bv.is_some() && cv.is_some() {
                    vars.insert(a, bv.unwrap() - cv.unwrap());
                } else {
                    if av.is_some() && bv.is_some() {
                        vars.insert(c, bv.unwrap() - av.unwrap());
                    } else if av.is_some() && cv.is_some() {
                        vars.insert(b, av.unwrap() + cv.unwrap());
                    } else {
                        ops.push_back(op);
                    }
                }
            }
            Operation::Mul(a, b, c) => {
                let av = vars.get(&a);
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                if bv.is_some() && cv.is_some() {
                    vars.insert(a, bv.unwrap() * cv.unwrap());
                } else {
                    if av.is_some() && bv.is_some() {
                        vars.insert(c, av.unwrap() / bv.unwrap());
                    } else if av.is_some() && cv.is_some() {
                        vars.insert(b, av.unwrap() / cv.unwrap());
                    } else {
                        ops.push_back(op);
                    }
                }
            }
            Operation::Div(a, b, c) => {
                let av = vars.get(&a);
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                if bv.is_some() && cv.is_some() {
                    vars.insert(a, bv.unwrap() / cv.unwrap());
                } else {
                    if av.is_some() && bv.is_some() {
                        vars.insert(c, bv.unwrap() / av.unwrap());
                    } else if av.is_some() && cv.is_some() {
                        vars.insert(b, av.unwrap() * cv.unwrap());
                    } else {
                        ops.push_back(op);
                    }
                }
            }
            Operation::RootEq(b, c) => {
                let bv = vars.get(&b);
                let cv = vars.get(&c);
                match (bv, cv) {
                    (Some(x), None) => {
                        vars.insert(c, *x);
                        ()
                    }
                    (None, Some(x)) => {
                        vars.insert(b, *x);
                        ()
                    }
                    _ => ops.push_back(op),
                }
            }
        }
    }

    println!("{:?}", vars.get(&humn).unwrap());
}

fn main() {
    let input = include_str!("../../../input/21.txt");
    part1(input);
    part2(input);
}
