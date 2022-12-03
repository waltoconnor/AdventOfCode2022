use std::{collections::HashSet, hash::Hash};

fn char_to_prio(letter: char) -> u32 {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    alphabet.iter().position(|r| *r == letter).unwrap() as u32 + 1
}

fn find_match(line: &str) -> char {
    let mut left: HashSet<char> = HashSet::new();
    let mut right: HashSet<char> = HashSet::new();

    let (left_str, right_str) = line.split_at(line.len()/2);
    for char in left_str.chars().into_iter() {
        left.insert(char);
    }

    for char in right_str.chars().into_iter() {
        right.insert(char);
    }

    *left.intersection(&right).next().unwrap()
}

fn find_matching(chunk: &[&str]) -> char {
    let mut a: HashSet<char> = HashSet::new();
    let mut b: HashSet<char> = HashSet::new();
    let mut c: HashSet<char> = HashSet::new();

    for ac in chunk[0].chars().into_iter() {
        a.insert(ac);
    }

    for bc in chunk[1].chars().into_iter() {
        b.insert(bc);
    }

    for cc in chunk[2].chars().into_iter() {
        c.insert(cc);
    }

    let i1: HashSet<char> = a.intersection(&b).map(|c| *c).collect();

    *(i1.intersection(&c).next().unwrap())
}

fn main() {
    let input: Vec<&str> = include_str!("../input/day3.txt").lines().collect();
    let pt1: u32 = input.iter().map(|line| char_to_prio(find_match(line))).sum();
    let pt2: u32 = input.chunks(3).map(|chunk| find_matching(chunk)).map(|c| char_to_prio(c)).sum();
    println!("pt1: {}", pt1);
    println!("pt2: {}", pt2);

}
