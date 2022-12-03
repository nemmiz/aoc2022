fn common_byte(bytes: &[&[u8]]) -> u8 {
    *bytes[0]
        .iter()
        .find(|byte| bytes[1..].iter().all(|slice| slice.contains(&*byte)))
        .unwrap()
}

fn priority(byte: u8) -> i32 {
    let prio = match byte {
        b'a'..=b'z' => byte - b'a' + 1,
        b'A'..=b'Z' => byte - b'A' + 27,
        _ => 0,
    };
    prio.into()
}

fn part1(byte_lines: &Vec<&[u8]>) -> i32 {
    let mut sum: i32 = 0;

    for line in byte_lines {
        let mid = line.len() / 2;
        let bytes1 = &line[..mid];
        let bytes2 = &line[mid..];
        sum += priority(common_byte(&[bytes1, bytes2]));
    }

    sum
}

fn part2(byte_lines: &Vec<&[u8]>) -> i32 {
    let mut sum: i32 = 0;

    for chunk in byte_lines.chunks(3) {
        sum += priority(common_byte(chunk));
    }

    sum
}

fn main() {
    let input = include_str!("../../../input/03.txt");
    let byte_lines: Vec<&[u8]> = input
        .lines()
        .map(|line| line.trim_end().as_bytes())
        .collect();

    let sum1 = part1(&byte_lines);
    let sum2 = part2(&byte_lines);

    println!("{} {}", sum1, sum2);
}
