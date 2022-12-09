use std::collections::HashMap;

enum Dir {
    R,
    L,
    U,
    D
}

struct Step {
    dir: Dir,
    dist: i32
}

impl Step {
    fn from_str(s: &str) -> Self {
        if let [dir, dist, ..] = s.split(" ").take(2).collect::<Vec<&str>>().as_slice() {
            let dir_s = match *dir {
                "U" => Dir::U,
                "R" => Dir::R,
                "L" => Dir::L,
                "D" => Dir::D,
                _ => { panic!("Invalid direction"); }
            };
    
            let d = dist.parse::<i32>().unwrap();
            Step { dir: dir_s, dist: d}
         } 
         else { 
            panic!("Failed to split string"); 
        }
    }
}

fn get_input() -> Vec<Step> {
    include_str!("../input/day9.txt").lines().map(|s| Step::from_str(s)).collect()
}

fn range(start: i32, end: i32) -> Vec<i32> {
    if start < end { (start..=end).collect() } else { (end..=start).rev().collect() }
}

fn generate_new_positions(head_pos: (i32, i32), step: &Step) -> Vec<(i32, i32)> {
    let (x, y) = head_pos;
    match step.dir {
        Dir::U => range(y, y + step.dist).into_iter().map(|y_tmp| (x, y_tmp)).collect(),
        Dir::L => range(x, x - step.dist).into_iter().map(|x_tmp| (x_tmp, y)).collect(),
        Dir::R => range(x, x + step.dist).into_iter().map(|x_tmp| (x_tmp, y)).collect(),
        Dir::D => range(y, y - step.dist).into_iter().map(|y_tmp| (x, y_tmp)).collect(),
    }
}

fn get_new_tail_position(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    let (hx, hy) = head_pos;
    let (tx, ty) = tail_pos;

    if (hx - tx).abs().max((hy - ty).abs()) <= 1 { return (tx, ty); }

    
    let tx = if tx > hx { tx - 1 } else if tx < hx { tx + 1 } else { tx };
    let ty = if ty > hy { ty - 1 } else if ty < hy { ty + 1 } else { ty };
    (tx, ty)
}

fn simulate(steps: &Vec<Step>) -> u32 {
    let mut m = HashMap::<(i32, i32), bool>::new();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    m.insert(tail_pos, true);
    
    for step in steps.iter() {
        let positions = generate_new_positions(head_pos, step);
        for pos in positions.iter() {
            head_pos = *pos;
            tail_pos = get_new_tail_position(head_pos, tail_pos);
            m.insert(tail_pos, true);
        }
    }

    m.len() as u32
}

fn simulate_part2(steps: &Vec<Step>) -> usize {
    let mut m = HashMap::<(i32, i32), bool>::new();
    let mut head_pos = (0, 0);
    let mut knots = Vec::from([(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0),(0,0)]);

    m.insert(knots[8], true);
    
    for step in steps.iter() {
        let positions = generate_new_positions(head_pos, step);
        for pos in positions.iter() {
            head_pos = *pos;
            for i in 0..9 {
                let target = if i == 0 { head_pos } else { knots[i - 1] };
                knots[i] = get_new_tail_position(target, knots[i]);
            }
            m.insert(knots[8], true);
        }
    }

    m.len()
}

fn main() {
    let steps = get_input();
    let pt1 = simulate(&steps);
    println!("Part1: {}", pt1);
    let pt2 = simulate_part2(&steps);
    println!("Part2: {}", pt2);

}