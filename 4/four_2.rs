use std::fs;


fn main() {
    let path = "input.txt";

    let mut output: u64 = 0;
    let mut nth_pass: u64 = 1;

    let mut lines: Vec<Vec<char>> = txt_to_vec(path).into_iter().map(|line| line.chars().collect()).collect();
    let len = 139;

    while nth_pass != 0 {

        nth_pass = 0;

        for row in 0..len {
            for col in 0..len {
                let character = lines[row][col];

                if character != '@' {
                    continue;
                }

                if valid(&mut lines, row, col) {
                    output += 1;
                    nth_pass += 1;
                }
            }

        }

    }
    println!("{output}");
}

fn valid(lines: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut count = 0;

    // check all surrounding cells (8 total)
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue; // skip itself
            }

            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                if lines[nr as usize][nc as usize] == '@' {
                    count += 1;
                    if count >= 4 {
                        return false;
                    }
                }
            }
        }
    }

    if count < 4 {
        lines[row][col] = '.';
        return true;
    }

    return false;
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
