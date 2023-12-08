#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    // day1();
    // day2();
    // day3();
    // day4();
    // day5();
    // day6();
    // day7();
    day8();
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
fn day6() {
    let test_res = day6::run("sample_data/day6.txt");
    assert_eq!(test_res.0,288);
    assert_eq!(test_res.1,71503);
    let res = day6::run("puzzle_input/day6.txt");
    println!("{:?}",res);
}
fn day7() {
    let test_res = day7::run("sample_data/day7.txt");
    assert_eq!(test_res.0,6440);
    assert_eq!(test_res.1,5905);
    let res = day7::run("puzzle_input/day7.txt");
    println!("{:?}",res);
}
fn day8() {
    let test_res = day8::run_p1("sample_data/day8-1.txt");
    assert_eq!(test_res,2);
    let test_res = day8::run_p1("sample_data/day8-2.txt");
    assert_eq!(test_res,6);
    let test_res = day8::run_p2("sample_data/day8-3.txt");
    assert_eq!(test_res,6);
    let res_1 = day8::run_p1("puzzle_input/day8.txt");
    let res_2 = day8::run_p2("puzzle_input/day8.txt");
    println!("{:?}",(res_1,res_2));
}