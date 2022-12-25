fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut decimal = 0;
    let mut place = 1;
    for c in snafu.chars().rev() {
        decimal += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        } * place;
        place *= 5;
    }
    decimal
}

fn print_snafu(decimal: i64) {
    if decimal != 0 {
        print_snafu((decimal + 2) / 5);
        print!(
            "{}",
            match (decimal + 2) % 5 {
                0 => '=',
                1 => '-',
                2 => '0',
                3 => '1',
                4 => '2',
                _ => panic!(),
            }
        );
    }
}

fn main() {
    let input = include_str!("../../../input/25.txt");
    let sum = input.lines().map(|line| snafu_to_decimal(line)).sum();
    print_snafu(sum);
    println!();
}
