use lazy_static::lazy_static;

use regex::Regex;

fn read_data(file_name: &str) -> Vec<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d)(.*?(\d))?+").expect("Failed to compile regex");
    }

    std::fs::read_to_string(file_name)
    .expect("File not found")
    .lines()
    .map(|x| {
    if let Some(n) = RE.captures(x) {
        let mut r = n[1].to_owned();
        let add = if let Some(d) = n.get(n.len()-1) {
            d.as_str().to_owned()
        } else {
            r.clone()
        };
        r.push_str(&add);
            usize::from_str_radix(&r,10).expect("Parse failed")
    } else {
        0
    }
})
    .collect::<Vec<usize>>()
}

fn read_data_pt2(file_name: &str) -> Vec<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)(.*?(\d|one|two|three|four|five|six|seven|eight|nine))?+").expect("Failed to compile regex");
    }

    std::fs::read_to_string(file_name)
    .expect("File not found")
    .lines()
    .map(|x| {
        let mut changed = x.replace("three","t3e");
        changed = (&changed).replace("nine","n9e");
        changed = (&changed).replace("seven","s7n");
        changed = (&changed).replace("one","o1e");
        changed = (&changed).replace("two","t2o");
        changed = (&changed).replace("five","f5e");
        changed = (&changed).replace("eight","e8t");
        //println!("{}", changed);
    if let Some(n) = RE.captures(&changed) {
      //  println!("{:?}",n);
        let mut r = parse_word(n[1].to_owned());
        let add = if let Some(d) = n.get(n.len()-1) {
            parse_word(d.as_str().to_owned())
        } else {
            r.clone()
        };
        r.push_str(&add);
            usize::from_str_radix(&r,10).expect("Parse failed")
    } else {
        panic!("Failed to capture");
    }
})
    .collect::<Vec<usize>>()
}


pub fn run(file: &str) -> (usize,usize) {
let p1 = read_data(file).iter().sum();
let fname = if file.contains("sample") {
    "sample_data/day1_pt2.txt"
} else {
    file
};

println!("{:?}",fname);

let p2 = read_data_pt2(fname).iter().sum();
return (p1,p2);
}

fn parse_word(s: String) -> String {
    if let Ok(_) = usize::from_str_radix(&s,10) {
        s
    } else {
        match s.as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "zero" => "0",
        &_ => panic!("Unexpected match case: {}", s),
        }.to_owned()
    }
}