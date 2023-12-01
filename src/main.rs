#![allow(dead_code)]



mod day1;

fn main() {
    day1();
}

fn day1() {
    let test_res = day1::run("sample_data/day1.txt");
    assert_eq!(test_res.0,142);
    assert_eq!(test_res.1,281);
    let res = day1::run("puzzle_input/day1.txt");
    println!("{:?}", res);
}