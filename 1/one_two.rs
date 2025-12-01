use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut current_num: u32 = 50;
    let mut zero_count: u32 = 0;

    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;

        let (direction, mut distance) = parse_instruction(&line)?;

        while distance >= 100 {
            distance -= 100;
            zero_count += 1;
        }

        let next = next_num(current_num, direction, distance);

        if direction == 'L' && next > current_num && current_num != 0{
            zero_count += 1;
        } else if next == 0 {
            zero_count += 1;
        } else if direction == 'R' && next < current_num && current_num != 0 {
            zero_count += 1;
        }

        current_num = next;
    }

    println!("Final number: {}", current_num);
    println!("Zeroes encountered: {}", zero_count);
    Ok(())
}

fn parse_instruction(s: &str) -> Result<(char, u32), io::Error> {
    let mut chars = s.chars();

    let direction = chars.next().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Empty line")
    })?;

    let distance_str: String = chars.collect();

    let distance: u32 = distance_str.parse().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Invalid distance in '{}'", s))
    })?;

    Ok((direction, distance))
}

fn next_num(current: u32, direction: char, distance: u32) -> u32 {
    let max = 100;

    match direction {
        'L' => (current + max - distance % max) % max,
        'R' => (current + distance) % max,
        _ => panic!("Invalid direction '{}'", direction),
    }
}

