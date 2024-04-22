use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::io;
use rand::Rng;
use std::cmp;
use std::collections::BinaryHeap;


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

//create a function that adds the absolute minimum to all of the positve records on a duplicate copy then use those vals as the rand num gen

fn min(x: &[(String, i32)]) -> i32 {
    x.iter().fold(i32::MAX, |min_val, (_, val)| cmp::min(min_val, *val))
}

fn max(x: &[(String, i32)]) -> i32{
	x.iter().fold(i32::MAX, |max_val,(_,val)| cmp::max(max_val, *val))
}

fn scale(mut x: Vec<(String, i32)>, min_val: i32) -> Vec<(String, i32)> {
    for mut i in & mut x{
    	if i.1 >0{
    		i.1 = i.1+(min_val*min_val);
    		
    	}
    }
    return x.to_vec()
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
    let mut indexs: HashMap<String,usize>= Default::default();; //making a hashmap so we can find vals out of the strings 
    let mut count = 0;
    for i in &adjacency_list{
    	indexs.insert(i.0.clone(),count);
    	count +=1;
    }


    let mut count = 0;
    for i in &adjacency_list{
    	for j in &i.1{
    		count += j.1;
    	}
    }


    let walks = 12120;
    let steps_per_walk = 10000; // Number of steps per random walk
    let mut probabilities = vec![0; walks]; // Initialize probabilities for each vertex
    let size = adjacency_list.len();



    for _ in 0..walks {
        let mut current_vertex = rand::thread_rng().gen_range(0..size);

        let mut temp = &mut adjacency_list[current_vertex]; // making pos values more heavily weighte
        let min = min(&temp.1);
    	let temp2 = scale(temp.1.clone(), min); // gives sevre prefernece to postive / wins
    	let max = max(&temp2);
        for _ in 0..steps_per_walk {
            let jump = rand::thread_rng().gen_range(min..=max) as u8; // Random jump probability
            let mut store: Vec<(String, i32)> = vec![]; // so we can only itterate through strictly pos or negative entries 
            if temp2.is_empty() {
    			// Jump to a random vertex since temp2 is empty
    			current_vertex = rand::thread_rng().gen_range(0..size);
			} else if jump > 0 {
    			// Sample from temp2 only if it has positive values
    			let mut store: Vec<(String, i32)> = vec![];
    			for i in &temp2 {
        			if i.1 > 0 {
            			store.push(i.clone());
        	}
    	}
    			if !store.is_empty() {
        			let next_vertex = rand::thread_rng().gen_range(0..store.len());
        			current_vertex = indexs[&store[next_vertex].0];
    		}else{
    			current_vertex = rand::thread_rng().gen_range(0..size);
    		}
			} else if jump <= 0 {
    			// Sample from temp2 only if it has non-positive values
    			let mut store: Vec<(String, i32)> = vec![];
    			for i in &temp2 {
        			if i.1 <= 0 {
            		store.push(i.clone());
        	}
    	}
    		if !store.is_empty() {
        		let next_vertex = rand::thread_rng().gen_range(0..store.len());
        		current_vertex = indexs[&store[next_vertex].0];
    		}else{
    			current_vertex = rand::thread_rng().gen_range(0..size);
    		}
}


            // Increment the count for the visited vertex
            probabilities[current_vertex] += 1;// keep as an int so can order
        }
    }
    let mut prob_heap = BinaryHeap::new();//initialize binary heap

    let mut prob_vec: Vec<(usize,usize)> = vec![];
    for (vertex, prob) in probabilities.iter().enumerate() {
    	prob_vec.push((*prob,vertex)); //invert how would normally store bc of tuple ordering
    }

    for i in prob_vec{
    	prob_heap.push(i);//adding to binary heap to sort
    }
    let mut finale: Vec<(usize,usize)> = vec![];
    for _ in 0..5{
    	finale.push(prob_heap.pop().unwrap()); //finding the top 5 best from our random selection and adding them
    }

    for i in finale{
    	println!("Vertice = {:?}, and prob/rank = {:.4}",adjacency_list[i.1].0, (i.0 as f64)/(walks*steps_per_walk)as f64 );//printing the top 5
    }


    }


