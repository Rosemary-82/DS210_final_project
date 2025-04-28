struct Nodes{
    name:String,
    new_id:usize,
}
struct Graph{
    names_ids:HashMap<usize,String>,
    edges:Vec<(usize,usize)>,
    adjacency_list:HashMap<usize,Vec<usize>>,
}
impl Graph{
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

}


fn main() {
    println!("Hello, world!");
}
