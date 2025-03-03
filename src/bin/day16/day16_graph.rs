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
    pub(crate) came_from: Option<Point<i32>>,
    pub(crate) seen: bool,
}

#[derive(Debug)]
pub struct Graph {
    pub adjacency_list: HashMap<Point<i32>, Vec<Point<i32>>>,
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
                                seen: false
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
