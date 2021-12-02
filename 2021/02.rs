use std::fs;

fn part_a(input: &String) -> u32 {
    let mut depth: u32 = 0;
    let mut distance: u32 = 0;

    for line in input.lines() {
        let (cmd, size_str) = line.split_once(" ").unwrap();
        let size: u32 = size_str.parse().unwrap();
        match cmd {
            "down" => depth += size,
            "up" => depth -= size,
            "forward" => distance += size,
            _ => unreachable!()
        }
    }
    depth * distance
}

fn part_b(input: &String) -> u32 {
    let mut depth: u32 = 0;
    let mut distance: u32 = 0;
    let mut aim: u32 = 0;

    for line in input.lines() {
        let (cmd, size_str) = line.split_once(" ").unwrap();
        let size: u32 = size_str.parse().unwrap();
        match cmd {
            "down" => aim += size,
            "up" => aim -= size,
            "forward" => {distance += size; depth += aim * size;}
            _ => unreachable!()
        }
    }
    depth * distance
}

fn main() {
    let input: String = fs::read_to_string("02.txt")
        .expect("Something went wrong reading the file");
    
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
