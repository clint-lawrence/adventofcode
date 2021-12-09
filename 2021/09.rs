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


fn count_fill(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    const FILL: u32 = 100;

    let mut count = 0;

    if map[y][x] == 9 || map[y][x] == FILL {
        return 0;
    }
    map[y][x] = FILL;
    count += 1;

    if x > 0 {
        count += count_fill(map, x - 1, y);
    }
    if x + 1 < map[0].len() {
        count += count_fill(map, x + 1, y);
    }
    if y > 0 {
        count += count_fill(map, x, y - 1);
    }
    if y +1 < map.len() {
        count += count_fill(map, x, y + 1);
    }
    count
}

fn part_b(input: &String) -> u32 {
    let mut map: Vec<Vec<u32>> = input
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
    
    let mut fill_counts = minima
        .iter()
        .map(|(x, y)| count_fill(&mut map, *x, *y))
        .collect::<Vec<u32>>();
    fill_counts.sort();
    fill_counts[(fill_counts.len()-3)..].iter().product()
        
    
}

fn main() {
    let input: String =
        fs::read_to_string("09.txt").expect("Something went wrong reading the file");

    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
