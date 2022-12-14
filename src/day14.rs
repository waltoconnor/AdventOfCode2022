#[derive(Debug, Clone, Copy, PartialEq)]
enum GridCell {
    Rock,
    Air,
    Sand
}

fn line_to_walls(line: &str) -> Vec<(u32, u32)> {
    let blocks = line.split(" -> ");
    blocks.map(|block| block.split_once(",").unwrap()).map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap())).collect()
}

fn get_input() -> Vec<Vec<(u32, u32)>> {
    include_str!("../input/day14.txt").lines().map(|line| line_to_walls(line)).collect()
}

fn get_offsets(input: &Vec<Vec<(u32, u32)>>) -> (u32, u32) {
    let x = input.iter().map(|line| line.iter().map(|(x, _y)| x).min().unwrap()).min().unwrap();
    let y = input.iter().map(|line| line.iter().map(|(_x, y)| y).min().unwrap()).min().unwrap();
    (*x, *y)
}

fn get_maxes(input: &Vec<Vec<(u32, u32)>>) -> (u32, u32) {
    let x = input.iter().map(|line| line.iter().map(|(x, _y)| x).max().unwrap()).max().unwrap();
    let y = input.iter().map(|line| line.iter().map(|(_x, y)| y).max().unwrap()).max().unwrap();
    (*x, *y)
}

fn transform_input(input: &Vec<Vec<(u32, u32)>>, min_x: u32, min_y: u32) -> Vec<Vec<(u32, u32)>> {
    input.iter().map(|line| line.iter().map(|(x, y)| (x - min_x, y - min_y)).collect()).collect()
}

fn make_grid(width: u32, height: u32) -> Vec<Vec<GridCell>> {
    (0..width).into_iter().map(|_| vec![GridCell::Air; height as usize]).collect()
}

fn setup_grid(input: &Vec<Vec<(u32, u32)>>) -> (Vec<Vec<GridCell>>, usize, u32, u32) {
    
    let (min_x, min_y) = get_offsets(&input);
    let (max_x, max_y) = get_maxes(&input);

    let left_clearance = 2;
    let right_clearance = 2;

    let lines = transform_input(input, min_x - left_clearance, 0); // moves to origin
    let width = (max_x - min_x) + right_clearance + left_clearance + 2;
    let height = max_y + 2; // bonus row on the bottom
    println!("width = {}, height = {}", width, height);

    let mut grid = make_grid(width, height);
    lines.iter().for_each(|line| apply_line(line, &mut grid));
    let sand_start_x = 500 - min_x as usize + 2;

    (grid, sand_start_x, width, height)
}

fn setup_grid_part_2(input_orig: &Vec<Vec<(u32, u32)>>) -> (Vec<Vec<GridCell>>, usize, u32, u32) {
    let mut input = input_orig.clone();

    let (min_x_orig, _min_y_orig) = get_offsets(&input_orig);
    let (max_x_orig, max_y_orig) = get_maxes(&input_orig);

    let floor_y = max_y_orig + 2;

    let floor_start_x = min_x_orig - 100; 
    let floor_end_x = max_x_orig + 1000; //very lazy, selected by looking at which side of plot sand was piling up on
    input.push(vec![(floor_start_x, floor_y), (floor_end_x, floor_y)]);

    let (min_x, min_y) = get_offsets(&input);
    let (max_x, max_y) = get_maxes(&input);

    let left_clearance = 2;
    let right_clearance = 2;

    let lines = transform_input(&input, min_x - left_clearance, 0); // moves to origin
    let width = (max_x - min_x) + right_clearance + left_clearance + 2;
    let height = floor_y + 1; // bonus row on the bottom
    println!("width = {}, height = {}", width, height);

    let mut grid = make_grid(width, height);
    lines.iter().for_each(|line| apply_line(line, &mut grid));
    let sand_start_x = 500 - min_x as usize + 4;

    (grid, sand_start_x, width, height)
}

fn range(start: u32, end: u32) -> Vec<u32> {
    if start < end { (start..=end).collect() } else { (end..=start).rev().collect() }
}

fn apply_segment(start: (u32, u32), end: (u32, u32), grid: &mut Vec<Vec<GridCell>>) {
    let (start_x, start_y) = start;
    let (end_x, end_y) = end;
    //println!("{:?} -> {:?}", start, end);
    if start_x == end_x {
        for y in range(start_y, end_y) {
            grid[start_x as usize][y as usize] = GridCell::Rock;
        }
    }
    else if start_y == end_y {
        for x in range(start_x, end_x) {
            grid[x as usize][start_y as usize] = GridCell::Rock;
        }
    }
}

fn apply_line(line: &Vec<(u32, u32)>, grid: &mut Vec<Vec<GridCell>>) {
    line.windows(2).for_each(|v| if v.len() == 2 { apply_segment(v[0], v[1], grid); });
}

fn print_grid(grid: &Vec<Vec<GridCell>>, width: u32, height: u32){
    for y in 0..height {
        let mut s = String::new();
        for x in 0..width {
            match grid[x as usize][y as usize] {
                GridCell::Rock => s += "#",
                GridCell::Air => s += ".",
                GridCell::Sand => s += "o",
            }
        }
        println!("{}", s);
    }
}

fn occupied(c: &GridCell) -> bool {
    match c {
        &GridCell::Air => false,
        _ => true
    }
}

//returns if it hit the bottom row
fn add_sand(grid: &mut Vec<Vec<GridCell>>, sand_pos_x: usize, height: u32) -> bool{
    let mut cur_pos = (sand_pos_x, 0usize);
    loop {
        let (x, y) = cur_pos;
        if y == height as usize - 1 { return true; }

        let down = grid[x][y + 1];
        let down_left = grid[x - 1][y +  1];
        let down_right = grid[x + 1][y + 1];

        match (occupied(&down_left), occupied(&down), occupied(&down_right)) {
            (_, false, _) => { cur_pos = (x, y+1); },
            (true, true, true) => { grid[x][y] = GridCell::Sand; return false; },
            (false, true, _) => { cur_pos = (x - 1, y + 1); },
            (true, true, false) => { cur_pos = (x + 1, y + 1); },
        }
    }
}

fn main() {
    let input = get_input();
    let (mut grid, sand_start_x, width, height) = setup_grid(&input);

    let mut added = 0;
    while !add_sand(&mut grid, sand_start_x, height) { added += 1; }
    print_grid(&grid, width, height);
    println!("A: {}", added);

    let (mut grid2, sand_start_x2, width2, height2) = setup_grid_part_2(&input);
    print_grid(&grid2, width2, height2); 

    let mut added2 = 0;
    while grid2[sand_start_x2][0] != GridCell::Sand {
        add_sand(&mut grid2, sand_start_x2, height2);
        added2 += 1;
        if added2 % 1000 == 0 {
            print_grid(&grid2, width2, height2); 
        }
    }
    print_grid(&grid2, width2, height2); 
    println!("B: {}", added2);

}