use std::fs;

fn main() {
    let path = "input.txt";

    let lines = txt_to_vec(path);

    let mut output: u128 = 0;

    for line in lines.iter() {
        output += parse_line(line);
        println!("{}", parse_line(line));
    }

    println!("{output}");
}

fn txt_to_vec(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Wrong path!");
    
    let items: Vec<String> = content
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    return items;
}

fn parse_line(line: &String) -> u128 {

    let bytes = line.as_bytes();

    let mut output: u128 = 0;

    let mut cut_off = 12;

    let mut start = 0;

    let length = line.len();

    while cut_off > 0 {
        let mut current_digit = 0;
        println!("Entering for loop: start: {start}, cuttoff: {cut_off}, length - cut_off = {}", length - cut_off);
        for i in start..(length - cut_off + 1) {
            let digit = (bytes[i] - b'0') as u128;

            if digit > current_digit {
                start = i + 1;
                current_digit = digit;
            }
        }
        output += current_digit;
        if cut_off > 1 {
            output *= 10;
        }
        cut_off -= 1;
    }
    output
}
