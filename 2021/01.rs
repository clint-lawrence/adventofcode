use std::fs;

fn part_a(input: &String) -> i32 {
    let depths: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let pairs = depths.iter().zip(depths.iter().skip(1));
    pairs.map(|(a, b)| if b > a { 1 } else { 0 }).sum()
}

fn part_b(input: &String) -> i32 {
    let depths: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let pairs = depths.iter().zip(depths.iter().skip(3));
    pairs.map(|(a, b)| if b > a { 1 } else { 0 }).sum()
}

fn main() {
    let input: String =
        fs::read_to_string("01.txt").expect("Something went wrong reading the file");

    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
