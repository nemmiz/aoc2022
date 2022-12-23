fn solve(input: &str, factor: i64, iterations: usize) {
    let numbers: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * factor)
        .collect();

    let mut indices: Vec<usize> = (0..numbers.len()).collect();
    let len = numbers.len() as i64 - 1;

    for _ in 0..iterations {
        for (i, &n) in numbers.iter().enumerate() {
            let index = indices.iter().position(|&x| i == x).unwrap() as i64;
            let mut new_index = index + n;
            new_index = ((new_index % len) + len) % len;
            indices.remove(index as usize);
            indices.insert(new_index as usize, i as usize);
        }
    }

    let index = numbers.iter().position(|&n| n == 0).unwrap();
    let index = indices.iter().position(|&i| i == index).unwrap();
    let a = numbers[indices[(index + 1000) % numbers.len()]];
    let b = numbers[indices[(index + 2000) % numbers.len()]];
    let c = numbers[indices[(index + 3000) % numbers.len()]];

    println!("{}", a + b + c);
}

fn main() {
    let input = include_str!("../../../input/20.txt");
    solve(input, 1, 1);
    solve(input, 811589153, 10);
}
