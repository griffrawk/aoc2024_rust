
#[cfg(test)]
mod tests {
    use std::env;
    use aocutils::point::Point;
    use aocutils::graph::Graph;

    #[test]
    fn test_shortest_path() {
        println!("Current directory {}", env::current_dir().unwrap().display());
        
        let mut graph = Graph::new("src/bin/day16/test_graph.txt");
        assert_eq!(graph.shortest_path(), Some(28));
        graph.visual_plot().unwrap();
    }
}