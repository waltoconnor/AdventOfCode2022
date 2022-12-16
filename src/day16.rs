use std::{collections::{HashMap, VecDeque, HashSet}, hash::Hash};

type Valves = HashSet<usize>;
type Memoize = HashMap::<(usize, i32, usize), u32>;
type MemoizePt2 = HashMap::<(usize, i32, usize, i32, usize), u32>;

struct Room {
    pub name: String,
    pub flow_rate: u32,
    pub connections: Vec<String>
}

impl Room {
    fn new(line: &str) -> Self {
        let (_, tail) = line.split_once("Valve ").unwrap();
        println!("{}", tail);
        let (name, tail) = tail.split_once(" has flow rate=").unwrap();
        println!("{} - {}", name, tail);
        if tail.contains("valves") {
            let (rate, tail) = tail.split_once("; tunnels lead to valves ").unwrap();
            let valves = tail.split(", ").map(|s| String::from(s)).collect();
            Room { name: name.to_string(), flow_rate: rate.parse().unwrap(), connections: valves}
        }
        else {
            let (rate, tail) = tail.split_once("; tunnel leads to valve ").unwrap();
            let valves = Vec::from([tail.to_string()]);
            Room { name: name.to_string(), flow_rate: rate.parse().unwrap(), connections: valves}
        }
    }
}

struct Tunnels {
    rooms: Vec<Room>,
    name_to_room: HashMap<String, usize>,
    distances: Vec<u32>
}

impl Tunnels {
    fn new(rooms: Vec<Room>) -> Self {
        let l = rooms.len();
        let name_to_rooms = rooms
            .iter()
            .enumerate()
            .map(|(idx, room)| (room.name.clone(), idx))
            .collect(); 
        let mut t = Tunnels { rooms: rooms, name_to_room: name_to_rooms, distances: vec![u32::MAX; l * l] };
        t.precompute_distances();
        t
    }

    fn precompute_distances(&mut self) {
        let list_a = self.rooms.iter().map(|r| r.name.clone());
        
        for a in list_a {
            let list_b = self.rooms.iter().map(|r| r.name.clone());
            for b in list_b {
                let dist = self.precompute_distance(&a, &b);
                let a_idx = self.name_to_idx(&a);
                let b_idx = self.name_to_idx(&b);
                self.distances[a_idx * self.rooms.len() + b_idx] = dist;
            }
        }
    }

    fn precompute_distance(&self, start_room: &String, end_room: &String) -> u32{
        let start_idx = self.name_to_idx(&start_room);
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited = vec![false; self.rooms.len()];
        let mut dist = vec![u32::MAX; self.rooms.len()];

        visited[start_idx] = true;
        dist[start_idx] = 0;
        queue.push_back(start_idx);

        while queue.len() > 0 {
            let cur = queue.pop_front().unwrap();
            for adj in self.rooms[cur].connections.iter() {
                let adj_idx = self.name_to_idx(adj);
                if !visited[adj_idx] {
                    visited[adj_idx] = true;
                    dist[adj_idx] = dist[cur] + 1;
                    queue.push_back(adj_idx);

                    if adj.eq(end_room) {
                        return dist[adj_idx];
                    }
                }
            }
        }
        println!("BFS NO SOLUTION");
        100000000
    }

    fn name_to_idx(&self, name: &String) -> usize {
        *self.name_to_room.get(name).unwrap()
    }

    fn get_dist(&self, start_room: usize, end_room: usize) -> u32 {        
        self.distances[start_room * self.rooms.len() + end_room]
    }

    fn get_valve_set(&self) -> Valves {
        self.rooms.iter().filter(|r| r.flow_rate > 0).map(|r| self.name_to_idx(&r.name)).collect()
    }

    fn get_valve_flow_by_idx(&self, idx: usize) -> u32 {
        self.rooms[idx].flow_rate
    }
}

fn parse_input() -> Tunnels {
    let rooms = include_str!("../input/day16.txt").lines().map(|l| Room::new(l)).collect();
    Tunnels::new(rooms)
}

