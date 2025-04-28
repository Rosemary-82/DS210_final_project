

fn read_nodes(filename:&str) ->HashMap<usize, String>{
    let mut rdr=ReaderBuilder::new().has_headers(true).from_path(filename)
    .expect("Cannot open CSV file");

    let mut nodes=HashMap::new();
    for line in rdr.records(){
        let record = line.expect("Failed to read record");
        let name=record[1].trim().to_string;
        let new_id=record[2].trim().parse::<usize>().expect("Invalid new_id");

        nodes.insert(new_id, name);
    }
    nodes
}
fn read_edges(filename:&str) ->Vec<(usize,usize)>{
    let mut rdr=ReaderBuilder::new().has_headers(true).from_path(filename)
    .expect("Cannot open CSV file");

    let mut edges=HashMap::new();
    for line in rdr.records(){
        let record = line.expect("Failed to read record");
        let from=record[0].trim().parse::<usize>().expect("Invalid new_id");;
        let to=record[1].trim().parse::<usize>().expect("Invalid new_id");

        edges.insert(from, to);
    }
    edges
}
fn find_connections(nodes:HashMap<usize, String>, edges:Vec<(usize,usize)>)->HashMap<usize,Vec<usize>>{
    let connections=HashMap<usize,Vec<usize>>::new();
    for (new_id,name) in nodes{
        for (from,to) in edges{
            if new_id==from{
                connections.entry(from).or_insert(Vec::new()).push(to)；
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
            popular_id=*id;
        }
    }
    nodes.get(&popular_id).cloned().unwrap_or("Unknown".to_string())
}
fn 3_steps_bfs(graph:HashMap<usize,Vec<usize>>,start:String) -> Option<usize>{
    let start_id = self.names_ids.iter()
            .find(|(_, name)| name.contains(start_name))
            .map(|(id, _)| *id);
    
    if let Some(id) = start_id {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        visited.insert(id);
        queue.push_back((id, 0)); 
        while let Some((current,depth))=queue.pop_front(){
            if depth>=3{
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



fn main() {
    let nodes = read_nodes("fb-pages-sport.nodes");
    let edges = read_edges("fb-pages-sport.edges");
    let connections = find_connections(nodes,edges);
    let popular_person=popular_person(connections,nodes);
    println!("The most popular person is {:?}",popular_person);
    let reached=3_steps_bfs(connections, popular_person);
    let 3_steps_ratio=reached/(connections.len());
    println!("The proportion of the most popular person can reach in three steps is {:?}",3_steps_ratio);
}
