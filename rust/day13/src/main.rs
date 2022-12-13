use serde_json::{json, Value};
use std::cmp::{min, Ordering};

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Array(arr1), Value::Array(arr2)) => match (arr1.len(), arr2.len()) {
            (0, 0) => Ordering::Equal,
            (0, _) => Ordering::Less,
            (_, 0) => Ordering::Greater,
            (x, y) => {
                for i in 0..min(x, y) {
                    let ordering = compare(&arr1[i], &arr2[i]);
                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }
                x.cmp(&y)
            }
        },
        (Value::Array(arr), Value::Number(num)) => {
            if arr.is_empty() {
                Ordering::Less
            } else {
                let bb = json!([num.as_i64().unwrap()]);
                compare(a, &bb)
            }
        }
        (Value::Number(num), Value::Array(arr)) => {
            if arr.is_empty() {
                Ordering::Greater
            } else {
                let aa = json!([num.as_i64().unwrap()]);
                compare(&aa, b)
            }
        }
        (Value::Number(num1), Value::Number(num2)) => {
            num1.as_i64().unwrap().cmp(&num2.as_i64().unwrap())
        }
        _ => panic!(),
    }
}

#[derive(Eq)]
struct Packet {
    orig: &'static str,
    data: Value,
}

impl Packet {
    fn new(s: &'static str) -> Packet {
        Packet {
            orig: s,
            data: serde_json::from_str(s).unwrap(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(&self.data, &other.data)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.orig == other.orig
    }
}

fn main() {
    let input = include_str!("../../../input/13.txt");
    let mut lines: Vec<Packet> = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| Packet::new(line))
        .collect();

    let part1: usize = lines
        .chunks(2)
        .enumerate()
        .filter(|(_, chunk)| compare(&chunk[0].data, &chunk[1].data) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    lines.push(Packet::new("[[2]]"));
    lines.push(Packet::new("[[6]]"));
    lines.sort();

    let a = lines.iter().position(|p| p.orig == "[[2]]").unwrap() + 1;
    let b = lines.iter().position(|p| p.orig == "[[6]]").unwrap() + 1;

    println!("{} {}", part1, a * b);
}
