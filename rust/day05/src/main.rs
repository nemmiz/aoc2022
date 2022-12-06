fn run_instructions(
    stacks: &Vec<Vec<u8>>,
    instructions: &Vec<Vec<usize>>,
    swap: fn(&mut Vec<Vec<u8>>, usize, usize, usize),
) {
    let mut stacks = stacks.clone();
    for inst in instructions {
        swap(&mut stacks, inst[0], inst[1], inst[2]);
    }

    let result = stacks
        .iter()
        .map(|stack| *stack.last().unwrap() as char)
        .collect::<String>();
    println!("{}", result);
}

fn swap_part1(stacks: &mut Vec<Vec<u8>>, amount: usize, from: usize, to: usize) {
    for _ in 0..amount {
        let byte = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(byte);
    }
}

fn swap_part2(stacks: &mut Vec<Vec<u8>>, amount: usize, from: usize, to: usize) {
    let mut copy = Vec::new();
    for _ in 0..amount {
        copy.push(stacks[from - 1].pop().unwrap());
    }
    for &item in copy.iter().rev() {
        stacks[to - 1].push(item);
    }
}

fn main() {
    let input = include_str!("../../../input/05.txt");
    let stack_lines: Vec<&[u8]> = input
        .lines()
        .take_while(|line| line.contains('['))
        .map(|line| line.as_bytes())
        .collect();

    let num_stacks = (stack_lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); num_stacks];

    for &line in stack_lines.iter().rev() {
        for i in (1..line.len()).step_by(4) {
            let index = (i - 1) / 4;
            let byte = line[i];
            if byte != 32 {
                stacks[index].push(line[i]);
            }
        }
    }

    let instructions = input
        .lines()
        .skip_while(|line| !line.starts_with("move"))
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    run_instructions(&stacks, &instructions, swap_part1);
    run_instructions(&stacks, &instructions, swap_part2);
}
