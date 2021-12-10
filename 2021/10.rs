use std::fs;


fn parse_line(line: &str) -> u32 {
    let mut parse_stack: Vec<char> = Vec::new();
    for c in line.chars().enumerate() {
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

fn parse_lineb(line: &str) -> Option<u64> {
    let mut parse_stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => parse_stack.push(c),
            ')' => if *parse_stack.last().unwrap() == '(' { parse_stack.pop(); } else { return None; },
            ']' => if *parse_stack.last().unwrap() == '[' { parse_stack.pop(); } else { return None; },
            '}' => if *parse_stack.last().unwrap() == '{' { parse_stack.pop(); } else { return None; },
            '>' => if *parse_stack.last().unwrap() == '<' { parse_stack.pop(); } else { return None; },
            _ => panic!()
        };
    }
    // either incomplete or well formed line
    let mut acc = 0;
    while let Some(c) = parse_stack.pop() {
        acc = match c {
            '(' => acc * 5 + 1,
            '[' => acc * 5 + 2,
            '{' => acc * 5 + 3,
            '<' => acc * 5 + 4,
            x => panic!("panic on char {}", x)
        };
    }
    Some(acc)
}

fn part_b(input: &String) -> u64 {
    let mut scores = input.lines().filter_map(|line| parse_lineb(line)).collect::<Vec<u64>>();
    scores.sort();
    scores[(scores.len()-1)/2]
}

fn main() {
    let input: String = fs::read_to_string("10.txt")
        .expect("Something went wrong reading the file");
    
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
