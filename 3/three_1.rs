use std::fs;

fn main() {
    let path = "input.txt";

    let lines = txt_to_vec(path);

    let mut output = 0;

    for line in lines.iter() {
        output += parse_line(line);
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

fn parse_line(line: &String) -> u32 {
    let mut first_digit = 0;
    let mut second_digit = 0;
    
    let mut first_index = 0;

    let nums = line.as_bytes();

    let length = line.len();

    for i in first_index..(length-1) {
        if (nums[i] - b'0') as u8 > first_digit {
            first_digit = (nums[i] - b'0') as u8;
            first_index = i;
        }

    }
    for i in (first_index + 1)..length {
        if (nums[i] - b'0') as u8 > second_digit {
            second_digit = (nums[i] - b'0') as u8;
        }
    }

    return first_digit as u32 * 10 + second_digit as u32;

}
