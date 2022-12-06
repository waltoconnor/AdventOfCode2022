use std::collections::HashSet;

//returns true if there is a duplicate
fn check_duplicates(input: &[char]) -> bool {
    let mut set = HashSet::new();
    for c in input.iter(){
        if set.contains(c) {
            return true;
        }
        set.insert(*c);
    }
    return false;
}

fn find_first_n_unique_chars(input: &str, len: usize) -> usize {
    let result = input
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .enumerate()
        .map(|(place, window)| (place, check_duplicates(window)))
        .find(|(_, status)| *status == false);
        
    match result {
        None => panic!("no solution found"),
        Some((place, _)) => place + len
    }
}

fn main() {
    let input = include_str!("../input/day6.txt").lines().collect::<Vec<&str>>();
    let res = find_first_n_unique_chars(input[0], 4);
    println!("A: {}", res);
    let res2 = find_first_n_unique_chars(input[0], 14);
    println!("B: {}", res2);
}