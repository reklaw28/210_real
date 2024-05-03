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


pub fn ad_list (x: Vec<point>) -> (Vec<(String, Vec<(String,i32)>)>){
    let mut win_hash: HashMap::<String, Vec<String>> = HashMap::new(); // making a hash map so that i can keep track of each win a team has before constructing graph
    let mut loss_hash: HashMap::<String, Vec<String>> = HashMap::new();

    for i in &x{
        win_hash.entry((*i.WIN).to_string()).or_insert(Vec::new()).push((*i.LOSS).to_string());//making the inputs append to a vec so all are there
    }
    //println!("{:?}",win_hash );
    for i in &x{
        loss_hash.entry((*i.LOSS).to_string()).or_insert(Vec::new()).push((*i.WIN).to_string());
    }

    let mut vec_of_vertex: Vec<String> = vec![]; // making a vector of all teams to use as a key 
    for i in &win_hash{
        if i.0 != "WIN"{// removing headers from list
        vec_of_vertex.push(i.0.to_string());// taking only the keys
    }

    }
    for i in &loss_hash{
        if i.0 == "LOSS"{
            println!("nah");
        }
        else if !(vec_of_vertex.iter().any(|e| (i.0).contains(e))){
            vec_of_vertex.push((i.0).to_string());}
    }// making sure we have every key

    // there are 1212 nodes 

    let mut adjacency_list: Vec<(String, Vec<(String,i32)>)> = vec![];
    for i in vec_of_vertex{
        let mut points: Vec<(String,i32)> = vec![];
        if win_hash.contains_key(&i){// making sure key exists so it doesn't panic 
        for j in &win_hash[&i]{
            if points.iter().any(|e| j.contains(&e.0)){
                for z in &mut points{
                    if (*j).to_string() == z.0{
                        z.1 +=1
                    }
                }
            }
            else{
            points.push((j.to_string(),1));
        }
    }


        }
        if loss_hash.contains_key(&i){// making sure key exist so it doesnt panic
        for j in &loss_hash[&i]{
            if points.iter().any(|e| j.contains(&e.0)){
                for z in &mut points{
                    if (*j).to_string() == z.0{
                        z.1-=1
                    }
                }
            }
            else{
            points.push((j.to_string(),-1));
        }
        
        }
    }

    adjacency_list.push((i,points));
    }
    return adjacency_list
}

#[cfg(test)]
#[test]
fn graph_right(){
    let data = read_file("C:\\Users\\sirbu\\Downloads\\DS_210_lectures\\homeworks\\Final_project\\make_graph\\Final_Data.csv");
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