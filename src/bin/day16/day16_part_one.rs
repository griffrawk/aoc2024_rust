use std::cmp::Reverse;
use aocutils::point::Point;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::ops::Range;
use num::{abs, ToPrimitive};
use plotters::coord::types::RangedCoordi32;
use plotters::prelude::*;
use plotters::prelude::full_palette::{CYAN_200, CYAN_500};

const OUTPUT_FILENAME: &str = "src/bin/day16/output/day16_gen";

// Undirected, weight 1 graph from an array of 'walls'. 'S' & 'E' mark start, end

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<Point<i32>, Vec<Point<i32>>>,
    node_list: HashMap<Point<i32>, (usize, Option<Point<i32>>)>,
    // for the visuals
    walls: HashSet<Point<i32>>,
    xrange: Range<i32>,
    yrange: Range<i32>,
    start: Point<i32>,
    end: Point<i32>,
    plot_sequence: usize,
}

impl Graph {
    fn new(file: &str) -> Self {
        let mut xrange = Range::default();
        let mut yrange = Range::default();
        let mut start = Point::default();
        let mut end = Point::default();
        let mut adjacency_list = HashMap::new();
        let mut node_list = HashMap::new();
        let mut walls = HashSet::new();
        let mut maze: Vec<Vec<char>> = Vec::new();
        
        // process the grid into adjacency_list & node_list
        for (y, row) in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
            .enumerate()
        {
            let iy = y as i32;
            // convert from array of &str into 2d array of chars, so we can 
            // perform cardinal point lookups on all the data in memory
            maze.push(row.chars().collect());
            xrange = 0..row.len() as i32;
            yrange = 0..iy + 1;
        }
        
        for (y, row) in maze.iter().enumerate() {
            let iy = y as i32;
            for (x, c) in row.iter().enumerate() {
                let ix = x as i32;
                let pos = Point { x: ix, y: iy };
                match c {
                    '.' | 'S' | 'E' => {
                        // record start and end coords
                        if *c == 'S' {
                            start = pos;
                        }
                        if *c == 'E' {
                            end = pos;
                        }
                        // process edges from cardinal points where not a wall
                        let mut edges: Vec<Point<i32>> = Vec::new();
                        for cardinal in pos.cardinal_points() {
                            if xrange.contains(&cardinal.x) && yrange.contains(&cardinal.y) {
                                let n = maze[cardinal.y.to_usize().unwrap()][cardinal.x.to_usize().unwrap()];
                                match n {
                                    '.' | 'S' | 'E' => edges.push(cardinal),
                                    _ => (),
                                }
                            }
                        }
                        // v: (cost (initially usize::MAX), came_from node (for track-back
                        // at the end) to get path. not to be confused with previous node in the
                        // priority queue which is just to track previous for turn detection)
                        node_list.insert(pos, (usize::MAX, None));
                        adjacency_list.insert(pos, edges);
                    },
                    _ => {
                        // store walls for the visuals
                        walls.insert(pos);
                    },
                }
            }
        }
        Self {
            adjacency_list,
            node_list,
            walls,
            xrange,
            yrange,
            start,
            end,
            plot_sequence: 0,
        }
    }

    // Dijkstra's shortest path algorithm. From BinaryHeap docs, modified to the puzzle.

