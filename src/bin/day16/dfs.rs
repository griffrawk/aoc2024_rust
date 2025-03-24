use aocutils::point::Point;
use crate::day16_graph::{Graph, Node};

impl Graph {
    // Recursive DFS without path
    pub fn rec_dfs(&mut self) -> i32 {
        self.dfs_rec(self.start);
        0
    }
    
    fn dfs_rec(&mut self, next: Point<i32>) {
        if next == self.end {return;}   
        self.node_list.entry(next).and_modify(|n| n.seen = true);  
        
        self.dijkstra_plot(false);
        self.plot_sequence += 1;
        
        for edge in self.adjacency_list[&next].clone() {
            if !self.node_list[&edge].seen {
                self.dfs_rec(edge);
            }
        }
    }
    
    // Recursive DFS with path
    pub fn rec_dfs_path(&mut self) -> Vec<Point<i32>> {
        let mut last: Vec<Point<i32>> = Vec::new();
        self.dfs_path_rec(self.start, &mut last);
        last
    }
    
    fn dfs_path_rec(&mut self, next: Point<i32>, last: &mut Vec<Point<i32>>) {
        if next == self.end {return;}

        self.dijkstra_plot(false);
        self.plot_sequence += 1;

        self.node_list.entry(next).and_modify(|n| n.seen = true);
        for edge in self.adjacency_list[&next].clone() {
            if !self.node_list[&edge].seen {
                last.push(edge);
                self.dfs_path_rec(edge, last);
            }
        }
    }
}
