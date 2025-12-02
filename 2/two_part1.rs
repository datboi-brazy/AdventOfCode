use std::fs;

fn main() -> std::io::Result<()> {
    let path = "input.txt";

    let mut solution: u64 = 0;

    let items = txt_to_vec(path);
        for item in &items {
            let (lower, upper) = parse_bounds(item);

            solution = calculate_invalid(solution, lower, upper);

        }
        println!("{solution}");
        


    Ok(())
}



fn txt_to_vec(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Wrong path!");
    
    let items: Vec<String> = content
        .trim()
        .split(',')
        .map(|s| s.to_string())
        .collect();

    return items;
}

fn parse_bounds(item: &str) -> (u64, u64) {
    let mut parts = item.split('-');

    let lower = parts.next().unwrap().trim().parse().unwrap();
    let upper = parts.next().unwrap().trim().parse().unwrap();

    (lower, upper)
}

fn calculate_invalid(solution: u64, lower: u64, upper: u64) -> u64 {
    let mut ret_val = solution;

     for i in lower..=upper {
        let element = i.to_string();
        let bytes = element.as_bytes();
        let length = element.len();
        if length % 2 != 0 {
            continue;
        }
//        for j in 0..(length / 2) {
//
//            if bytes[j] != bytes[length - 1 - j] {
//                continue 'num_loop;
//            }
//
//        }
        let half = length / 2;

        if bytes[..half] == bytes[half..] {
            ret_val += element.parse::<u64>().unwrap();
        }
    }
    ret_val
}
