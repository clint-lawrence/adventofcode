use std::fs;

fn bit_counts(values: &Vec<&str>) -> Vec<u32> {
    let digits = *&values[0].len() as u32;
    let mut counts: Vec<u32>= vec![0; digits as usize];
    for val in values{
        for (i, chr) in val.chars().map(|x| x.to_digit(2).unwrap()).enumerate() {
            counts[i] += chr;
        }
    }
    counts
}

// 1082324
fn part_a(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let digits = *&lines[0].len() as u32;
    let threshold = (lines.len() / 2) as u32;
    let gamma: u32 = bit_counts(&lines).iter()
        .enumerate()
        .map(|(i,x)| if x > &threshold {1<<(digits-i as u32 -1)} else {0})
        .sum();
    
    gamma * (!gamma & 2_u32.pow(digits) - 1)
}


fn filter(values: &Vec<u32>,  bit: u32, invert: bool) -> Vec<u32> {
    let threshold = (values.len() + 1) / 2;
    let bit_count = values.iter().fold(0, |acc, x| if bit & x == 0 { acc } else { acc + 1 });
    let test_val;
    if ((bit_count >= threshold) && !invert)
        || ((bit_count < threshold) && invert) {
        test_val = bit;
    }
    else {
        test_val = 0;
    }
    values.iter().filter(|&&x| (x & bit == test_val)).copied().collect()
}

fn find_value(digits: u32, init_values: &Vec<u32>, invert: bool) -> u32 {
    let mut current_bit = 2_u32.pow(digits-1);
    let mut values = init_values.clone();

    while current_bit > 0 {
        values = filter(&values, current_bit, invert);
        current_bit = current_bit >> 1;
        if values.len() == 1 {
            return values.pop().unwrap();
        }
    }
    panic!();
}

fn part_b(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let digits = *&lines[0].len() as u32;
    let values: Vec<u32> = lines.iter().map(|line| u32::from_str_radix(line, 2).unwrap()).collect();

    let oxygen = find_value(digits, &values, false);
    let co2 = find_value(digits, &values, true);
    oxygen * co2
}

fn main() {
    let input: String = fs::read_to_string("03.txt")
        .expect("Something went wrong reading the file");
    
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
