#[derive(Debug)]
enum Cmd {
    noop,
    addx(i32)
}

fn cmd_to_time(cmd: &Cmd) -> u32 {
    match cmd {
        Cmd::noop => 1,
        Cmd::addx(_) => 2
    }
}

fn parse_cmd(input: &str) -> Cmd {
    match input.split(" ").into_iter().collect::<Vec<&str>>().as_slice() {
        ["noop", ..] => Cmd::noop,
        ["addx", x, ..] => Cmd::addx(x.parse::<i32>().unwrap()),
        _ => panic!("unable to parse cmd")
    }
}

fn parse_input() -> Vec<Cmd> {
    include_str!("../input/day10.txt").lines().map(|l| parse_cmd(l)).collect()
}

fn run_sim(target_cycle: u32, cmds: &Vec<Cmd>) -> (i32, i32) {
    let mut x = 1;
    let mut cur_cycle = 0;

    for cmd in cmds {
        let cost = cmd_to_time(cmd);
        let next_cycle_time = cur_cycle + cost;
        //if (cur_cycle >= target_cycle && next_cycle_time > target_cycle)   {
            if next_cycle_time > target_cycle  {
            //println!("cur cmd: {:?} at cycle {}, with next cycle time {}", cmd, cur_cycle, next_cycle_time);
            break;
        }
        cur_cycle = next_cycle_time;

        match cmd {
            Cmd::addx(n) => { x += *n; },
            Cmd::noop => {}
        }

    }

    (x * ((target_cycle + 1) as i32), x)
}

fn get_strength_sums(times: &Vec<u32>, cmds: &Vec<Cmd>) -> i32 {
    let vals = times.iter().map(|time| run_sim(*time - 1, cmds));
    println!("{:?}", vals.clone().collect::<Vec<(i32, i32)>>());
    
    vals.map(|(a, _)| a).sum()
}

fn is_visible(cur_draw_x: u32, cur_val: i32) -> bool {
    (cur_draw_x as i32 - cur_val).abs() <= 1
}

fn render_row(cmds: &Vec<Cmd>, row_idx: u32) -> String {
    (0..40).into_iter().map(|col_idx| {
        let (_str, val) = run_sim(col_idx + (40 * row_idx), cmds);
        (col_idx, val)
    })
    .map(|(col_idx, val)| is_visible(col_idx, val))
    .map(|visible| match visible { true => "#", false => "."})
    .collect()
}

fn render_crt(cmds: &Vec<Cmd>) -> String {
    (0..6).into_iter().map(|row_idx| render_row(cmds, row_idx)).collect::<Vec<String>>().join("\n")
}

fn main() {
    let input = parse_input();
    let times = Vec::from([20, 60, 100, 140, 180, 220]);
    let total = get_strength_sums(&times, &input);
    println!("Part A: {}", total);
    println!("Part B: \n{}", render_crt(&input));
}