use std::collections::{HashMap,HashSet};
use std::mem::size_of;

pub fn run_1(file:&str) -> usize {
    let data = read_data(file);
    let res = pt1_threaded(data.0,data.1);
    res.0
}

pub fn run_2(file:&str) -> usize {
    let data = read_data(file);
    let res = pt2_threaded(data.0,data.1,read_chars(file));

    res
}

fn read_chars(file:&str) -> Vec<Vec<char>> {
    std::fs::read_to_string(file).expect("Failed to read file").lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect()
}

fn read_data(file :&str) -> (Coord,HashMap<Coord,Connections>) {
    let mut ret = HashMap::new();
    let mut st: Coord = Coord{row:0,col:0};
    for (row,line) in std::fs::read_to_string(file).expect("Failed to read file").lines().enumerate() {
        for (col, ch) in  line.chars().enumerate() {
            if ch == 'S'  {
                st = Coord {row,col};
            }
            let my_coord = Coord{row: row,col: col};
            let node = char_to_connec(&my_coord, ch);
            if let Some(node) = node{
                ret.insert(my_coord, node);
            }
        }
    }
    (st,ret)
}

fn pt2_threaded(st: Coord, map: HashMap<Coord,Connections>,mut orig: Vec<Vec<char>>) -> usize {
    let new_st = st.clone();
    let new_map = map.clone();
    let handle = std::thread::Builder::new()
    .stack_size(size_of::<f64>()*1000000)
    .spawn(move ||{
        pt1(&st,&map)
    });
    let res = handle.expect("Should have a handle").join().unwrap();
    let max_coord = &res.1.iter().max().expect("Should have a max coord somewhere...").clone();
    let set = res.1;
    let mut count = 0;
    replace_start(&mut orig,&new_st,&new_map,&set);
    for row in 0..=max_coord.row {
        let mut should_count = false;
        let mut flip_next = 'x';
        for col in 0..=max_coord.col {
            let c = orig[row][col];
            if !set.contains(&Coord{row,col}) && should_count {
                count += 1;
            } else if set.contains(&Coord{row,col}) {
                if c == '|' {
                    should_count = !should_count;
                } else if c == 'F' {
                    should_count = !should_count;
                    flip_next = '7';
                } else if c == 'L' {
                    should_count = !should_count;
                    flip_next = 'J'
                } else if (c == '7' && flip_next == c) || (c == 'J' && flip_next == c) {
                    should_count = !should_count;
                    flip_next = 'x';
                }
                    
            }
        }
    }
    count
}

fn replace_start(orig: &mut Vec<Vec<char>>, st: &Coord, map: &HashMap<Coord,Connections>, inloop: &HashSet<Coord>) {
    let joins = find_around_start(st,map);
    let mut abv = false;
    let mut blw = false;
    let mut rght = false;
    let mut left = false;
    for j in joins {
        if inloop.contains(&j) {
            if j.row < st.row {
                abv = true;
            }
            if j. row > st.row {
                blw = true;
            }
            if j.col < st.col {
                left = true;
            }
            if j.col > st.col {
                rght = true;
            }
        }
    }

    if abv && blw {
        orig[st.row][st.col] = '|';
    } else  if abv && rght {
        orig[st.row][st.col] = 'L';
    } else if abv && left {
        orig[st.row][st.col] = 'J';
    } else if blw && left {
        orig[st.row][st.col] = '7';
    } else if blw && rght {
        orig[st.row][st.col] = 'F';
    } else if rght && left {
        orig[st.row][st.col] = '-';
    }

}

fn pt1_threaded(st: Coord, map: HashMap<Coord,Connections>) -> (usize,HashSet<Coord>) {
    let handle = std::thread::Builder::new()
    .stack_size(size_of::<f64>()*1000000)
    .spawn(move ||{
        pt1(&st,&map)
    });
    handle.expect("Should have a handle").join().unwrap()
}

fn pt1(st: &Coord, map: &HashMap<Coord,Connections>) -> (usize,HashSet<Coord>) {
    let mut isloop = HashSet::new();
    isloop.insert(st.clone());
    let start_steps = find_around_start(st,map);
    let mut steps = Vec::new();
    for step in start_steps {
        steps.push(
            follow_path(&step,st,st,map,1,&mut isloop)
        );
    }

    let max = steps.iter().max().unwrap();
    (max/2 + max%2, isloop)

}

fn follow_path(current_step: &Coord, from: &Coord, target: &Coord, map: &HashMap<Coord,Connections>, steps_so_far: usize,isloop: &mut HashSet<Coord>) -> usize {
    if current_step == target{
        return steps_so_far;
    }
    let mut res= Vec::new();
    let routes = map.get(&current_step).expect("stepped onto invalid tile");
    for route in &routes.nodes{
        if route != from {
            isloop.insert(route.clone());
            res.push(follow_path(
                &route, current_step, target, map, steps_so_far+1,isloop
            ))
        }
    }
    *res.iter().max().expect("should have had a longest path")

}

fn char_to_connec(own_coord: &Coord, ch: char) -> Option<Connections> {
    let mut res = Vec::new();
    let higher = Coord{row: own_coord.row.wrapping_sub(1), col: own_coord.col};
    let lower = Coord{row: own_coord.row+1, col: own_coord.col};
    let righter = Coord{row: own_coord.row, col: own_coord.col+1};
    let lefter = Coord{row: own_coord.row, col: own_coord.col.wrapping_sub(1)};
    match ch {
        '|' => {
            if let Some(_) = own_coord.row.checked_sub(1) {
                res.push(higher);
            }
            res.push(lower);
        },
        '-' => {
            if let Some(_) =  own_coord.col.checked_sub(1) {
                res.push(lefter);
            }
            res.push(righter);
        },
        'L' => {
            if let Some(_) = own_coord.row.checked_sub(1) {
                res.push(higher);
            }
            res.push(righter);
        },
        'J' => {
            if let Some(_) = own_coord.row.checked_sub(1) {
                res.push(higher);
            }
            if let Some(_) =  own_coord.col.checked_sub(1) {
                res.push(lefter);
            }
        },
        '7' => {
            if let Some(_) =  own_coord.col.checked_sub(1) {
                res.push(lefter);
            }
            res.push(lower);
        },
        'F' => {
            res.push(righter);
            res.push(lower);
        },
        _ => {},
    }
    if res.len()>0 {
        return Some(Connections{nodes:res})
    }
    None
}

fn find_around_start(st: &Coord, map: &HashMap<Coord,Connections>) -> Vec<Coord> {
    let mut res: Vec<Coord> = Vec::new();
    let mut attempts: Vec<Coord> = Vec::new();
    if let Some(lower) = st.row.checked_sub(1) {
        attempts.push(Coord{row:lower,col:st.col});
    }
    if let Some(lefter) = st.col.checked_sub(1) {
        attempts.push(Coord{row:st.row,col:lefter});
    }
    attempts.push(Coord{row:st.row+1,col:st.col});
    attempts.push(Coord{row:st.row,col:st.col+1});

    for coord in attempts {
        if let Some(node) = map.get(&coord){
            if connects(&st,node) {
                res.push(coord);
            }
        }
    }
    res
}

fn connects(from: &Coord, node: &Connections) -> bool {
    node.nodes.iter().any(|n| n==from)
}

#[derive(Debug,Hash,PartialEq,Eq,PartialOrd,Clone,Ord)]
struct Coord {
    row: usize,
    col: usize
}

#[derive(Debug,Clone)]
struct Connections {
    nodes: Vec<Coord>,
}