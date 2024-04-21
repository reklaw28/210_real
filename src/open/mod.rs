use std::fs::File;
use std::io::prelude::*;
use rand::Rng;


pub fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        result.push((x, y));
    }
    result
}

#[cfg(test)]
    #[test]
        fn equal_to_one(){
            let data = read_file("C://Users/sirbu//Downloads//DS_210_lectures//homeworks//homework_10//page//src//data.txt");
            type Vertex = usize;
            type ListOfEdges = Vec<(Vertex, Vertex)>;
            type AdjacencyLists = Vec<Vec<Vertex>>;

            #[derive(Debug)]
            struct Graph {
                n: usize,
                outedges: AdjacencyLists,
            }

            impl Graph {
                fn add_directed_edges(&mut self, edges: &ListOfEdges) {
                    for (u, v) in edges {
                        self.outedges[*u].push(*v);
                    }
                }

                fn sort_graph_lists(&mut self) {
                    for l in self.outedges.iter_mut() {
                        l.sort();
                    }
                }

                fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
                    let mut g = Graph { n, outedges: vec![vec![]; n] };
                    g.add_directed_edges(edges);
                    g.sort_graph_lists();
                    g
                }
    }
        //my code start
            let mut graph = Graph::create_directed(1000, &data);

            let mut probabilities = vec![0; graph.n]; // Initialize probabilities for each vertex
            let walks = 100; // Number of random walks
            let steps_per_walk = 100; // Number of steps per random walk

            for _ in 0..walks {
                let mut current_vertex = rand::thread_rng().gen_range(0..graph.n);

                for _ in 0..steps_per_walk {
                    let jump = rand::thread_rng().gen_range(1..=10) as u8; // Random jump probability

                    if graph.outedges[current_vertex].is_empty() || jump == 10 {
                        current_vertex = rand::thread_rng().gen_range(0..graph.n);
            }       else {
                        let next_vertex = rand::thread_rng().gen_range(0..graph.outedges[current_vertex].len());
                        current_vertex = graph.outedges[current_vertex][next_vertex];
            }

            // Increment the count for the visited vertex
                    probabilities[current_vertex] += 1;
        }
    }
    let mut count = 0;
    for i in probabilities{//adding all the probabilities together 
        count +=i;
    }  
    assert_eq!( 10000, count);// making sure they are == using 10000 here because thats 100*100

        }