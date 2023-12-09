use std::collections::HashMap;

pub fn run_p1 (file :&str) -> usize {
    let (dirs,nodes) = read_data(file);
    let dir_iter = DirectionsIter{
        ways: &dirs,
        pointer: 0,
    };
    pt1(dir_iter, &nodes)
}

pub fn run_p2(file: &str) -> usize {
    let (dirs,nodes) = read_data(file);
    let dir_iter = DirectionsIter{
        ways: &dirs,
        pointer: 0,
    };
    pt2(dir_iter, &nodes)
}

fn pt1(directions: DirectionsIter, map: &HashMap<String, (String,String)>) -> usize {
    let mut steps = 0;
    let mut current = "AAA";
    for direction in directions{
        steps += 1;
        let next_steps = map.get(current).expect("No path forwards!");
        current = match direction {
            Way::L => &next_steps.0,
            Way::R => &next_steps.1,
        };
        if current == "ZZZ" {
            break;
        }
    }
    steps
}

fn pt2(directions: DirectionsIter, map: &HashMap<String, (String,String)> ) -> usize {
    let starts = get_starts(map);
    let mut cycle_counts = Vec::new();
    for start in starts{
        cycle_counts.push(get_cycle_num(start,directions.clone(),map))
    }
    println!("Cycle count: {:?}", cycle_counts);
    alt_lcm(&cycle_counts)
}

fn get_cycle_num<'a>(mut current: &'a str, directions: DirectionsIter, map: &'a HashMap<String,(String,String)>) -> usize {
    let mut steps = 0;
    for direction in directions {
        steps += 1;
        let next_steps = map.get(current).expect("No path forwards!");
        current = match direction {
            Way::L => &next_steps.0,
            Way::R => &next_steps.1,
        };
        if current.ends_with('Z') {
            break
        }
    }
    steps
}


fn get_starts(map: &HashMap<String,(String,String)>) -> Vec<&String> {
    let mut res = Vec::new();
    for key in map.keys() {
        if key.ends_with('A') {
            res.push(key)
        }
    }
    res
}

fn alt_lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = alt_lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn lcm(counts: Vec<usize>) -> usize {
    let mut multiples = counts.clone();
    loop {
        let low_i = find_lowest(&multiples);
        multiples[low_i] += counts[low_i];
        if all_same(&multiples) {
            return multiples[0]
        }
    }
}

fn find_lowest(multiples: &Vec<usize>) -> usize {
    let lowest = multiples.iter().min().expect("No min found");
    for (i,x) in multiples.iter().enumerate() {
        if x == lowest{
            return i
        }
    }
    panic!("No lowest found");
}

fn all_same(counts: &Vec<usize>) -> bool {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for item in counts{
        map.insert(*item,0);
    }
    if map.len() == 1{
        return true
    }
    false
}

#[derive(Debug,Clone)]
struct Directions {
    data: Vec<Way>,
}

#[derive(Debug,Clone)]
struct DirectionsIter<'a> {
    ways: &'a Directions,
    pointer: usize,
}

#[derive(Debug,Clone)]
enum Way {
    L,
    R,
}

impl <'a>  Iterator for DirectionsIter <'a> {
    type Item = &'a Way;
    fn next(&mut self) -> Option<&'a Way> {
        let ret = Some(&self.ways.data[self.pointer]);
        self.pointer += 1;
        if self.pointer >= self.ways.data.len() {
            self.pointer = 0;
        }
        ret
    }
}

fn read_data(file :&str) -> (Directions, HashMap<String,(String,String)>) {
    let str_file = std::fs::read_to_string(file).expect("Failed to read file");
    let mut file_lines = str_file.lines();
    let dir = file_lines.next().expect("Failed to get directions")
    .chars()
    .map(|c| {
        match c {
            'R' => Way::R,
            'L' => Way::L,
            _  => panic!("Unexpected direction variant"),
        }
    })
    .collect::<Vec<Way>>();

    let _ = file_lines.next();

    let mut map = HashMap::new();
    for line in file_lines{
        let mut elems = line.split_whitespace();
        let entry = elems.next().expect("Failed to get entry").to_owned();
        let left = elems.nth(1).expect("Failed to get left").trim_matches('(').trim_matches(',').to_owned();
        let right = elems.next().expect("Failed to get right").trim_matches(')').to_owned();
        map.insert(entry,(left,right));
    }

    (Directions{data:dir},map)

}