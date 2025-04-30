mod graph {
    use std::collections::{HashMap, HashSet,VecDeque};
    use std::io::{BufReader};
    use std::fs::File;
    use csv::Reader;
    
    //Represents a graph of people and their connections.
    pub struct GraphData {
        pub nodes: HashMap<usize, String>,       // ID to Name
        pub edges: Vec<(usize, usize)>,          // Directed edges (from, to)
        pub connections: HashMap<usize, Vec<usize>>, // Adjacency list
    }

    impl GraphData {
        //Reading node data from a CSV file, ensures unique names, and returns a mapping from node IDs to unique names.
        pub fn read_nodes(filename: &str) -> HashMap<usize, String> {
            let file = File::open(filename).expect("Cannot open file");
            let reader = BufReader::new(file);
            let mut rdr = Reader::from_reader(reader);

            let mut nodes = HashMap::new();
            let mut name_counts = HashMap::new();
            for line in rdr.records() {
                let record = line.expect("Failed to read record");
                let base_name = record[1].trim().to_string(); 
                let new_id = record[2].trim().parse::<usize>().expect("Invalid new_id");

                let count=name_counts.entry(base_name.clone()).or_insert(0);//this counts how many times each name has appeared so far, to ensure uniqueness.
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
        //reading a list of ID pairs, which represent edges from a CSV file.
        pub fn read_edges(filename: &str) -> Vec<(usize, usize)> {
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
        //This builds an adjacency list representing outgoing edges for each node.
        pub fn find_connections(nodes:HashMap<usize, String>, edges:&Vec<(usize,usize)>)->HashMap<usize,Vec<usize>>{
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
        //this identifies the node with the most outgoing connections and returns its name.
        pub fn popular_person(connections:HashMap<usize,Vec<usize>>, nodes:HashMap<usize, String>)->String{
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
        //This function runs a breadth-first search from the given start node and returns how many nodes can be reached within the given number of steps.
        pub fn three_steps_bfs(graph:HashMap<usize,Vec<usize>>, nodes:HashMap<usize, String>, start:String, depth_threshold:usize) -> Option<usize>{
            let start_id = nodes.iter()
                    .find(|(_, name)| name.contains(&start))
                    .map(|(id, _)| *id);//This tries to find the start node ID based on a name substring match.
            
            if let Some(id) = start_id {
                let mut visited = std::collections::HashSet::new();//this tracks which nodes have been visited to avoid revisiting in BFS.
                let mut queue = std::collections::VecDeque::new();

                visited.insert(id);
                queue.push_back((id, 0)); 
                while let Some((current,depth))=queue.pop_front(){
                    if depth>=depth_threshold{
                        continue;
                    }
                    if let Some(neighbors)=graph.get(&current){
                        for &neighbor in neighbors{
                            if !visited.contains(&neighbor){
                                visited.insert(neighbor);
                                queue.push_back((neighbor,depth+1));
                            }

                        }
                    }
                }
                Some(visited.len()-1)
            }else{
                None
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::graph::GraphData;
    use std::collections::HashMap;
    #[test]
    fn test_three_steps_bfs() {
        let nodes = vec![
            (0, "Alice".to_string()),
            (1, "Bob".to_string()),
            (2, "Charlie".to_string()),
            (3, "Dave".to_string()),
            (4, "Eve".to_string()),
        ].into_iter().collect::<HashMap<usize, String>>();

        // Create mock edges (Alice → Bob → Charlie → Dave → Eve)
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
        ];

        let graph = GraphData::find_connections(nodes.clone(), &edges);
        let start_name = "Alice".to_string();
        let reached = GraphData::three_steps_bfs(graph.clone(), nodes.clone(), start_name, 3);

        assert_eq!(reached, Some(3)); // Alice can reach Bob, Charlie, Dave (3 others in 3 steps)
    }

    #[test]
    fn test_popular_person() {
        let nodes = vec![
            (0, "Alice".to_string()),
            (1, "Bob".to_string()),
            (2, "Charlie".to_string()),
        ].into_iter().collect::<HashMap<usize, String>>();

        let edges = vec![
            (0, 1),
            (0, 2),
            (1, 2),
        ];

        let connections = GraphData::find_connections(nodes.clone(), &edges);
        let most_popular = GraphData::popular_person(connections, nodes);

        assert_eq!(most_popular, "Alice");
    }
}
fn main() {
    use crate::graph::GraphData;
    
    let nodes = GraphData::read_nodes("fb-pages-sport_nodes.csv");
    let edges = GraphData::read_edges("fb-pages-sport_edges.csv");
    let connections = GraphData::find_connections(nodes.clone(),&edges);
    let popular_person=GraphData::popular_person(connections.clone(),nodes.clone());
    println!("The most popular person is {:?}",popular_person);
    let reached=GraphData::three_steps_bfs(connections.clone(), nodes.clone(), popular_person, 3);
    let three_steps_ratio: f64 =((reached.unwrap() as f64)/(connections.len() as f64)) * 100.0;
    println!("The proportion of the most popular person can reach in three steps is {:.2} %",three_steps_ratio);
}
