mod day16_part_one;

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::day16_part_one::Graph;

    #[test]
    fn test_part_one_test_a() {
        let mut graph = Graph::new("src/bin/day16/day16_test_a.txt");
        if let Some(res) = graph.shortest_path() {
            graph.visual_plot(true).unwrap();
            assert_eq!(res, 7036);
        }
    }

    #[test]
    fn test_part_one_test_b() {
        let mut graph = Graph::new("src/bin/day16/day16_test_b.txt");
        if let Some(res) = graph.shortest_path() {
            graph.visual_plot(true).unwrap();
            assert_eq!(res, 11048);
        }
    }

    #[test]
    fn test_part_one_data() {
        let mut graph = Graph::new("src/bin/day16/day16_data.txt");
        if let Some(res) = graph.shortest_path() {
            graph.visual_plot(true).unwrap();
            assert_eq!(res, 107512);
        }
    }

    #[test]
    fn test_part_one_astar() {
        // let mut graph = Graph::new("src/bin/day16/day16_test_a.txt");
        let mut graph = Graph::new("src/bin/day16/day16_data.txt");
        if let Some(res) = graph.astar() {
            graph.astar_visual_plot(true).unwrap();
            dbg!(res);
        }
    }
}