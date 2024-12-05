use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp;
#[derive(Debug)]
pub struct point{
    pub WIN: String,
    pub LOSS: String,
}


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
    x.iter().fold(i32::MIN, |max_val,(_,val)| cmp::max(max_val, *val))
}

pub fn scale(mut x: Vec<(String, i32)>, min_val: i32) -> Vec<(String, i32)> {
    for i in & mut x{ // basing off of losses makes more sense because then people will be visited alot if alot of people have lost to them
        if i.1 <0 {
            i.1 = i.1 + min_val;
            
        }
    }
    return x.to_vec()}


pub fn ad_list(x: Vec<point>) -> Vec<(String, Vec<(String, i32)>)> {
    let mut win_hash: HashMap<String, Vec<String>> = HashMap::new();
    let mut loss_hash: HashMap<String, Vec<String>> = HashMap::new();

    // Populate win_hash and loss_hash
    for point in &x {
        win_hash.entry(point.WIN.clone()).or_default().push(point.LOSS.clone());
        loss_hash.entry(point.LOSS.clone()).or_default().push(point.WIN.clone());
    }

    // Collect all unique teams
    let mut teams: Vec<String> = win_hash.keys().cloned().collect();
    for team in loss_hash.keys() {
        if !teams.contains(team) {
            teams.push(team.clone());
        }
    }

    // Build the adjacency list
    let mut adjacency_list: Vec<(String, Vec<(String, i32)>)> = Vec::new();
    for team in teams {
        let mut points: Vec<(String, i32)> = Vec::new();

        // Process wins
        if let Some(opponents) = win_hash.get(&team) {
            for opponent in opponents {
                if let Some(point) = points.iter_mut().find(|(op, _)| op == opponent) {
                    point.1 += 1;
                } else {
                    points.push((opponent.clone(), 1));
                }
            }
        }

        // Process losses
        if let Some(opponents) = loss_hash.get(&team) {
            for opponent in opponents {
                if let Some(point) = points.iter_mut().find(|(op, _)| op == opponent) {
                    point.1 -= 1;
                } else {
                    points.push((opponent.clone(), -1));
                }
            }
        }

        adjacency_list.push((team, points));
    }

    adjacency_list
}

#[cfg(test)]
#[test]
fn graph_right(){
    let data = read_file("C:\\Users\\sirbu\\Downloads\\sorted_data_real.csv");
    let mut all_point = vec![];
    for i in &data{
        let a = point{
            WIN: (*i.0).to_string(),
            LOSS: (*i.1).to_string(),
        };
        all_point.push(a);
    }

    let mut adjacency_list = ad_list(all_point);
    let mut score: i32 = 0;

    for i in adjacency_list{
        for j in i.1{
            score+=j.1;
        }
    }
    assert_eq!( 0,score );
}
#[test]
fn scale_right(){
    let data = read_file("C:\\Users\\sirbu\\Downloads\\DS_210_lectures\\homeworks\\Final_project\\make_graph\\test.csv");
    let mut all_point = vec![];
    for i in &data{
        let a = point{
            WIN: (*i.0).to_string(),
            LOSS: (*i.1).to_string(),
        };
        all_point.push(a);
    }
    let mut scal = vec![];

    let mut adjacency_list = ad_list(all_point);
    for i in &mut adjacency_list{
            let min_val = min(&i.1);
            let scaler = (i.1).clone();
            scal = scale(scaler, min_val);

    }
    assert_eq!( -4,scal[0].1, "should be {} and is {}",-2, scal[0].1);
}