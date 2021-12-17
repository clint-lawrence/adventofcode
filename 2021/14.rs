use std::fs;
use std::collections::HashMap;


type PairCount = HashMap<(char, char), u64>;
type Rules = HashMap<(char, char), char>;

fn do_one_generation(pair_counts: PairCount, rules: &Rules) -> PairCount {
    let mut updated_count = PairCount::new();

    for ((l, r), count) in pair_counts {
        let insert = *rules.get(&(l, r)).unwrap();
        *updated_count.entry((l, insert)).or_insert(0) += count;
        *updated_count.entry((insert, r)).or_insert(0) += count;
    }
    updated_count
}

fn solution(input: &str, generations: u32) -> u64 {
    let (template, rules) = input.split_once("\n\n").unwrap();
    
    // Example rule: FV -> C
    let rules: Rules = rules
        .split("\n")
        .map(|rule| rule.split_once(" -> ").unwrap())
        .map(|(pair, insert)| {
            let cs: Vec<_> = pair.chars().collect();
            ((cs[0], cs[1]), insert.chars().next().unwrap())
        })
        .collect();
    let start = &template.chars().next().unwrap();
    let end = &template.chars().last().unwrap();
    
    let mut counts = PairCount::new();
    for pair in template.chars().zip(template.chars().skip(1)) {
        counts.entry(pair)
            .and_modify(|e| *e += 1)
            .or_insert(1);    
    }

    for _step in 0..generations {
        counts = do_one_generation(counts, &rules);
    
    }
    
    let mut element_totals = HashMap::new();
    for ((l, r), count) in counts {
        *element_totals.entry(l).or_insert(0) += count;
        *element_totals.entry(r).or_insert(0) += count;
    }
    *element_totals.get_mut(start).unwrap() += 1;
    *element_totals.get_mut(end).unwrap() += 1;

    let mut totals: Vec<_> = element_totals.iter().map(|(c, count)| (count/2 , c)).collect();
    totals.sort();

    totals.last().unwrap().0 - totals.first().unwrap().0
}

// 3587
fn part_a(input: &str) -> u64 {
    solution(input, 10)
}

// 3906445077999
fn part_b(input: &str) -> u64 {
    solution(input, 40)
}

fn main() {
    let input: String = fs::read_to_string("14.txt")
        .expect("Something went wrong reading the file");
    
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
