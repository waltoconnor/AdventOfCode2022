type Grid = Vec<Vec<u32>>;

fn prep_input() -> Grid {
    let input = include_str!("../input/day8.txt");
    input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

fn check_tree(grid: &Grid, x: usize, y: usize) -> bool {
    let x_size = grid[0].len();
    let y_size = grid.len();
    let height = grid[x][y];

    if x == 0 || y == 0 || x == (x_size - 1) || y == (y_size - 1) {
        return true;
    }

    let mut right = true;
    for i in (x + 1)..x_size {
        if grid[i][y] >= height { right = false; }
    }

    let mut left = true;
    for i in 0..x {
        if grid[i][y] >= height { left = false; }
    }

    let mut bottom = true;
    for i in (y + 1)..y_size {
        if grid[x][i] >= height { bottom = false; }
    }

    let mut top = true;
    for i in 0..y {
        if grid[x][i] >= height { top = false; }
    }

    left || right || top || bottom 
}

fn score_tree(grid: &Grid, x: usize, y: usize) -> i32 {
    let x_size = grid[0].len();
    let y_size = grid.len();
    let height = grid[x][y];

    let mut right = 0;
    for i in (x + 1)..x_size {
        //if grid[i][y] >= cur_max { right += 1; cur_max = grid[i][y]; }
        right += 1;
        if grid[i][y] >= height { break; }
    }

    let mut left = 0;
    if x != 0 {
        for i in (0..x).into_iter().rev() {
            //if grid[i][y] >= cur_max { left += 1; cur_max = grid[i][y]; }
            left += 1;
            if grid[i][y] >= height { break; }
        }
    }
    
    let mut bottom = 0;
    for i in (y + 1)..y_size {
        //if grid[x][i] >= cur_max { bottom += 1; cur_max = grid[x][i]; }
        bottom += 1;
        if grid[x][i] >= height { break; }
    }

    let mut top = 0;
    if y != 0 {
        for i in (0..y).into_iter().rev() {
            println!("{},{}", x, i);
            //if grid[x][i] >= cur_max { top += 1; cur_max = grid[x][i]; }
            top += 1;
            if grid[x][i] >= height { break; }
        }
    }

    left * right * top * bottom 
}

fn main() {
    let grid = prep_input();
    let mut sum = 0;
    let mut score_vec = Vec::new();
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if check_tree(&grid, x, y) {
                //println!("{},{} -> VISIBLE", x, y);
                sum += 1;
            }
            
        }
    }

    for x in 1..(grid[0].len() - 1) {
        for y in 1..(grid.len() - 1) {
            score_vec.push(score_tree(&grid, x, y));
        }
    }

    println!("A: {}\nB: {}", sum, score_vec.iter().max().unwrap());
}
