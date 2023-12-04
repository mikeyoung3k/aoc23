use std::cmp::min;
use std::collections::{HashMap,HashSet};

pub fn run (filename: &str) -> (usize,usize) {
    let data = read_data(filename);
    let p1 = p1(&data);
    let p2 = p2(&data);
    (p1,p2)
}

fn read_data(file: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(file)
    .expect("File not found")
    .lines()
    .map(|x| {
        x.chars().collect::<Vec<char>>()
    })
    .collect::<Vec<Vec<char>>>()
}

#[derive(Debug,Clone)]
struct Part{
    pn: usize,
    start: (usize,usize),
    fin: (usize,usize),
}

fn parse_parts(data:&[Vec<char>]) -> Vec<Part> {
    let mut res = Vec::new();
    let mut pn = String::new();
    let mut st = (0,0);
    let mut fin;
    let mut midpart = false;
    for (rownum,row) in data.iter().enumerate(){
        for (colnum,item) in row.iter().enumerate(){
            if item.is_ascii_digit() {
                pn.push(*item);
                if !midpart {
                    st = (rownum,colnum);
                }
                midpart = true;
            } else {
                if midpart {
                    fin = (rownum,colnum.saturating_sub(1));
                    midpart = false;
                    res.push(Part{
                        pn: pn.parse::<usize>().expect("Invalid part number"),
                        start: st,
                        fin,
                    });
                }
                pn.drain(..);
            }
        }
        if midpart {
            fin = (rownum, row.len()-1);
            midpart = false;
            res.push(Part{
                pn: pn.parse::<usize>().expect("Invalid part number at end of row"),
                start: st,
                fin,
            })
        }
        pn.drain(..);
    }

    res
}

fn is_valid_part(pn: &Part, data: &Vec<Vec<char>>) -> bool {
    let left_corner = (pn.start.0.saturating_sub(1),pn.start.1.saturating_sub(1));
    let right_corner = (min(pn.fin.0+1,data.len()-1),min(pn.fin.1+1,data.len()-1));

    let mut rownum = left_corner.0;
    let mut colnum = left_corner.1;
    while rownum <= right_corner.0 {
        if data[rownum][colnum] != '.' && !data[rownum][colnum].is_ascii_digit(){
            return true
        }
        colnum += 1;
        if colnum > right_corner.1{
            colnum = left_corner.1;
            rownum += 1;
        }
    }
    false
}

fn p1(data: &Vec<Vec<char>>) -> usize {
    let parts = parse_parts(data);
    parts.iter().filter(|p| is_valid_part(p, data)).map(|p| p.pn).sum()
}

fn p2(data: &[Vec<char>]) -> usize {
    let gears = find_gears(data);
    let mut ratios = Vec::new();
    for gear in gears {
        if let Some(result) = find_gear_ratio(gear,data){
            ratios.push(result)
        }
    }
    return ratios.iter().sum()
}


fn find_gears(data: &[Vec<char>]) -> Vec<(usize,usize)> {
    let mut res = Vec::new();
    for (rownum, row) in data.iter().enumerate() {
        for (colnum, val) in  row.iter().enumerate(){
            if *val == '*' {
                res.push((rownum, colnum));
            }
        }
    }

    res
}

fn find_gear_ratio(gear: (usize, usize), data: &[Vec<char>]) -> Option<usize> {
    let left_corner = (gear.0.saturating_sub(1),gear.1.saturating_sub(1));
    let right_corner = (min(gear.0+1,data.len()-1),min(gear.1+1,data.len()-1));

    let map = map_parts(data);

    let mut parts = HashSet::new();
    for row in left_corner.0 ..= right_corner.0 {
        for col in left_corner.1 ..= right_corner.1 {
            if let Some(pt) = map.get(&(row,col)) {
                parts.insert(pt.pn);
            }
        }
    }

    if parts.len() == 2 {
        return Some(parts.iter().product());
    }

    None

}

fn map_parts(data:&[Vec<char>]) -> HashMap<(usize,usize),Part> {
    let mut res = HashMap::new();
    let mut pn = String::new();
    let mut st = (0,0);
    let mut fin;
    let mut midpart = false;
    for (rownum,row) in data.iter().enumerate(){
        for (colnum,item) in row.iter().enumerate(){
            if item.is_ascii_digit() {
                pn.push(*item);
                if !midpart {
                    st = (rownum,colnum);
                }
                midpart = true;
            } else {
                if midpart {
                    fin = (rownum,colnum.saturating_sub(1));
                    midpart = false;
                    let p = Part{
                        pn: pn.parse::<usize>().expect("Invalid part number"),
                        start: st,
                        fin,
                    };

                    for i in st.1 ..= fin.1{
                        res.insert((st.0,i),p.clone());
                    }
                }
                pn.drain(..);
            }
        }
        if midpart {
            fin = (rownum, row.len()-1);
            midpart = false;
            let p = Part{
                pn: pn.parse::<usize>().expect("Invalid part number at end of row"),
                start: st,
                fin,
            };

            for i in st.1 ..= fin.1 {
                res.insert((st.0,i),p.clone());
            }
        }
        pn.drain(..);
    }

    res
}