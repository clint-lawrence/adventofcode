use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn explore(map: &HashMap<String, Vec<String>>, current: &str, visited: Vec<String>, extra_small: bool) -> u32 {
    if current == "end" {
        // println!("{:?} end", visited);
        return 1;
    }
    let mut paths = 0;
    
    for connected in map.get(current).unwrap() {
        let small = *connected == connected.to_lowercase();
        let connected_is_normal_small = small && !visited.contains(connected);
        let connected_is_extra_small = small
            && visited.contains(connected)
            && extra_small
            && *connected != "start"
            && *connected != "end";

        if connected_is_normal_small || connected_is_extra_small || !small {
                let mut v = visited.clone();
                v.push(connected.to_string());
                paths += explore(
                    map,
                    connected,
                    v,
                    extra_small && ! connected_is_extra_small
                );                
        }
        else {
            // println!("{:?} X", &visited);    
        }
    }
    paths
}

fn part_a(input: &String) -> u32 {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (a, b) in input.lines().map(|line| line.trim().split_once("-").unwrap()) {
        map.entry(a.to_string()).and_modify(|e| e.push(b.to_string())).or_insert(vec![b.to_string()]);
        map.entry(b.to_string()).and_modify(|e| e.push(a.to_string())).or_insert(vec![a.to_string()]);
    }
    explore(&map, "start", vec!["start".to_string()], false)
}

fn part_b(input: &String) -> u32 {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (a, b) in input.lines().map(|line| line.trim().split_once("-").unwrap()) {
        map.entry(a.to_string()).and_modify(|e| e.push(b.to_string())).or_insert(vec![b.to_string()]);
        map.entry(b.to_string()).and_modify(|e| e.push(a.to_string())).or_insert(vec![a.to_string()]);
    }
    explore(&map, "start", vec!["start".to_string()], true)
}

fn main() {
    let input: String = fs::read_to_string("12.txt")
        .expect("Something went wrong reading the file");
    
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
