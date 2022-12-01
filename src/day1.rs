use std::collections::BinaryHeap;


pub fn main(){
    let elf_packs: Vec<&str> = include_str!("../input/day1.txt").split("\n\n").collect();
    let elf_totals: Vec<u32> = elf_packs.iter().map(|pack| pack.split("\n").map(|line| line.parse::<u32>().unwrap()).sum()).collect();
    let heap = BinaryHeap::from(elf_totals);
    println!("PART 1: {}", heap.peek().unwrap());
    println!("PART 2: {}", heap.iter().take(3).sum::<u32>());
}
