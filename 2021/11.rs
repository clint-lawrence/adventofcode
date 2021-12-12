use std::fs;
use std::collections::HashSet;

const DIM: usize = 10;

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let xr = match x { 0=> 0..=1, 9 => -1..=0, _ => -1..=1 };
    let yr = match y { 0=> 0..=1, 9 => -1..=0, _ => -1..=1 };

    yr.flat_map(move |yi| xr.clone().map(move |xi|
        ((x as isize + xi) as usize, (y as isize+yi) as usize)))
        .filter(move |(xi, yi)| !((x == *xi) && (y == *yi)))
}

fn do_generation(octopi: &mut [[u32; DIM]; DIM]) -> u32 {
    let mut reset_list: HashSet<(usize, usize)>= HashSet::new();
    let mut flash: Vec<(usize, usize)> = Vec::new();
    let mut flash_count = 0;

    // increase energey of every octopus by 1
    for x in 0..DIM {
        for y in 0..DIM {
            octopi[y][x] += 1;
            if octopi[y][x] > 9 {
                // if greater than 9, queue it to flash
                flash.push((x, y));
                reset_list.insert((x, y));
            }
        }
    }

    // do the flash and distribute energy
    while let Some((flash_x, flash_y)) = flash.pop() {
        flash_count += 1;

        for (x, y) in neighbours(flash_x, flash_y) {
            octopi[y][x] += 1;

            if octopi[y][x] > 9 &&  !reset_list.contains(&(x,y)) {
                flash.push((x,y));
                reset_list.insert((x, y));
            }
        }
    }

    // reset the energy for each octopus that flashed
    for (reset_x, reset_y) in reset_list {
        octopi[reset_y][reset_x] = 0;
    }
    flash_count
}

fn _print_them(octopi: & [[u32; DIM]; DIM]){
    println!("");
        for row in octopi{
            println!("{:?}", row);
        }
}

fn part_a(input: &String) -> u32 {
    let mut octopi = [[0_u32; DIM]; DIM];
    for (i,line) in input.lines().enumerate() {
        for (j, value) in line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect(&format!("can't convert char '{}'", c)))
            .enumerate(){
            octopi[i][j] = value;
        }
    }
    let mut flashes = 0;
    for _generation in 0..100 {
        flashes += do_generation(&mut octopi);
    }
    flashes
}



fn part_b(input: &String) -> i32 {
    let mut octopi = [[0_u32; DIM]; DIM];
    for (i,line) in input.lines().enumerate() {
        for (j, value) in line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect(&format!("can't convert char '{}'", c)))
            .enumerate(){
            octopi[i][j] = value;
        }
    }
    for generation in 1.. {
        do_generation(&mut octopi);
        if octopi.iter().flat_map(|row| row.iter()).all(|energy| *energy == 0) {
            return generation;
        }
    }
    unreachable!();

}

fn main() {
    let input: String = fs::read_to_string("11.txt")
        .expect("Something went wrong reading the file");

    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
