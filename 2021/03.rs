use std::fs;

// 1082324
fn part_a(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let digits = *&lines[0].len() as u32;
    let mut counts: Vec<u32>= Vec::new();
    counts.resize(digits as usize, 0);
    let threshold = (lines.len() / 2) as u32;
    for line in lines{
        for (i, chr) in line.chars().map(|x| x.to_digit(2).unwrap()).enumerate() {
            counts[i] += chr;
        }
    }
    let gamma: u32 = counts.iter()
        .enumerate()
        .map(|(i,x)| if x > &threshold {1<<(digits-i as u32 -1)} else {0})
        .sum();
    
    gamma * (!gamma & 2_u32.pow(digits) - 1)
}

fn part_b(input: &String) -> u32 {
    0
}

fn main() {
    let input: String = fs::read_to_string("03.txt")
        .expect("Something went wrong reading the file");
    
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
