use std::fs;

fn fuel_cost_a(positions: &Vec<i32>, target: i32) -> i32 {
    positions.iter().map(|&pos| (pos - target).abs()).sum()
}


// 355764
fn part_a(input: &String) -> i32 {
    let positions: Vec<i32> = input.split(",").map(|value| value.parse().unwrap()).collect();
    let max_pos = *positions.iter().max().unwrap();
    let min_pos = *positions.iter().min().unwrap();
    let mut prev_target_cost = positions.len() as i32 * max_pos;
    for target in min_pos..max_pos {
        let cost = fuel_cost_a(&positions, target);
        if cost > prev_target_cost {
            break;
        }
        else {
            prev_target_cost = cost;
        }
    }
    prev_target_cost
}

fn fuel_cost_b(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .map(|&pos| (pos - target).abs())
        .map(|diff| diff * (diff+1) / 2)
        .sum()
}

// 99634572
fn part_b(input: &String) -> i32 {
    let positions: Vec<i32> = input.split(",").map(|value| value.parse().unwrap()).collect();
    let max_pos = *positions.iter().max().unwrap();
    let min_pos = *positions.iter().min().unwrap();
    
    // Find the starting point. This just sneaks into a i32.
    let mut prev_target_cost = positions.len() as i32 * max_pos * (max_pos/2);
    for target in min_pos..max_pos {
        let cost = fuel_cost_b(&positions, target);
        if cost > prev_target_cost {
            break;
        }
        else {
            prev_target_cost = cost;
        }
    }
    prev_target_cost

}

fn main() {
    let input: String = fs::read_to_string("07.txt")
        .expect("Something went wrong reading the file");
    
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}