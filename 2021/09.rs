use std::fs;

fn part_a(input: &String) -> u32 {
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();
    let size_y: isize = map.len() as isize;
    let size_x: isize = map[0].len() as isize;
    let mut minima: Vec<(usize, usize)> = Vec::new();

    let lookup = |x: isize, y: isize| -> u32  {
        if x < 0 || y < 0 || x >= size_x || y >= size_y {
            // return the max hight, so local minimum on the edge are detected
            return 9
        }
        map[y as usize][x as usize]
    };
    
    for y in 0..size_y {
        for x in 0..size_x {
            let val = lookup(x, y);
            if lookup(x-1, y) > val
                && lookup(x+1, y) > val
                && lookup(x, y-1) > val
                && lookup(x, y+1) > val {
                    minima.push((x as usize, y as usize));
                }
        }
    }
    minima.into_iter().fold(0, |acc, (x, y)| acc + map[y][x] + 1)
    
}

fn part_b(input: &String) -> i32 {
    0
}

fn main() {
    let input: String =
        fs::read_to_string("09.txt").expect("Something went wrong reading the file");

    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
