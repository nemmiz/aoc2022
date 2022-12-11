use std::str;

fn check_cycle(cycle: i32, x: i32) -> i32 {
    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => cycle * x,
        _ => 0,
    }
}

fn draw_pixel(display: &mut Vec<u8>, cycle: i32, x: i32) {
    let dispx = (cycle - 1) % 40;
    if dispx >= x - 1 && dispx <= x + 1 {
        display.push(b'#');
    } else {
        display.push(b'.');
    }
}

fn draw_display(display: &Vec<u8>) {
    for line in display.chunks(40) {
        let line = str::from_utf8(line).unwrap();
        println!("{}", line);
    }
}

fn main() {
    let input = include_str!("../../../input/10.txt");
    let mut cycle = 1;
    let mut x = 1;
    let mut result = 0;
    let mut display: Vec<u8> = Vec::new();

    for line in input.lines() {
        if line.starts_with("noop") {
            draw_pixel(&mut display, cycle, x);
            cycle += 1;
            result += check_cycle(cycle, x);
        } else if line.starts_with("addx") {
            draw_pixel(&mut display, cycle, x);
            cycle += 1;
            result += check_cycle(cycle, x);
            draw_pixel(&mut display, cycle, x);
            cycle += 1;

            let n = line.split_at(5).1.parse::<i32>().unwrap();
            x += n;

            result += check_cycle(cycle, x);
        }
    }

    println!("{}", result);
    draw_display(&display);
}
