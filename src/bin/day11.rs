fn main() {
    let input = include_str!("../../assets/input1.txt");
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        split_input(line, &mut left_list, &mut right_list);
    }

    left_list.sort();
    right_list.sort();

    assert!(left_list.len() == right_list.len());

    let mut distance = 0;
    for i in 0..left_list.len() {
        distance += left_list[i].abs_diff(right_list[i]);
    }

    println!("Distance: {}", distance);
}

fn split_input(input: &str, left_list: &mut Vec<u32>, right_list: &mut Vec<u32>) {
    let raw_input: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    left_list.push(raw_input[0]);
    right_list.push(raw_input[1]);
}
