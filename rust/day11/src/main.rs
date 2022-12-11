#[derive(Debug, Clone)]
enum Operation {
    Square,
    Add(i64),
    Mul(i64),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisor: i64,
    if_true: usize,
    if_false: usize,
    inspections: i64,
}

impl Monkey {
    fn new(lines: &[&str]) -> Monkey {
        Monkey {
            items: lines[0]
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect(),
            operation: match lines[1]
                .strip_prefix("new = old ")
                .unwrap()
                .split_once(' ')
                .unwrap()
            {
                ("*", "old") => Operation::Square,
                ("+", n) => Operation::Add(n.parse::<i64>().unwrap()),
                ("*", n) => Operation::Mul(n.parse::<i64>().unwrap()),
                _ => panic!(),
            },
            divisor: lines[2]
                .strip_prefix("divisible by ")
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            if_true: lines[3]
                .strip_prefix("throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            if_false: lines[4]
                .strip_prefix("throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            inspections: 0,
        }
    }
}

fn simulate<F: Fn(i64) -> i64>(
    monkeys: &mut Vec<Monkey>,
    iterations: usize,
    reduction_fn: F,
) -> i64 {
    let mut tmp: Vec<(usize, i64)> = Vec::new();

    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            {
                let monkey = monkeys.get_mut(i).unwrap();

                for item in monkey.items.iter() {
                    let mut worry_level = match monkey.operation {
                        Operation::Square => item * item,
                        Operation::Add(x) => item + x,
                        Operation::Mul(x) => item * x,
                    };
                    worry_level = reduction_fn(worry_level);

                    if worry_level % monkey.divisor == 0 {
                        tmp.push((monkey.if_true, worry_level));
                    } else {
                        tmp.push((monkey.if_false, worry_level));
                    }
                    monkey.inspections += 1;
                }

                monkey.items.clear();
            }
            for (midx, item) in tmp.iter() {
                monkeys[*midx].items.push(*item);
            }
            tmp.clear();
        }
    }

    let mut tmp: Vec<i64> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    tmp.sort();
    tmp.reverse();
    tmp[0] * tmp[1]
}

fn main() {
    let input = include_str!("../../../input/11.txt");

    let mut monkeys1: Vec<Monkey> = input
        .lines()
        .map(|line| line.split_once(": "))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().1)
        .collect::<Vec<&str>>()
        .chunks(5)
        .map(|chunk| Monkey::new(chunk))
        .collect();

    let mut monkeys2 = monkeys1.clone();

    let reducer = monkeys2
        .iter()
        .map(|monkey| monkey.divisor)
        .reduce(|a, b| a * b)
        .unwrap();

    let part1 = simulate(&mut monkeys1, 20, |x| x / 3);
    let part2 = simulate(&mut monkeys2, 10000, |x| x % reducer);

    println!("{} {}", part1, part2);
}
