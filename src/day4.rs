
fn is_contained(inside: (u32, u32), out: (u32, u32)) -> bool {
    let (i1, i2) = inside;
    let (o1, o2) = out;
    i1 >= o1 && i2 <= o2
}

fn overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    let (a1, a2) = a;
    let (b1, b2) = b;
    (a1 >= b1 && a1 <= b2) ||
    (a2 >= b1 && a2 <= b2) ||
    (b1 >= a1 && b1 <= a2) ||
    (b2 >= a1 && b2 <= a2)
}

fn parse_range(range: &str) -> (u32, u32) {
    let split: Vec<&str> = range.split("-").into_iter().collect();
    if let [a, b] = *split.as_slice() {
        (a.parse().unwrap(), b.parse().unwrap())
    } else {
        panic!("invalid line inner");
    }
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let split: Vec<&str> = line.split(",").into_iter().collect();
    //println!("{:?}", split);
    if let [a, b] = *split.as_slice() {
        (parse_range(a), parse_range(b))
    } else {
        panic!("invalid line outer");
    }

}

fn main() {
    let input: Vec<&str> = include_str!("../input/day4.txt").lines().collect();
    let sum: u32 = input.iter().map(|l| parse_line(l)).map(|(i, o)| is_contained(i, o) || is_contained(o, i)).map(|b| if b { 1 } else { 0 }).sum();
    let sum2: u32 = input.iter().map(|l| parse_line(l)).map(|(i, o)| overlap(i, o)).map(|b| if b { 1 } else { 0 }).sum();
    println!("a: {}, b: {}", sum, sum2);
}