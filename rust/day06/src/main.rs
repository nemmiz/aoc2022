fn detect(input: &[u8], size: usize) -> usize {
    size + input
        .windows(size)
        .enumerate()
        .find(|(_, window)| !(1..window.len()).any(|i| window[i..].contains(&window[i - 1])))
        .unwrap()
        .0
}

fn main() {
    let input = include_bytes!("../../../input/06.txt");
    println!("{}", detect(input, 4));
    println!("{}", detect(input, 14));
}
