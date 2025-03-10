use aocutils::point::Point;
use num::{abs, ToPrimitive};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ops::Range;
use std::{fs, i32};

// Undirected, weight 1 graph from an array of 'walls'. 'S' & 'E' mark start, end

#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub(crate) g_cost: i32,
    pub(crate) f_est_cost: i32,
    came_from: Option<Point<i32>>,
}

#[derive(Debug)]
pub struct Graph {
    adjacency_list: HashMap<Point<i32>, Vec<Point<i32>>>,
    pub(crate) node_list: HashMap<Point<i32>, Node>,
    // for the visuals
    pub(crate) walls: HashSet<Point<i32>>,
    pub(crate) xrange: Range<i32>,
    pub(crate) yrange: Range<i32>,
    pub(crate) start: Point<i32>,
    pub(crate) end: Point<i32>,
    pub(crate) plot_sequence: usize,
}

impl Graph {
    pub fn new(file: &str) -> Self {
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
                        node_list.insert(
                            pos,
                            Node {
                                g_cost: i32::MAX,
                                f_est_cost: 0, // for A*
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
    pub fn shortest_path(&mut self) -> Option<i32> {
        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost. node_list already init with usize::MAX,
        // came_from None
        // heap contains cost, start, and the previous to start, off west by 1. This
        // forces the Reindeer to be facing east.
        self.node_list.insert(
            self.start,
            Node {
                g_cost: 0,
                f_est_cost: 0,
                came_from: None,
            },
        );

        // cost, position, previous position
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
            if cost > self.node_list[&position].g_cost {
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
                    if next_cost < self.node_list[&node].g_cost {
                        let next = Reverse((next_cost, *node, position));
                        heap.push(next);
                        // Relaxation, we have now found a better way. Update cost and came_from
                        self.node_list.insert(
                            *node,
                            Node {
                                g_cost: next_cost,
                                f_est_cost: 0,
                                came_from: Some(position),
                            },
                        );
                    }
                }
                // uncomment for animation frames
                // self.visual_plot(false).unwrap();
                // self.plot_sequence += 1;
            }
        }
        // Goal not reachable
        None
    }

    pub fn a_star(&mut self) -> Option<i32> {
        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost. node_list already init with i32::MAX, but
        // this is overwritten for start,
        // came_from = None
        // heap contains cost, start, and the previous to start, off west by 1. This
        // forces the Reindeer to be facing east.
        let h = abs(self.end.x - self.start.x) + abs(self.end.y - self.start.y);
        let f = h;
        let g = 0;
        self.node_list.insert(
            self.start,
            Node {
                g_cost: g,
                f_est_cost: f,
                came_from: None,
            },
        );

        // min-q
        // f (est_cost), g (dijkstra cost), position, previous position to West for the 90deg check
        heap.push(Reverse((
            f,
            g,
            self.start,
            Point {
                x: self.start.x - 1,
                y: self.start.y,
            },
        )));

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(Reverse((_heap_f, heap_g, heap_position, heap_previous))) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if heap_position == self.end {
                return Some(heap_g);
            }

            // Important as we may have already found a better way. Can it improve?
            let position_g = self.node_list[&heap_position].g_cost;
            if heap_g > position_g {
                // No, can't improve. Go back to next on heap.
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            if let Some(edges) = self.adjacency_list.get(&heap_position) {
                for next_node in edges {
                    let mut g = heap_g + 1;

                    // Need to account for a 90-degree turn here. Use previous and
                    // next points to check for a change in x && y. Add 1000 to weight g
                    if abs(heap_previous.x - next_node.x) > 0 && abs(heap_previous.y - next_node.y) > 0 {
                        g += 1000;
                    }

                    let h = abs(self.end.x - next_node.x) + abs(self.end.y - next_node.y);
                    let f = g + h;

                    // If so, add it to the frontier and continue
                    if g < self.node_list[&next_node].g_cost {
                        let next = Reverse((f, g, *next_node, heap_position));
                        heap.push(next);

                        // Relaxation, we have now found a better way. Update cost, est_cost and came_from
                        self.node_list.insert(
                            *next_node,
                            Node {
                                g_cost: g,
                                f_est_cost: f,
                                came_from: Some(heap_position),
                            },
                        );
                    }
                }
                // uncomment for animation frames
                // self.a_star_visual_plot(false).unwrap();
                // self.plot_sequence += 1;
            }
        }
        // Goal not reachable
        None
    }

    pub fn show_path(&mut self) -> Vec<Point<i32>> {
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
}

