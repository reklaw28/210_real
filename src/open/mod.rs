use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::cmp;


pub fn read_file(path: &str) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x = v[0].parse::<String>().unwrap();
        let y = v[1].parse::<String>().unwrap();
        result.push((x, y));
    }
    result
}

pub fn min(x: &[(String, i32)]) -> i32 {
    x.iter().fold(i32::MAX, |min_val, (_, val)| cmp::min(min_val, *val))
}

pub fn max(x: &[(String, i32)]) -> i32{
    x.iter().fold(i32::MAX, |max_val,(_,val)| cmp::max(max_val, *val))
}

pub fn scale(mut x: Vec<(String, i32)>, min_val: i32) -> Vec<(String, i32)> {
    for mut i in & mut x{
        if i.1 >0{
            i.1 = i.1+(min_val*min_val);
            
        }
    }
    return x.to_vec()}
