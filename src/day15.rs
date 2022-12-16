use std::collections::HashSet;

type Coord = (i64, i64);
type BeaconsSensors = Vec<(Coord, Coord)>;

//BEACON then SENSOR
fn parse_line(line: &str) -> (Coord, Coord) {
    let (_, tail) = line.split_once("=").unwrap(); //tail now starts with x number
    let (sx, tail) = tail.split_once(",").unwrap(); //head is x number, tail starts with garbage
    let (_, tail) = tail.split_once("=").unwrap(); //head is garbage, tail now starts with y number
    let (sy, tail) = tail.split_once(":").unwrap(); //head is y number
    
    let (_, tail) = tail.split_once("=").unwrap();
    let (bx, tail) = tail.split_once(",").unwrap();
    let (_, by) = tail.split_once("=").unwrap();

    println!("bx: {}, by: {}, sx: {}, sy: {}", bx, by, sx, sy);
    let beacon = (bx.parse().unwrap(), by.parse().unwrap());
    let sensor = (sx.parse().unwrap(), sy.parse().unwrap());

    (beacon, sensor)
}

fn read_input() -> Vec<(Coord, Coord)> {
    include_str!("../input/day15.txt").lines().map(|line| parse_line(line)).collect()
}


fn manhatten_dist(p1: Coord, p2: Coord) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn in_range_of_beacon(p: Coord, b: Coord, s: Coord) -> bool {
    let range = manhatten_dist(b, s);
    manhatten_dist(p, s) <= range 
}


fn get_empty_from_sensor(s: Coord, b: Coord, y_idx: i64) -> HashSet<i64> {
    let range = manhatten_dist(s, b);
    let (sx, sy) = s;
    let start = sx - range - 1;
    let end = sx + range + 1;

    let mut set = HashSet::new();
    for i in start..=end {
        let coord = (i, y_idx);
        if manhatten_dist(coord, s) <= range && coord != s && coord != b  {
            set.insert(i);
        }
    }

    set
}

fn get_all_sensors(bs: &BeaconsSensors, y_idx: i64) -> HashSet<i64> {
    let empty = bs
    .iter()
    .map(|(b, s)| get_empty_from_sensor(*s, *b, y_idx));
    // .fold(HashSet::<i64>::new(), |acc, el| {  let mut n = HashSet::new(); n.extend(acc); n.extend(el); n })
    let mut s = HashSet::new();
    for e in empty {
        s.extend(e);
    }
    s
}

fn get_edge_from_sensor(s: Coord, b: Coord) -> Vec<Coord> {
    let range = manhatten_dist(s, b);
    let (sx, sy) = s;
    let mut edge_cells = Vec::new();

    //edge 1  --> /\
    //            \/
    {
        let x_start = sx - range - 1;
        let y_end = sy + range + 1;
        (x_start..=sx).zip(sy..=y_end).for_each(|p|{ edge_cells.push(p); });
    }

    //edge 2      /\ <--
    //            \/
    {
        let x_end = sx + range + 1;
        let y_start = sy + range + 1;
        (sx..=x_end).zip((sy..=y_start).rev()).for_each(|p|{ edge_cells.push(p); });
    }

    //edge 3      /\
    //            \/ <--
    {
        let x_end = sx + range + 1;
        let y_start = sy - range - 1;
        (sx..=x_end).zip(y_start..=sy).for_each(|p|{ edge_cells.push(p); });
    }

    //edge 4      /\ <--
    //            \/
    {
        let x_start = sx - range - 1;
        let y_end = sy - range - 1;
        (x_start..=sx).zip((y_end..=sy).rev()).for_each(|p|{ edge_cells.push(p); });
    }

    println!("Sensor done");

    edge_cells
}

fn far_from_all_sensors(bs: &BeaconsSensors, p: Coord) -> bool{
    for (b, s) in bs {
        if in_range_of_beacon(p, *b, *s) { return false; }
    }
    true
}

fn check_all_sensors_for_empty(bs: &BeaconsSensors) -> Coord {
    let mut to_check = HashSet::new();
    for (b, s) in bs.iter() {
        to_check.extend(get_edge_from_sensor(*s, *b));
    }
    println!("Checking {} cells", to_check.len());

    let limit = 4000000;
    for (x, y) in to_check.iter() {
        if *x > limit || *y > limit || *x <= 0 || *y <= 0 { continue; }
        if far_from_all_sensors(bs, (*x, *y)) { return (*x, *y); }
    }

    println!("NO SOLUTION FOUND");
    return (-1, -1);
}

fn main() {
    let input = read_input();
    let result = get_all_sensors(&input, 2000000).len();
    println!("A: {}", result);

    let (x, y) = check_all_sensors_for_empty(&input);
    println!("B: {} ({},{})", 4000000 * x + y, x, y);
}