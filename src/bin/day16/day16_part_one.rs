use aocutils::point::Point;
use num::{abs, ToPrimitive};
use plotters::coord::types::RangedCoordi32;
use plotters::prelude::full_palette::CYAN_200;
use plotters::prelude::*;
use std::cmp::{max, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::ops::Range;
use plotters::style::full_palette::GREY;

const OUTPUT_FILENAME: &str = "src/bin/day16/output/day16_gen";

// Undirected, weight 1 graph from an array of 'walls'. 'S' & 'E' mark start, end

#[derive(Debug, Clone)]
struct Node {
    cost: usize,
    came_from: Option<Point<i32>>,
}

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<Point<i32>, Vec<Point<i32>>>,
    node_list: HashMap<Point<i32>, Node>,
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
                                let n = maze[cardinal.y.to_usize().unwrap()]
                                    [cardinal.x.to_usize().unwrap()];
                                match n {
                                    '.' | 'S' | 'E' => edges.push(cardinal),
                                    _ => (),
                                }
                            }
                        }
                        // v: (cost (initially usize::MAX), came_from node (for track-back
                        // at the end) to get path. not to be confused with previous node in the
                        // priority queue which is just to track previous for turn detection)
                        node_list.insert(
                            pos,
                            Node {
                                cost: usize::MAX,
                                came_from: None,
                            },
                        );
                        adjacency_list.insert(pos, edges);
                    }
                    _ => {
                        // store walls for the visuals
                        walls.insert(pos);
                    }
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
        self.node_list.insert(
            self.start,
            Node {
                cost: 0,
                came_from: None,
            },
        );
        heap.push(Reverse((
            0,
            self.start,
            Point {
                x: self.start.x - 1,
                y: self.start.y,
            },
        )));

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(Reverse((cost, position, previous))) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == self.end {
                return Some(cost);
            }

            // Important as we may have already found a better way
            if cost > self.node_list[&position].cost {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            if let Some(edges) = self.adjacency_list.get(&position) {
                for node in edges {
                    let mut next_cost = cost + 1;
                    // Need to account for a 90-degree turn here. Use previous and
                    // next points to check for a change in x and y
                    if abs(previous.x - node.x) > 0 && abs(previous.y - node.y) > 0 {
                        next_cost += 1000;
                    }
                    // If so, add it to the frontier and continue
                    if next_cost < self.node_list[&node].cost {
                        let next = Reverse((next_cost, *node, position));
                        heap.push(next);
                        // Relaxation, we have now found a better way. Update cost and came_from
                        self.node_list.insert(
                            *node,
                            Node {
                                cost: next_cost,
                                came_from: Some(position),
                            },
                        );
                    }
                }
                // self.visual_plot(false).unwrap();
                // self.plot_sequence += 1;
            }
        }
        // Goal not reachable
        None
    }

    fn show_path(&mut self) -> Vec<Point<i32>> {
        // Assemble a list of path nodes from the end to start, and referring to
        // each node's came_from to find previous node
        let mut res = Vec::new();
        let mut next = self.node_list[&self.end].came_from.unwrap();
        while next != self.start {
            res.push(next);
            next = self.node_list[&next].came_from.unwrap();
        }
        // finally push start on list
        res.push(next);
        res
    }

    fn visual_plot(&mut self, last: bool) -> Result<(), Box<dyn std::error::Error>> {
        let out = format!("{}_{:06}{}", OUTPUT_FILENAME, self.plot_sequence, ".png");
        let root_area = BitMapBackend::new(&out, (1024, 1024)).into_drawing_area();

        root_area.fill(&WHITE).unwrap();
        let end_x = self.xrange.end;
        let end_y = self.yrange.end;
        let root_area =
            root_area.apply_coord_spec(Cartesian2d::<RangedCoordi32, RangedCoordi32>::new(
                0..end_x,
                0..end_y,
                (0..1024, 0..1024),
            ));

        let block_side = 1024 / self.yrange.end + 1;
        let node_block = |x: i32, y: i32, max_cost: usize, cost: usize| {
            return EmptyElement::at((x, y))
                + Rectangle::new(
                    [(0, 0), (block_side, block_side)],
                    ShapeStyle::from(&MandelbrotHSL::get_color_normalized(cost as f64, 0.0, max_cost as f64)).filled(),
                );
        };
        let block = |x: i32, y: i32, c: RGBColor| {
            return EmptyElement::at((x, y))
                + Rectangle::new(
                [(0, 0), (block_side, block_side)],
                ShapeStyle::from(&c).filled(),
            );
        };

        for pos in self.walls.clone() {
            root_area.draw(&block(pos.x, pos.y, GREY))?;
        }

        // todo revisit this for animation maybe. not convinced the calc is correct
        //  as different to end cost as below, when found at the last frame
        // let max_cost = 
        //     .node_list
        //     .values()
        //     .fold(0, |acc, node| max(acc, node.cost));
        
        let end_cost = self.node_list[&self.end].cost;
        
        for (pos, node) in self.node_list.clone() {
            if node.cost < usize::MAX {
                root_area.draw(&node_block(pos.x, pos.y, end_cost, node.cost))?;
            }
        }

        if last {
            for pos in self.show_path() {
                root_area.draw(&block(pos.x, pos.y, BLACK))?;
            }
        }
        root_area.draw(&block(self.start.x, self.start.y, RED))?;
        root_area.draw(&block(self.end.x, self.end.y, GREEN))?;

        root_area.present()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use std::env;

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
        println!(
            "Current directory {}",
            env::current_dir().unwrap().display()
        );

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
