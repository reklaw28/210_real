use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::io;

type Vertex = String;
type Edge = (Vertex, Vertex);
type ListOfEdges = Vec<Edge>;
type AdjacencyLists = Vec<Vec<Vertex>>;


#[derive(Debug)]
struct point{
	WIN: String,
	LOSS: String,
	weight: i32,
}

fn read_file(path: &str) -> ListOfEdges {
    let mut result: ListOfEdges = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x = v[0].to_string();
        let y = v[1].to_string();
        result.push((x, y));
    }
    result
}


fn main() {
    let data = read_file("C:\\Users\\sirbu\\Downloads\\DS_210_lectures\\homeworks\\Final_project\\make_graph\\Final_Data.csv");


    let mut all_point = vec![];
    for i in &data{
    	let mut a = point{
    		WIN: (*i.0).to_string(),
    		LOSS: (*i.1).to_string(),
    		weight: 1,
    	};
    	all_point.push(a);
    }

    let mut win_hash: HashMap::<String, Vec<String>> = HashMap::new(); // making a hash map so that i can keep track of each win a team has before constructing graph
    let mut loss_hash: HashMap::<String, Vec<String>> = HashMap::new();

    for i in &all_point{
    	win_hash.entry((*i.WIN).to_string()).or_insert(Vec::new()).push((*i.LOSS).to_string());//making the inputs append to a vec so all are there
    }
    //println!("{:?}",win_hash );
    for i in &all_point{
    	loss_hash.entry((*i.LOSS).to_string()).or_insert(Vec::new()).push((*i.WIN).to_string());
    }

    let mut vec_of_vertex: Vec<String> = vec![]; // making a vector of all teams to use as a key 
    for i in &win_hash{
    	if i.0 != "WIN"{// removing headers from list
    	vec_of_vertex.push(i.0.to_string());// taking only the keys
    }

    }
    for i in &loss_hash{
    	if !(vec_of_vertex.iter().any(|e| (i.0).contains(e))){
    		vec_of_vertex.push((i.0).to_string());}
    	else if i.0 == "LOSS"{ // removing headers from list
    		println!("nah");
    	}
    }// making sure we have every key


    println!("{:?}",vec_of_vertex.len() );


    let mut adjacency_list: Vec<(String, Vec<(String,i32)>)> = vec![];
    for i in vec_of_vertex{
    	let mut points: Vec<(String,i32)> = vec![];
    	if win_hash.contains_key(&i){// making sure key exists so it doesn't panic 
    	for j in &win_hash[&i]{
    		if points.iter().any(|e| j.contains(&e.0)){
    			for mut z in &mut points{
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
    			for mut z in &mut points{
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

    println!("{:?}",adjacency_list[0]);

    println!("{:?}",loss_hash["Oklahoma: Shadman & Sullivan"] );
    let mut count = 0;
    for i in adjacency_list{
    	for j in i.1{
    		count += j.1;
    	}
    }
    println!("{:?}",count );// this is close to what it should be 

    let mut ident_mat: Vec<Vec<f32>> = vec![vec![0.0; 1212]; 1212];//making an identity matrix for later lin alg
    for i in 0..1212{
    	ident_mat[i][i] = 1.0;
    }


    }