fn hashset_to_hash(hs: &Valves) -> usize {
    //half baked hash algo
    hs.iter().fold(0, |acc, el| (el + (acc.wrapping_shl(16)).wrapping_add(acc.wrapping_shl(6)).wrapping_sub(acc)) % 0xFFFFFFFFFFFF)

    // hs.iter().fold(0, |acc, el| {
    //     let p1 = (acc & 0xFFFFFFFF) << 16;
    //     let p2 = (acc & 0xFFFFFFFF) << 6;
    //     el + p1 + p2 - acc % 0xFFFFFFFFFFFF
    // })
}

fn recurrence(t: &Tunnels, inactive_valves: &Valves, cur_time: i32, last_valve: usize, memoize: &mut Memoize) -> u32 {
    if inactive_valves.len() == 0 { return 0; }
    if cur_time > 25 { println!("Hi"); }
    match memoize.get(&(hashset_to_hash(&inactive_valves), cur_time, last_valve)) { Some(v) => { return *v; }, _ => ()};
    if inactive_valves.len() == 53 { println!("Layer {}", inactive_valves.len()); }
    let mut new_inactive = inactive_valves.clone();
    let max = inactive_valves
        .iter()
        .map(|v| {
            let cost = t.get_dist(last_valve, *v) as i32 + 1;
            if cur_time - cost <= 0 { return 0; }
            let benefit = (cur_time - cost) as u32 * t.get_valve_flow_by_idx(*v);

            //let mut new_inactive = inactive_valves.clone();
            new_inactive.remove(v);
            let result = benefit + recurrence(t, &new_inactive, cur_time - cost, *v, memoize);
            new_inactive.insert(*v);
            result
        })
        .max()
        .unwrap();

    memoize.insert((hashset_to_hash(&inactive_valves), cur_time, last_valve.clone()), max);
    max
}

fn recurrence_pt2(t: &Tunnels, inactive_valves: &Valves, cur_time: i32, cur_el_time: i32, last_valve: usize, last_valve_el: usize, memoize: &mut MemoizePt2) -> u32 {
    if inactive_valves.len() == 0 { return 0; }
    if cur_time > 25 { println!("Hi"); }
    match memoize.get(&(hashset_to_hash(&inactive_valves), cur_time, last_valve, cur_el_time, last_valve_el)) { Some(v) => { return *v; }, _ => ()};
    //if inactive_valves.len() == 53 { println!("Layer {}", inactive_valves.len()); }
    let mut new_inactive = inactive_valves.clone();
    let mut search_space = Vec::new();
    for v1 in inactive_valves.iter() {
        for v2 in inactive_valves.iter() {
            search_space.push((*v1, *v2));
        }
    }

    let max = search_space
        .iter()
        .map(|(v, el_v)| {
            if *v == *el_v { 0 } else {
            
            let cost = t.get_dist(last_valve, *v) as i32 + 1;
            let benefit = if cur_time - cost <= 0 { 0 } else { (cur_time - cost) as u32 * t.get_valve_flow_by_idx(*v) };

            let cost_el = t.get_dist(last_valve_el, *el_v) as i32 + 1;
            let benefit_el = if cur_el_time - cost_el <= 0 { 0 } else { (cur_el_time - cost_el) as u32 * t.get_valve_flow_by_idx(*el_v) };
            if benefit == 0 && benefit_el == 0 { return 0; }

            //let mut new_inactive = inactive_valves.clone();
            new_inactive.remove(v);
            new_inactive.remove(el_v);
            let result = benefit + benefit_el + recurrence_pt2(t, &new_inactive, cur_time - cost, cur_el_time - cost_el, *v, *el_v, memoize);
            new_inactive.insert(*v);
            new_inactive.insert(*el_v);
            //println!("Result: {}", result);
            result
        }})
        .max()
        .unwrap();
    
    //println!("max value: {}", max);
    memoize.insert((hashset_to_hash(&inactive_valves), cur_time, last_valve, cur_el_time, last_valve_el), max);
    max
}

fn main() {
    let tunnels = parse_input();
    let mut valves = tunnels.get_valve_set();
    let mut memoize_table = Memoize::new();
    let mut memoize_table_pt2 = MemoizePt2::new();
    println!("A: {}", recurrence(&tunnels, &mut valves, 30, tunnels.name_to_idx(&String::from("AA")), &mut memoize_table));
    println!("B: {}", recurrence_pt2(&tunnels, &mut valves, 26, 26, tunnels.name_to_idx(&String::from("AA")), tunnels.name_to_idx(&String::from("AA")), &mut memoize_table_pt2));
}