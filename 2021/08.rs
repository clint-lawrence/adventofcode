use std::fs;

// 0 - 6
// 1 - 2
// 2 - 5
// 3 - 5
// 4 - 4
// 5 - 5
// 6 - 6
// 7 - 3
// 8 - 7
// 9 - 6

// 288
fn part_a(input: &String) -> i32 {
    input
        .lines()
        .map(|line| line.split(" | ").skip(1).next().unwrap().split(" "))
        .flatten()
        .fold(0, |acc, output| match output.len() {
            2 | 3 | 4 | 7 => acc + 1,
            _ => acc,
        })
}

fn part_b(input: &String) -> i32 {
    0
}

fn main() {
    let input: String =
        fs::read_to_string("08.txt").expect("Something went wrong reading the file");

    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
