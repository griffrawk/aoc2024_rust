use std::cmp::{Ordering, Reverse};
use aocutils::point::Point;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::ops::Range;
use num::{abs, ToPrimitive};
use plotters::coord::types::RangedCoordi32;
use plotters::prelude::*;

const OUTPUT_FILENAME: &str = "src/bin/day16/output/day16_gen";

// Undirected, weight 1 graph from an array of 'walls'. 'S' & 'E' mark start, end

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<Point<usize>, Vec<Point<usize>>>,
    node_list: HashMap<Point<usize>, (usize, Option<Point<usize>>)>,
    // for the visuals
    walls: HashSet<Point<usize>>,
    xrange: Range<usize>,
    yrange: Range<usize>,
    start: Point<usize>,
    end: Point<usize>,
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
        for (y, row) in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
            .enumerate()
        {
            maze.push(row.chars().collect());
            xrange = 0..row.len();
            yrange = 0..y + 1;
        }
        for (y, row) in maze.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    '.' | 'S' | 'E' => {
                        if *c == 'S' {
                            start.x = x;
                            start.y = y;
                        }
                        if *c == 'E' {
                            end.x = x;
                            end.y = y;
                        }
                        let mut edges: Vec<Point<usize>> = Vec::new();
                        // let mut edges: Vec<(Point<usize>, EdgeData)> = Vec::new();
                        for cardinal in (Point { x: x as i32, y: y as i32 }).cardinal_points() {
                            let cardinal = Point { x: cardinal.x.to_usize().unwrap(), y: cardinal.y.to_usize().unwrap() };
                            if xrange.contains(&cardinal.x) && yrange.contains(&cardinal.y) {
                                let n = maze[cardinal.y][cardinal.x];
                                match n {
                                    '.' | 'S' | 'E' => edges.push(cardinal),
                                    // '.' | 'S' | 'E' => edges.push((cardinal, EdgeData::Weight(1))),
                                    _ => (),
                                }
                            }
                        }
                        let node = Point { x, y };
                        node_list.insert(node, (usize::MAX, None));
                        adjacency_list.insert(node, edges);
                    },
                    _ => {
                        walls.insert(Point { x, y });
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
        }
    }

    // Dijkstra's shortest path algorithm.

    // Start at `start` and use `dist` to track the current shortest distance
    // to each node. This implementation isn't memory-efficient as it may leave duplicate
    // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
    // for a simpler implementation.
    fn shortest_path(&mut self) -> Option<usize> {
        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost. node_list already init with usize::MAX, came_from None
        self.node_list.insert(self.start, (0, None));
        heap.push(Reverse((0, self.start)));

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(Reverse((cost, position))) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == self.end { return Some(cost); }

            // Important as we may have already found a better way
            if cost > self.node_list[&position].0 { continue; }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            if let Some(edges) = self.adjacency_list.get(&position) {
                for node in edges {
                    let next_cost = cost + 1;
                    let next = Reverse((next_cost, *node));
                    // If so, add it to the frontier and continue
                    if next_cost < self.node_list[&node].0 {
                        heap.push(next);
                        // Relaxation, we have now found a better way. Update cost and came_from
                        self.node_list.insert(*node, (next_cost, Some(position)));
                    }
                }
            }
        }
        // Goal not reachable
        None
    }

    fn show_path(&mut self) -> Vec<Point<usize>> {
        let mut res = Vec::new();
        let mut next = self.node_list[&self.end].1.unwrap();
        while next != self.start {
            res.push(next);
            next = self.node_list[&next].1.unwrap();
        }
        res
    }

    fn visual_plot(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let out = format!("{}{}",
                          OUTPUT_FILENAME,
                          ".png");
        let root_area = BitMapBackend::new(&out, (1024, 1024)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();
        let root_area = root_area.apply_coord_spec(
            Cartesian2d::<RangedCoordi32, RangedCoordi32>::new(0..15, 0..15, (0..1024, 0..1024)),
        );

        let wall_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (40, 40)], ShapeStyle::from(&BLUE).filled());
        };
        let path_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (40, 40)], ShapeStyle::from(&CYAN).filled());
        };
        let start_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (40, 40)], ShapeStyle::from(&RED).filled());
        };
        let end_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0, 0), (40, 40)], ShapeStyle::from(&GREEN).filled());
        };

        for pos in self.walls.clone() {
            root_area.draw(&wall_block(pos.x as i32, pos.y as i32))?;

        }
        for pos in self.show_path() {
            root_area.draw(&path_block(pos.x as i32, pos.y as i32))?;
        }
        root_area.draw(&start_block(self.start.x as i32, self.start.y as i32))?;
        root_area.draw(&end_block(self.end.x as i32, self.end.y as i32))?;

        root_area.present()?;
        Ok(())
    }
}

fn heuristic_distance(pos: Point<usize>, end: Point<usize>) -> usize {
    (abs(end.x as i32 - pos.x as i32)
        + abs(end.y as i32 - pos.y as i32))
        .to_usize()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::Graph;

    #[test]
    fn test_shortest_path() {
        println!("Current directory {}", env::current_dir().unwrap().display());

        let mut graph = Graph::new("src/bin/day16/test_graph.txt");
        assert_eq!(graph.shortest_path(), Some(28));
        graph.visual_plot().unwrap();
    }
}