    // Start at `start` and use `dist` to track the current shortest distance
    // to each node. This implementation isn't memory-efficient as it may leave duplicate
    // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
    // for a simpler implementation.
    fn shortest_path(&mut self) -> Option<usize> {
        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost. node_list already init with usize::MAX, 
        // came_from None
        // heap contains cost, start, and the previous to start, off west by 1. This
        // forces the Reindeer to be facing east.
        self.node_list.insert(self.start, (0, None));
        heap.push(Reverse((0, self.start, Point { x: self.start.x - 1, y: self.start.y})));

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(Reverse((cost, position, previous))) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == self.end { return Some(cost); }

            // Important as we may have already found a better way
            if cost > self.node_list[&position].0 { continue; }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            if let Some(edges) = self.adjacency_list.get(&position) {
                for node in edges {
                    let mut next_cost = cost + 1;
                    // Need to account for a 90 degree turn here. Use previous and
                    // next points to check for a change in x and y
                    if abs(previous.x - node.x) > 0 && abs(previous.y - node.y) > 0 {
                        next_cost += 1000;
                    }
                    // If so, add it to the frontier and continue
                    if next_cost < self.node_list[&node].0 {
                        let next = Reverse((next_cost, *node, position));
                        heap.push(next);
                        // Relaxation, we have now found a better way. Update cost and came_from
                        self.node_list.insert(*node, (next_cost, Some(position)));
                    }
                }
                self.visual_plot(false).unwrap();
                self.plot_sequence += 1;
            }
        }
        // Goal not reachable
        None
    }

    fn show_path(&mut self) -> Vec<Point<i32>> {
        // Assemble a list of path nodes from the end to start, and referring to
        // each node's came_from to find previous node
        let mut res = Vec::new();
        let mut next = self.node_list[&self.end].1.unwrap();
        while next != self.start {
            res.push(next);
            next = self.node_list[&next].1.unwrap();
        }
        // finally push start on list
        res.push(next);
        res
    }

    fn visual_plot(&mut self, last: bool) -> Result<(), Box<dyn std::error::Error>> {
        let out = format!("{}_{:06}{}",
                          OUTPUT_FILENAME,
                          self.plot_sequence,
                          ".png");
        let root_area = BitMapBackend::new(&out, (1024, 1024)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();
        let end_x = self.xrange.end;
        let end_y = self.yrange.end;
        let root_area = root_area.apply_coord_spec(
            Cartesian2d::<RangedCoordi32, RangedCoordi32>::new(0..end_x, 0..end_y, (0..1024, 0..1024)),
        );

        // todo This could probably ben done better, but...
        let block_side = 1024 / self.yrange.end + 1;
        let wall_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (block_side, block_side)], ShapeStyle::from(&BLUE).filled());
        };
        let path_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (block_side, block_side)], ShapeStyle::from(&MAGENTA).filled());
        };
        let start_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (block_side, block_side)], ShapeStyle::from(&RED).filled());
        };
        let end_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (block_side, block_side)], ShapeStyle::from(&GREEN).filled());
        };
        let node_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (block_side, block_side)], ShapeStyle::from(&CYAN_200).filled());
        };

        for pos in self.walls.clone() {
            root_area.draw(&wall_block(pos.x, pos.y))?;
        }
        
        for (pos, (cost, _)) in self.node_list.clone() {
            if cost < usize::MAX {
                root_area.draw(&node_block(pos.x, pos.y))?;
            }
        }
        
        if last {       // future flag for drawing the path at the end
            for pos in self.show_path() {
                root_area.draw(&path_block(pos.x, pos.y))?;
            }
        }
        root_area.draw(&start_block(self.start.x, self.start.y))?;
        root_area.draw(&end_block(self.end.x, self.end.y))?;

        root_area.present()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::Graph;

    #[test]
    fn test_part_one_test_a() {
        let mut graph = Graph::new("src/bin/day16/day16_test_a.txt");
        let res = graph.shortest_path();
        graph.visual_plot(true).unwrap();
        assert_eq!(res, Some(7036));
    }

    #[test]
    fn test_part_one_test_b() {
        let mut graph = Graph::new("src/bin/day16/day16_test_b.txt");
        let res = graph.shortest_path();
        graph.visual_plot(true).unwrap();
        assert_eq!(res, Some(11048));
    }

#[test]
    fn test_part_one_data() {
        // to debug cwd when I'm trying to find the png
        println!("Current directory {}", env::current_dir().unwrap().display());

        let mut graph = Graph::new("src/bin/day16/day16_data.txt");
        let res = graph.shortest_path();
        graph.visual_plot(true).unwrap();
        assert_eq!(res, Some(107512));
    }

#[test]
    fn test_part_one_joost() {
        let mut graph = Graph::new("src/bin/day16/joost.txt");
        let res = graph.shortest_path();
        graph.visual_plot(true).unwrap();
        assert_eq!(res, Some(82464));
    }
}