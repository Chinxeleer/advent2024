fn main() {
    let input = include_str!("../../assets/input2.txt");
    let mut count = 0;
    for i in input.lines() {
        let vec_num = string_to_vec(i);
        if is_creasing(&vec_num) && within_radius(&vec_num) {
            count += 1;
        }
    }

    println!("The number is {} ", count);
}

fn string_to_vec(input: &str) -> Vec<u32> {
    let raw = input
        .split_whitespace()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    raw
}

fn is_creasing(list: &Vec<u32>) -> bool {
    let decrease = list.iter().is_sorted_by(|a, b| a > b);
    let increase = list.iter().is_sorted_by(|a, b| a < b);

    if decrease || increase {
        return true;
    }

    false
}

fn within_radius(list: &Vec<u32>) -> bool {
    let mut iter = list.iter().peekable();

    while let Some(current) = iter.next() {
        if let Some(&next) = iter.peek() {
            if (next.abs_diff(*current)) < 1 || (next.abs_diff(*current)) > 3 {
                return false;
            }
        }
    }

    true
}
