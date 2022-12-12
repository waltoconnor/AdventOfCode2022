use std::collections::{HashSet, VecDeque};

static ASCII_LOWER: &str = "SabcdefghijklmnopqrstuvwxyzE";

fn row_col_to_idx(row: usize, col: usize, row_size: usize) -> usize {
    row_size * row + col
}

fn idx_to_row_col(idx: usize, row_size: usize) -> (usize, usize) {
    (idx / row_size, idx % row_size)
}

#[derive(Debug, Clone)]
struct Node {
    idx: usize,
    outbound: HashSet<usize>,
    height: u32
}

impl Node {
    fn new(c: char, row: usize, col: usize, row_size: usize, col_size: usize) -> Self {
        let height = ASCII_LOWER.find(c).unwrap() as u32;
        let idx = row_col_to_idx(row, col, row_size);

        let mut outbound = HashSet::new();
        if row > 0 { outbound.insert(row_col_to_idx(row - 1, col, row_size)); }
        if row < col_size - 1 { outbound.insert(row_col_to_idx(row + 1, col, row_size)); }
        if col > 0 { outbound.insert(row_col_to_idx(row, col - 1, row_size)); }
        if col < row_size - 1 { outbound.insert(row_col_to_idx(row, col + 1, row_size)); }

        Node { idx, outbound, height }
    }
}



fn get_input() -> Vec<Node> {
    let chars: Vec<Vec<char>> = include_str!("../input/day12.txt").lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let rows = chars.len();
    let cols = chars[0].len();
    
    let mut nodes = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let node = Node::new(chars[i][j], i, j, cols, rows);
            //println!("{:?}", node);
            nodes.push(node);

        }
    }
    nodes
}

fn cleanup_links(mut nodes: Vec<Node>) -> Vec<Node> {
    nodes.iter().map(|node| {
        let bad_values = node.outbound.iter().filter(|val| nodes[**val].height as i32 - node.height as i32 > 1);
        let mut new_node = node.clone();
        for v in bad_values {
            new_node.outbound.remove(v);
        }
        new_node
    }).collect()
}

fn get_path(preds: &Vec<usize>, start_idx: usize, end_idx: usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut cur = end_idx;
    loop {
        if cur == start_idx {
            break;
        }
        path.push(cur);
        cur = preds[cur];
    }
    path.reverse();
    path
}

fn bfs(nodes: &Vec<Node>, start_idx: usize, end_idx: usize) -> Vec<usize> {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited = vec![false; nodes.len()];
    let mut dist = vec![u32::MAX; nodes.len()];
    let mut pred = vec![nodes.len() + 1; nodes.len()];

    visited[start_idx] = true;
    dist[start_idx] = 0;
    queue.push_back(start_idx);

    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        for adj in nodes[cur].outbound.clone() {
            if !visited[adj] {
                visited[adj] = true;
                dist[adj] = dist[cur] + 1;
                pred[adj] = cur;
                queue.push_back(adj);

                if adj == end_idx {
                    return get_path(&pred, start_idx, end_idx);
                }
            }
         }
    }
    println!("BFS FOUND NO SOLUTION");
    return vec![0; nodes.len()];
}

fn main() {
    let mut input = get_input();
    let start_idx = input.iter().map(|node| (node.height, node.idx)).filter(|(h, idx)| *h == 0).next().unwrap().1;
    let end_idx = input.iter().map(|node| (node.height, node.idx)).filter(|(h, idx)| *h == (ASCII_LOWER.len() - 1) as u32).next().unwrap().1;
    let nodes = cleanup_links(input);
    let path = bfs(&nodes, start_idx, end_idx);
    println!("A: len = {}, path = {:?}", path.len(), path);

    let a_squares = nodes.iter().map(|node| (node.height, node.idx)).filter(|(h, idx)| *h == 1 as u32).map(|(_h, idx)| idx);
    let min = a_squares.map(|start| bfs(&nodes, start, end_idx).len()).min().unwrap();
    println!("B: min = {}", min);

}