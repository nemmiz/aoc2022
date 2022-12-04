fn main() {
    let input = include_str!("../../../input/04.txt");
    let nums: Vec<i32> = input
        .split(&[',', '-', '\n'])
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let part1 = nums
        .chunks(4)
        .filter(|&chunk| {
            (chunk[0] >= chunk[2] && chunk[1] <= chunk[3])
                || (chunk[2] >= chunk[0] && chunk[3] <= chunk[1])
        })
        .count();

    let part2 = nums
        .chunks(4)
        .filter(|&chunk| chunk[0] <= chunk[3] && chunk[1] >= chunk[2])
        .count();

    println!("{} {}", part1, part2);
}
