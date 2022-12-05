use std::collections::HashMap;
use std::collections::LinkedList;

fn parse_stacks(input: &str) -> HashMap<u32, LinkedList<char>> {
    let mut results = HashMap::new();

    let mut lines = input.lines().into_iter().collect::<Vec<&str>>();
    let labels = lines.pop().unwrap();

    for l in labels.split("   ").map(|c| c.trim().parse::<u32>().unwrap()) {
        results.insert(l, LinkedList::new());
    }

    for line in lines.iter().map(|l| l.chars().collect::<Vec<char>>()) {
        let mut cur_col: u32 = 0;
        for c in line.chunks(4){
            let val = match *c {
                ['[', l, ']', ' '] => Some(l),
                ['[', l, ']' ] => Some(l),
                _ => None
            };
            if let Some(r) = val { results.get_mut(&(cur_col + 1)).unwrap().push_front(r); };
            cur_col += 1;
        }
    }

    results
}

struct Move {
    count: u32,
    start: u32,
    to: u32
}

fn parse_moves(moves: &str) -> Vec<Move> {
    moves.lines().map(|l| { 
        let vals: Vec<&str> = l.split(" ").collect(); 
        let count = vals[1].parse().unwrap();
        let start = vals[3].parse().unwrap();
        let to = vals[5].parse().unwrap();
        Move{ count, start, to }
    }).collect()
}

fn apply_move(map: &mut HashMap<u32, LinkedList<char>>, mv: &Move){
    for _ in 0..mv.count {
        let v = map.get_mut(&mv.start).unwrap().pop_back().unwrap();
        map.get_mut(&mv.to).unwrap().push_back(v);
    }
}

fn apply_move_part_2(map: &mut HashMap<u32, LinkedList<char>>, mv: &Move) {
    let mut tmp = LinkedList::<char>::new();;
    for _ in 0..mv.count {
        let v = map.get_mut(&mv.start).unwrap().pop_back().unwrap();
        tmp.push_back(v);
        
    }

    for _ in 0..mv.count {
        let v = tmp.pop_back().unwrap();
        map.get_mut(&mv.to).unwrap().push_back(v);
    }
}

fn main(){
    let (stacks_str, moves_str) = include_str!("../input/day5.txt").split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stacks_str);
    let moves = parse_moves(moves_str);
    for m in moves.iter() {
        apply_move(&mut stacks, m);
    }

    println!("PART 1");
    for i in 1u32..=(stacks.len() as u32) {
        println!("{}", stacks.get(&i).unwrap().back().unwrap());
    }

    let mut stacks2 = parse_stacks(stacks_str);
    for m in moves.iter() {
        apply_move_part_2(&mut stacks2, m);
    }

    println!("PART 2");
    for i in 1u32..=(stacks2.len() as u32) {
        println!("{}", stacks2.get(&i).unwrap().back().unwrap());
    }
}