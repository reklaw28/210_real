use std::collections::HashMap;
use rand::Rng;
use std::collections::BinaryHeap;
mod open;

fn main() {
    let data = crate::open::read_file("C:\\Users\\sirbu\\Downloads\\DS_210_lectures\\homeworks\\Final_project\\make_graph\\Final_Data.csv");

    let mut all_point = vec![];
    for i in &data {
        let a = crate::open::point {
            WIN: (*i.0).to_string(),
            LOSS: (*i.1).to_string(),
        };
        all_point.push(a);
    }

    let mut adjacency_list = crate::open::ad_list(all_point);

    let mut indexs: HashMap<String, usize> = Default::default(); //making a hashmap so we can find vals out of the strings 
    let mut count = 0;
    for i in &adjacency_list {
        indexs.insert(i.0.clone(), count);
        count += 1;
    }

    let walks = 1212;
    let steps_per_walk = 1000; // Number of steps per random walk
    let mut probabilities = vec![0; walks]; // Initialize probabilities for each vertex
    let size = adjacency_list.len();

    for _ in 0..walks {
        let mut current_vertex = rand::thread_rng().gen_range(0..size);

        for _ in 0..steps_per_walk {
            let temp = &mut adjacency_list[current_vertex]; // making pos values more heavily weighte
            let min = crate::open::min(&temp.1);
            let mut max = crate::open::max(&temp.1);
            let temp2 = crate::open::scale(temp.1.clone(), min); // gives sevre prefernece to postive / wins
            let jump = rand::thread_rng().gen_range(min..=max) as i32; // Random jump probability
            let _store: Vec<(String, i32)> = vec![]; // so we can only itterate through strictly pos or negative entries 
            let mut store2: Vec<(String, i32)> = vec![];

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
                    for i in &mut store {
                        for _ in 0..(i.1) {// increase the prob a higher num gets selected 
                            store2.push(i.clone());
                        }
                    }
                    let next_vertex = rand::thread_rng().gen_range(0..store2.len());
                    current_vertex = indexs[&store2[next_vertex].0];
                } else {
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
                    if min == 0 {
                        store2 = store;
                    } else {
                        for i in &mut store {
                            for _ in 0..(i.1 * -1) {// increase the prob a higher num gets selected 
                                store2.push(i.clone());//if there are more losses increases prob being selected
                            }
                        }
                    }
                    let next_vertex = rand::thread_rng().gen_range(0..store2.len());
                    current_vertex = indexs[&store2[next_vertex].0];
                } else {
                    current_vertex = rand::thread_rng().gen_range(0..size);
                }
            }

            // Increment the count for the visited vertex
            probabilities[current_vertex] += 1;// keep as an int so can order
        }
    }
    let mut prob_heap = BinaryHeap::new();//initialize binary heap

    let mut prob_vec: Vec<(usize, usize)> = vec![];
    for (vertex, prob) in probabilities.iter().enumerate() {
        prob_vec.push((*prob, vertex)); //invert how would normally store bc of tuple ordering

    }

    for i in prob_vec {
        prob_heap.push(i);//adding to binary heap to sort
    }
    let mut finale: Vec<(usize, usize)> = vec![];
    for _ in 0..15 {
        finale.push(prob_heap.pop().unwrap()); //finding the top 5 best from our random selection and adding them
    }

    for i in finale {
        println!("Vertice = {:?}, and prob/rank = {:.4}", adjacency_list[i.1].0, (i.0 as f64) / (walks * steps_per_walk) as f64);//printing the top 5
    }
}
