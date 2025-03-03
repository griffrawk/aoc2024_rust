mod a_star;
mod day16_graph;
mod dfs;
mod dijkstra;
mod visuals;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::day16_graph::Graph;

    #[test]
    fn test_part_one_test_a() {
        let mut graph = Graph::new("src/bin/day16/data/day16_test_a.txt");
        if let Some(res) = graph.shortest_path() {
            graph.visual_plot(true).unwrap();
            assert_eq!(res, 7036);
        }
    }

    #[test]
    fn test_part_one_test_b() {
        let mut graph = Graph::new("src/bin/day16/data/day16_test_b.txt");
        if let Some(res) = graph.shortest_path() {
            graph.visual_plot(true).unwrap();
            assert_eq!(res, 11048);
        }
    }

    #[test]
    fn test_part_one_data_dijkstra() {
        let mut graph = Graph::new("src/bin/day16/data/day16_data.txt");
        if let Some(res) = graph.shortest_path() {
            graph.visual_plot(true).unwrap();
            assert_eq!(res, 107512);
        }
    }

    #[test]
    fn test_part_one_data_a_star() {
        // let mut graph = Graph::new("src/bin/day16/data/day16_test_a.txt");
        let mut graph = Graph::new("src/bin/day16/data/day16_data.txt");
        if let Some(res) = graph.a_star() {
            graph.a_star_visual_plot(res, true).unwrap();
            // assert_eq!(res, 7036);
            assert_eq!(res, 107512);
        }
    }

    #[test]
    fn test_part_one_test_a_with_astar_out() {
        let mut graph = Graph::new("src/bin/day16/data/day16_test_a.txt");
        if let Some(res) = graph.shortest_path() {
            graph.a_star_visual_plot(res, true).unwrap();
            assert_eq!(res, 7036);
        }
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new("src/bin/day16/data/day16_test_a.txt");
        let res = graph.rec_dfs();
        let _ = graph.visual_plot(false).unwrap();
    }
    
    #[test]
    fn test_dfs_path() {
        println!("debug_assertions is {}", cfg!(debug_assertions));
        if !cfg!(debug_assertions) {
            println!("Release!");
        }
        
        let mut graph = Graph::new("src/bin/day16/data/day16_test_a.txt");
        let res = graph.rec_dfs_path();
        dbg!(res);
        let _ = graph.visual_plot(false).unwrap();
    }
}
