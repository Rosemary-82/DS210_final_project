use std::collections::{HashMap, HashSet,VecDeque};
use std::io::{BufReader};
use std::fs::File;
use csv::Reader;

fn read_nodes(filename: &str) -> HashMap<usize, String> {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut rdr = Reader::from_reader(reader);

    let mut nodes = HashMap::new();
    let mut name_counts = HashMap::new();
    for line in rdr.records() {
        let record = line.expect("Failed to read record");
        let base_name = record[1].trim().to_string(); 
        let new_id = record[2].trim().parse::<usize>().expect("Invalid new_id");

        let count=name_counts.entry(base_name.clone()).or_insert(0);
        *count+=1;
        let unique_name = if *count == 1 {
            base_name.clone()
        } else {
            format!("{}_{}", base_name, count)
        };

        nodes.insert(new_id, unique_name);
    }
    nodes
}

fn read_edges(filename: &str) -> Vec<(usize, usize)> {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut rdr = Reader::from_reader(reader);

    let mut edges = Vec::new(); 
    for line in rdr.records() {
        let record = line.expect("Failed to read record");
        let from = record[0].trim().parse::<usize>().expect("Invalid from ID");
        let to = record[1].trim().parse::<usize>().expect("Invalid to ID");

        edges.push((from, to)); 
    }
    edges
}

fn find_connections(nodes:HashMap<usize, String>, edges:&Vec<(usize,usize)>)->HashMap<usize,Vec<usize>>{
    let mut connections=HashMap::<usize,Vec<usize>>::new();
    for (new_id,name) in nodes.into_iter(){
        for (from,to) in edges{
            if new_id==*from{
                connections.entry(*from).or_insert(Vec::new()).push(*to);
            }
        }
    }
    connections
}
fn popular_person(connections:HashMap<usize,Vec<usize>>, nodes:HashMap<usize, String>)->String{
    let mut max_connection=0;
    let mut popular_id=0;
    for (id,neighbors) in connections{
        if neighbors.len()>max_connection{
            popular_id=id;
            max_connection=neighbors.len();
        }
    }
    nodes.get(&popular_id).cloned().unwrap_or("Unknown".to_string())
}
fn three_steps_bfs(graph:HashMap<usize,Vec<usize>>, nodes:HashMap<usize, String>, start:String, depth_threshold:usize) -> Option<usize>{
    let start_id = nodes.iter()
            .find(|(_, name)| name.contains(&start))
            .map(|(id, _)| *id);
    
    if let Some(id) = start_id {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        visited.insert(id);
        queue.push_back((id, 0)); 
        while let Some((current,depth))=queue.pop_front(){
            if depth>=depth_threshold{
                continue;
            }
            if let Some(neighbors)=graph.get(&current){
                //println!("{:?}", neighbors);
                for &neighbor in neighbors{
                    if !visited.contains(&neighbor){
                        visited.insert(neighbor);
                        queue.push_back((neighbor,depth+1));
                        //println!("Queue added to");
                        //println!("Count of visited so far: {}\n Neighbor name: {}", visited.len(), neighbor);
                    }

                }
            }
        }
        Some(visited.len()-1)
    }else{
        None
    }
}



fn main() {
    let nodes = read_nodes("fb-pages-sport_nodes.csv");
    let edges = read_edges("fb-pages-sport_edges.csv");
    let connections = find_connections(nodes.clone(),&edges);
    let popular_person=popular_person(connections.clone(),nodes.clone());
    println!("The most popular person is {:?}",popular_person);
    let reached=three_steps_bfs(connections.clone(), nodes.clone(), popular_person, 3);
    println!("{:?}", reached);
    println!("{:?}", connections.len());
    let three_steps_ratio: f64 =((reached.unwrap() as f64)/(connections.len() as f64)) * 100.0;
    println!("The proportion of the most popular person can reach in three steps is {:.2} %",three_steps_ratio);
}
