
pub fn run(file: &str) -> (i32, i32) {
    let data = read_data(file);

    (pt1(data.clone()),pt2(data))
}

fn pt1(mut data: Vec<Sequence>) -> i32 {
    data.iter_mut().map(|line| {
        line.find_pattern();
        line.get_next()
    })
    .sum()
}
fn pt2(mut data: Vec<Sequence>) -> i32 {
    data.iter_mut().map(|line| {
        line.find_pattern();
        line.get_previous()
    })
    .sum()
}

fn read_data(file :&str) -> Vec<Sequence> {
    std::fs::read_to_string(file).expect("Failed to read file")
    .lines()
    .map( |l| {
        Sequence{
            base: l.split_whitespace()
            .map(|n| n.parse::<i32>().expect("Failed to parse base sequence"))
            .collect(),
            pattern: None,
            init_pattern: None,
        }
    })
    .collect::<Vec<Sequence>>()
}

#[derive(Debug,Clone)]
struct Sequence {
    base: Vec<i32>,
    pattern:Option<Vec<i32>>,
    init_pattern:Option<Vec<i32>>,
}

impl Sequence {
    fn find_pattern(&mut self) {
        let mut pattern_rights = Vec::new();
        let mut pattern_lefts = Vec::new();
        let mut go_more = true;
        let mut base = self.base.clone();
        pattern_rights.push(base[base.len()-1]);
        pattern_lefts.push(base[0]);
        while go_more {
            base = base.iter().map_windows(|[&x,&y]| {
                y-x
            } )
            .collect::<Vec<i32>>();
            go_more = !base.iter().all(|&x|x==0);
            pattern_rights.push(base[base.len()-1]);
            pattern_lefts.push(base[0]);
            
        }
        self.pattern = Some(pattern_rights);
        self.init_pattern = Some(pattern_lefts);
    }

    fn get_next(&self) -> i32 {
        if let Some(pat) = &self.pattern {
            return pat.iter().sum()
        }
        panic!("No pattern found");
    }

    fn get_previous(&self) -> i32 {
        let mut previous = 0;
        if let Some(pat) = &self.init_pattern {
            for x in pat.iter().rev() {
                previous = x-previous;
            }
            return previous
        }
        panic!("No init pattern found!")
    }
}