use std::fs::File;
use std::io::prelude::*;
use rand::Rng;


fn main() {

fn read_file(path: &str) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        println!("{:?}",line );
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x = v[0].parse::<String>().unwrap();
        let y = v[1].parse::<String>().unwrap();
        result.push((x, y));
    }
    result
}

	let data = read_file("C:\\Users\\sirbu\\Downloads\\DS_210_lectures\\homeworks\\Final_project\\make_graph\\Final_Data.csv");

	println!("{:?}",data[0] );

}
