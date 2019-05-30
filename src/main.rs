#![allow(dead_code)]

pub mod tuple;
use regex::Regex;
use std::vec::Vec;

fn parse_floats(input: &str) -> Vec<f32> {
    let nums = input
        .split(", ")
        .filter_map(|s| s.parse::<f32>().ok())
        .collect::<Vec<_>>();
    return nums;
}

fn main() {
    let re = Regex::new(r"(tuple|point|vector)\((.*)\)").unwrap();
    let caps = re.captures("tuple(1, 2, 3)").unwrap();
    assert_eq!("tuple", &caps[1]);
    let e = vec![1.0, 2.0, 3.0];
    assert_eq!(e, parse_floats(&caps[2]));

    println!("Go speedracer!");
}
