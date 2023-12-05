#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    // day1();
    // day2();
    // day3();
    // day4();
    day5();
}


fn day1() {
    let test_res = day1::run("sample_data/day1.txt");
    assert_eq!(test_res.0,142);
    assert_eq!(test_res.1,281);
    let res = day1::run("puzzle_input/day1.txt");
    println!("{:?}", res);
}
fn day2() {
    let test_res = day2::run("sample_data/day2.txt");
    assert_eq!(test_res.0,8);
    assert_eq!(test_res.1,2286);
    let res = day2::run("puzzle_input/day2.txt");
    println!("{:?}", res);
}
fn day3() {
    let test_res = day3::run("sample_data/day3.txt");
    assert_eq!(test_res.0,4361);
    assert_eq!(test_res.1,467835);
    let res = day3::run("puzzle_input/day3.txt");
    println!("{:?}",res);
}
fn day4() {
    let test_res = day4::run("sample_data/day4.txt");
    assert_eq!(test_res.0,13);
    assert_eq!(test_res.1,30);
    let res = day4::run("puzzle_input/day4.txt");
    println!("{:?}",res);
}
fn day5() {
    let test_res = day5::run("sample_data/day5.txt");
    assert_eq!(test_res.0,35);
    assert_eq!(test_res.1,46);
    let res = day5::run("puzzle_input/day5.txt");
    println!("{:?}",res);
}