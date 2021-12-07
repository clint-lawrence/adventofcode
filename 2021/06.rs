use std::fs;

// 375482
fn part_a(input: &String) -> u32 {
    let mut fishies: Vec<u32> = input.trim().split(",").map(|x| x.parse().unwrap()).collect();
    for _day in 0..80 {
        let mut new = 0;
        fishies = fishies.iter().map(|&fish| {
            if fish == 0 {
                new += 1;
                6
            }
            else {
                fish - 1
            }
        })
        .collect::<Vec<u32>>();
        fishies.extend(vec![8; new].iter());
    }
    fishies.len() as u32
}

// 1689540415957
fn part_b(input: &String) -> u64 {
    let init_fishies: Vec<u64> = input.trim().split(",").map(|x| x.parse().unwrap()).collect();
    let mut fishies = [0_u64; 9];
    for fish in init_fishies {
        fishies[fish as usize] = fishies[fish as usize] + 1;
    }
    for _day in 0..256 {
        let new_babies = fishies[0];
        fishies[0] = fishies[1];
        fishies[1] = fishies[2];
        fishies[2] = fishies[3];
        fishies[3] = fishies[4];
        fishies[4] = fishies[5];
        fishies[5] = fishies[6];
        fishies[6] = fishies[7] + new_babies;
        fishies[7] = fishies[8];
        fishies[8] = new_babies;
    }
    fishies.iter().sum()
}

fn main() {
    let input: String =
        fs::read_to_string("06.txt").expect("Something went wrong reading the file");

    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
    // println!("Part B: {}", part_b(&"3,4,3,1,2".to_string()));
}
