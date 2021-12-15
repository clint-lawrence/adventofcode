use std::cmp::max;
use std::collections::HashSet;
use std::fs;

enum Fold {
    X(u32),
    Y(u32),
}

fn page_size(points: &HashSet<(u32, u32)>) -> (u32, u32) {
    points
        .iter()
        .fold((0, 0), |acc, (x, y)| (max(acc.0, *x), max(acc.1, *y)))
}

fn print_page(points: &HashSet<(u32, u32)>) {
    let (w, h) = page_size(&points);
    for row in 0..=h {
        let line = (0..=w)
            .map(|x| if points.contains(&(x, row)) { "#" } else { " " })
            .collect::<String>();
        println!("{}", line);
    }
}

fn fold_paper(points: HashSet<(u32, u32)>, fold_type: Fold) -> HashSet<(u32, u32)> {
    let (w, h) = page_size(&points);
    let (f, l) = match fold_type {
        Fold::X(f) => (f, w),
        Fold::Y(f) => (f, h),
    };

    points
        .iter()
        .map(|(x, y)| {
            let (x, y) = (*x, *y);

            // swap if folding on y
            let (mut x, y) = match fold_type {
                Fold::X(_) => (x, y),
                Fold::Y(_) => (y, x),
            };
            // Do everything as if we're folding x
            if 2 * f >= l {
                if x > f {
                    x = 2 * f - x;
                }
            } else {
                // The folded section is longer than the unfolded
                // section. Not sure if this is needed in general
                // Isn't needed for my input...
                if x > f {
                    x = l - x;
                } else {
                    x = x + l - 2 * f;
                }
            }
            // swap back
            match fold_type {
                Fold::X(_) => (x, y),
                Fold::Y(_) => (y, x),
            }
        })
        .collect()
}

// 807
fn part_a(input: &String) -> u32 {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points: HashSet<(u32, u32)> = points
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let fold = folds
        .lines()
        .map(|line: &str| {
            match line
                .strip_prefix("fold along ")
                .unwrap()
                .split_once("=")
                .unwrap()
            {
                ("x", f) => Fold::X(f.parse().unwrap()),
                ("y", f) => Fold::Y(f.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .next()
        .unwrap();

    let points = fold_paper(points, fold);

    points.len() as u32
}

// #     ##  #  # ####  ##  #  # ####   ##
// #    #  # #  # #    #  # #  # #       #
// #    #    #### ###  #    #  # ###     #
// #    # ## #  # #    # ## #  # #       #
// #    #  # #  # #    #  # #  # #    #  #
// ####  ### #  # ####  ###  ##  ####  ##

fn part_b(input: &String) -> u32 {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points: HashSet<(u32, u32)> = points
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    // fold along x=655
    // fold along y=447
    let folds = folds.lines().map(|line: &str| {
        match line
            .strip_prefix("fold along ")
            .unwrap()
            .split_once("=")
            .unwrap()
        {
            ("x", f) => Fold::X(f.parse().unwrap()),
            ("y", f) => Fold::Y(f.parse().unwrap()),
            _ => unreachable!(),
        }
    });

    let code = folds.fold(points, |acc, f| fold_paper(acc, f));
    print_page(&code);
    0
}

fn main() {
    let input: String =
        fs::read_to_string("13.txt").expect("Something went wrong reading the file");

    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
