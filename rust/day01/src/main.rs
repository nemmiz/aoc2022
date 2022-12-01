fn main() {
    let input = include_str!("../../../input/01.txt");

    let mut calories = vec![0];
    for line in input.lines() {
        if line.trim().is_empty() {
            calories.push(0);
        } else {
            let n: i32 = line.parse::<i32>().unwrap();
            *calories.last_mut().unwrap() += n;
        }
    }

    let max = calories.iter().max().unwrap();
    println!("{max}");

    calories.sort();
    let len = calories.len();
    let last3 = &calories[len - 3..];
    let sumlast3: i32 = last3.iter().sum();
    println!("{sumlast3}");
}
