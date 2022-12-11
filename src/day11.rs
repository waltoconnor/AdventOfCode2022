use std::collections::VecDeque;

#[derive(Debug)]
enum Operand {
    Old,
    Const(i64)
}

#[derive(Debug)]
enum Op {
    Mul,
    Add
}

#[derive(Debug)]
struct Operation {
    op: Op,
    first: Operand,
    second: Operand
}

impl Operation {
    pub fn from_string(line: &str) -> Self {
        fn sym_to_op(sym: &str) -> Op { match sym { "*" => Op::Mul, "+" => Op::Add, _ => panic!("Unrecognized op") }}
        let back_half = line.split("=").collect::<Vec<&str>>()[1].trim_start().split(" ").collect::<Vec<&str>>();
        let relevant = back_half.as_slice();
        //println!("relevant: {:?}", relevant);
        match relevant {
            ["old", sym, "old"] => Operation { op: sym_to_op(sym), first: Operand::Old, second: Operand::Old },
            ["old", sym, num] => Operation { op: sym_to_op(sym), first: Operand::Old, second: Operand::Const(num.parse().unwrap()) },
            _ => panic!("Could not parse Operation")
        }
    }
}

#[derive(Debug)]
struct Test {
    modulo: i64,
    true_monkey: u32,
    false_monkey: u32
}

impl Test {
    fn from_strings(line3: &str, line4: &str, line5: &str) -> Self {
        let modulo: i64 = line3.split(" ").last().unwrap().parse().unwrap();
        let true_monkey: u32 = line4.split(" ").last().unwrap().parse().unwrap();
        let false_monkey: u32 = line5.split(" ").last().unwrap().parse().unwrap();

        Test {
            modulo,
            true_monkey,
            false_monkey
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    op: Operation,
    test: Test,
    times_inspected: u32
}

impl Monkey {
    pub fn from_lines(lines: &Vec<&str>) -> Self {
        //println!("lines: {:?}", lines);
        let starting_items: Vec<i64> = lines[1].split(":").into_iter().collect::<Vec<&str>>()[1].split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect(); //holy
        let operation = Operation::from_string(lines[2]);
        let test = Test::from_strings(lines[3], lines[4], lines[5]);

        Monkey { items: VecDeque::from(starting_items), op: operation, test, times_inspected: 0 }
    }
}

fn get_input() -> Vec<Monkey> {
    include_str!("../input/day11.txt").split("\n\n").map(|sec| Monkey::from_lines(&sec.lines().into_iter().collect::<Vec<&str>>())).collect()
}

fn apply_op(old: i64, op: &Operation) -> i64 {
    match (&op.op, &op.second) {
        (Op::Add, Operand::Old) => old + old,
        (Op::Add, Operand::Const(val)) => old + val,
        (Op::Mul, Operand::Const(val)) => old * val,
        (Op::Mul, Operand::Old) => old * old,
    }
}

fn test_to_dst_monkey(item: i64, test: &Test) -> usize {
    if item % test.modulo == 0 { test.true_monkey as usize } else { test.false_monkey as usize }
}

fn simulate(mut monkeys: Vec<Monkey>, rounds: u32, divide_worry: bool) -> Vec<Monkey>{
    let modulo_product: i64 = monkeys.iter().map(|m| m.test.modulo).product();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let item = match monkeys[i].items.pop_front() { Some(v) => v, None => { continue; }};
                let mut new_item = apply_op(item as i64, &monkeys[i].op);
                // since our operations only include addition and multiplication, we can use a cylcic group to capture the worry level without it overflowing
                // The most efficent way to do this is to find all the relatively prime test modulos and multiply them, but I don't want to write a procedure for relatively prime, so I'll multiply all of them
                // actually I think you just need the least common multiple of all the modulos, still don't want to implement that
                if divide_worry { new_item /= 3 } else { new_item %= modulo_product } 
                let next_monkey = test_to_dst_monkey(new_item, &monkeys[i].test);
                monkeys[next_monkey].items.push_back(new_item);
                monkeys[i].times_inspected += 1;
            }
        }
    }

    monkeys
}

fn main() {
    let input1 = get_input();
    let input2 = get_input(); //to lazy to derive clone on all the structs
    
    let final_monkeys_pt1 = simulate(input1, 20, true);
    let final_monkeys_pt2 = simulate(input2, 10000, false);

    let mut monkey_vals_pt1: Vec<u32> = final_monkeys_pt1.iter().map(|m| m.times_inspected).collect();
    let mut monkey_vals_pt2: Vec<u32> = final_monkeys_pt2.iter().map(|m| m.times_inspected).collect();

    monkey_vals_pt1.sort();
    monkey_vals_pt1.reverse();
    monkey_vals_pt2.sort();
    monkey_vals_pt2.reverse();

    println!("A: {}", (monkey_vals_pt1[0] as i64) * (monkey_vals_pt1[1] as i64));
    println!("B: {}", (monkey_vals_pt2[0] as i64) * (monkey_vals_pt2[1] as i64));


}