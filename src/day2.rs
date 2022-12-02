use std::panic;

enum Shape {
    Rock,
    Paper,
    Scissors
}

fn to_shape(shape: &str) -> Shape {
    match shape {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("INVALID SHAPE")
    }
}

fn parse_line(line: &str) -> (Shape, Shape) {
    if let [fst, snd] = line.split(" ").collect::<Vec<&str>>().as_slice() {
        (to_shape(fst), to_shape(snd))
    } else { 
        panic!("INVALID LINE");
    }
}


fn compute_score(theirs: &Shape, yours: &Shape) -> u32 {
    let pick_score = match yours {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3
    };

    let match_score = match (theirs, yours) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Scissors, Shape::Scissors) => 3,
        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Paper, Shape::Rock) => 0,
        (Shape::Scissors, Shape::Paper) => 0,
        (Shape::Rock, Shape::Scissors) => 0,
        _ => 6
    };
    pick_score + match_score
}

enum MatchResult {
    Win,
    Draw,
    Loss
}

fn to_match_result(res: &str) -> MatchResult {
    match res {
        "X" => MatchResult::Loss,
        "Y" => MatchResult::Draw,
        "Z" => MatchResult::Win,
        _ => { panic!("INVALID LINE"); }
    }
}

fn parse_line_pt2(line: &str) -> (Shape, MatchResult) {
    if let [fst, snd] = line.split(" ").collect::<Vec<&str>>().as_slice() {
        (to_shape(fst), to_match_result(snd))
    } else { 
        panic!("INVALID LINE");
    }
}

fn compute_score_pt2(theirs: &Shape, win: &MatchResult) -> u32 {
    match (theirs, win) {
        (Shape::Rock, MatchResult::Draw) => 1 + 3,
        (Shape::Paper, MatchResult::Draw) => 2 + 3,
        (Shape::Scissors, MatchResult::Draw) => 3 + 3,
        (Shape::Rock, MatchResult::Loss) => 3 + 0,
        (Shape::Paper, MatchResult::Loss) => 1 + 0,
        (Shape::Scissors, MatchResult::Loss) => 2 + 0,
        (Shape::Rock, MatchResult::Win) => 2 + 6,
        (Shape::Paper, MatchResult::Win) => 3 + 6,
        (Shape::Scissors, MatchResult::Win) => 1 + 6,
    }
}

fn main(){
    let lines = include_str!("../input/day2.txt").lines();
    let strats: Vec<(Shape, Shape)> = lines.clone().map(|line|  parse_line(line)).collect();
    let score: u32 = strats.iter().map(|(theirs, yours)| compute_score(theirs, yours)).sum();
    println!("A: {}", score);

    let strats_pt2: Vec<(Shape, MatchResult)> = lines.map(|line|  parse_line_pt2(line)).collect();
    let score_pt2: u32 = strats_pt2.iter().map(|(theirs, win)| compute_score_pt2(theirs, win)).sum();
    println!("B: {}", score_pt2);
}