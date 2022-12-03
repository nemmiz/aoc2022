fn main() {
    let input = include_str!("../../../input/02.txt");

    let mut score1 = 0;
    let mut score2 = 0;

    for line in input.lines() {
        let bytes = line.as_bytes();

        let (a, b) = match (bytes[0], bytes[2]) {
            (b'A', b'X') => (1 + 3, 3 + 0),
            (b'A', b'Y') => (2 + 6, 1 + 3),
            (b'A', b'Z') => (3 + 0, 2 + 6),
            (b'B', b'X') => (1 + 0, 1 + 0),
            (b'B', b'Y') => (2 + 3, 2 + 3),
            (b'B', b'Z') => (3 + 6, 3 + 6),
            (b'C', b'X') => (1 + 6, 2 + 0),
            (b'C', b'Y') => (2 + 0, 3 + 3),
            (b'C', b'Z') => (3 + 3, 1 + 6),
            _ => panic!(),
        };

        score1 += a;
        score2 += b;
    }

    println!("{} {}", score1, score2);
}
