use std::fs;

fn part_a(input: &String) -> i32 {
    let depths: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let pairs = depths.iter().zip(depths[1..].iter());
    pairs.map(|(a, b)| if b > a { 1 } else { 0 }).sum()
}

fn part_b(input: &String) -> i32 {
    let depths: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let ave_depths: Vec<i32> = depths.windows(3).map(|x| x.iter().sum()).collect();
    let pairs = ave_depths.iter().zip(ave_depths[1..].iter());
    pairs.map(|(a, b)| if b > a { 1 } else { 0 }).sum()
}

fn main() {
    let input: String =
        fs::read_to_string("01.txt").expect("Something went wrong reading the file");

    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
