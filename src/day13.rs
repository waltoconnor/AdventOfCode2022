
#[derive(Debug, Clone, PartialEq)]
enum Element {
    List(Vec<Element>),
    Int(i32)
}

fn get_input() -> Vec<(Element, Element)> {
    include_str!("../input/day13.txt")
    .split("\n\n")
    .map(|set| { 
        let v: Vec<Element> = set.lines().map(|l| input_parser(l, 0)).collect(); 
        (v[0].clone(), v[1].clone()) 
    })
    .collect()
}

fn get_closing_bracket(subexpr: &str) -> usize {
    let mut stack = Vec::new();
    for (idx, c) in subexpr.chars().enumerate() {
        match c {
            '[' => stack.push(c),
            ']' => { if stack[stack.len() - 1] == '[' { stack.pop(); } else { println!("ERROR, UNMATCHED ["); }},
            _num => ()
        }

        if stack.len() == 0 {
            return idx;
        }
    }

    return 0;
}

fn get_split_points(subexpr: &str) -> Vec<usize> {
    let mut split_points = Vec::new();
    let mut idx = 0;
    let expr_vec = subexpr.chars().collect::<Vec<char>>();
    while idx < subexpr.len() {
        if expr_vec[idx] == ',' {
            split_points.push(idx);
        }
        if expr_vec[idx] == '[' {
            idx += get_closing_bracket(&subexpr[idx..])
        }
        idx += 1;
    }

    split_points
}

fn parse_interior(subexpr: &str, depth: usize) -> Vec<Element> {
    let split_points = get_split_points(subexpr);
    let mut substrs = Vec::new();
    let mut tail = subexpr;
    let mut head = subexpr;
    for point in split_points.iter().rev() {
        (head, tail) = head.split_at(*point);
        substrs.push(tail.strip_prefix(",").unwrap());
    }
    substrs.push(head);
    substrs.reverse();
    //println!("input: {:?}, output: {:?}", subexpr, substrs);

    substrs.iter().map(|substr| input_parser(substr, depth + 1)).collect()
}

fn input_parser(subexpr: &str, depth: usize) -> Element {
    println!("{} subexpr: {}", depth, subexpr);
    let ret = match subexpr.chars().next().unwrap() {
        '[' => {
            let closing_brace = get_closing_bracket(subexpr);
            if closing_brace == 1 { return Element::List(Vec::new()); }
            let interior = parse_interior(&subexpr[1..closing_brace], depth);
            //println!("Interior: {:?}", interior);
            Element::List(interior)
        },
        _ => Element::Int(subexpr.parse().unwrap())
    };
    println!("{} ret: {:?}", depth, ret);
    ret
}

#[derive(PartialEq, Debug)]
enum CompareResult {
    Good,
    Bad,
    Inconclusive
}

fn compare(e1: &Element, e2: &Element) -> CompareResult {
    match (e1, e2) {
        (Element::Int(l), Element::Int(r)) => if l == r { CompareResult::Inconclusive } else if l < r { CompareResult::Good } else { CompareResult::Bad },
        (Element::List(l), Element::List(r)) => {
            for i in 0..l.len().min(r.len()) {
                let l_el = &l[i];
                let r_el = &r[i];
                let res = compare(l_el, r_el);
                if res == CompareResult::Good { return CompareResult::Good; }
                if res == CompareResult::Bad { return CompareResult::Bad; }
            }
            if l.len() < r.len() { return CompareResult::Good; } else if l.len() == r.len() { return CompareResult::Inconclusive; } else { return CompareResult::Bad; }
        }
        (Element::Int(_l), Element::List(_r)) => compare(&Element::List(vec![e1.clone()]), e2),
        (Element::List(l), Element::Int(r)) => compare(e1, &Element::List(vec![e2.clone()]))
    }
}

fn main() {
    let mut elements = get_input();
    //println!("Elements: {:?}", elements);
    let results = elements.iter().map(|(l, r)| compare(l, r)).collect::<Vec<CompareResult>>();
    println!("results: {:?}", results);
    let sum = results.iter().enumerate().filter(|(idx, res)| **res == CompareResult::Good).map(|(idx, res)| idx as i32 + 1).sum::<i32>();
    println!("Part A: {}", sum);

    let search_2 = Element::List(vec![Element::List(vec![Element::Int(2)])]);
    let search_6 = Element::List(vec![Element::List(vec![Element::Int(6)])]);
    elements.push((search_2.clone(), search_6.clone()));
    let mut el_flat = elements.into_iter().map(|(a, b)| vec![a, b]).flatten().collect::<Vec<Element>>();
    el_flat.sort_by(|a, b| match compare(a, b) { CompareResult::Good => std::cmp::Ordering::Less, CompareResult::Bad => std::cmp::Ordering::Greater, CompareResult::Inconclusive => std::cmp::Ordering::Equal  });
    el_flat.iter().for_each(|l| println!("{:?}", l));
    let idx_2 = el_flat.iter().position(|x| *x == search_2).unwrap() + 1;
    let idx_6 = el_flat.iter().position(|x| *x == search_6).unwrap() + 1;
    println!("B: {}", idx_2 * idx_6);
}