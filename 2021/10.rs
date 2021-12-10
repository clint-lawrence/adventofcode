use std::fs;
use std::collections::HashMap;


fn parse_line(line: &str) -> u32 {
    let mut parse_stack: Vec<char> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        match c {
            '(' | '[' | '{' | '<' => parse_stack.push(c),
            ')' => if *parse_stack.last().unwrap() == '(' { parse_stack.pop(); } else { return 3; },
            ']' => if *parse_stack.last().unwrap() == '[' { parse_stack.pop(); } else { return 57; },
            '}' => if *parse_stack.last().unwrap() == '{' { parse_stack.pop(); } else { return 1197; },
            '>' => if *parse_stack.last().unwrap() == '<' { parse_stack.pop(); } else { return 25137; },
            _ => panic!()
        };
    }
    // either incomplete or well formed line
    0
}

fn part_a(input: &String) -> u32 {
    input.lines().map(|line| parse_line(line)).sum()
}

fn part_b(input: &String) -> i32 {
    0
}

fn main() {
    let input: String = fs::read_to_string("10.txt")
        .expect("Something went wrong reading the file");
    
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
