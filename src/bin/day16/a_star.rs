use crate::day16_graph::{Graph, Node};
use aocutils::point::Point;
use num::abs;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Graph {
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
                seen: false,
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
        while let Some(Reverse((_prio_q_f, prio_q_g, prio_q_pos, prio_q_prev))) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if prio_q_pos == self.end {
                return Some(prio_q_g);
            }

            // Important as we may have already found a better way. Can it improve?
            let position_g = self.node_list[&prio_q_pos].g_cost;
            if prio_q_g > position_g {
                // No, can't improve. Go back to next on heap.
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            if let Some(edges) = self.adjacency_list.get(&prio_q_pos) {
                for next_node in edges {
                    // Need to account for a 90-degree turn here. Use previous and
                    // next points to check for a change in x && y. Add 1000 to weight g
                    let g = if abs(prio_q_prev.x - next_node.x) > 0
                        && abs(prio_q_prev.y - next_node.y) > 0
                    {
                        prio_q_g + 1001
                    } else {
                        prio_q_g + 1
                    };

                    let h = abs(self.end.x - next_node.x) + abs(self.end.y - next_node.y);
                    let f = g + h;

                    // If so, add it to the frontier and continue
                    
                    if g < self.node_list[&next_node].g_cost {
                        let next = Reverse((f, g, *next_node, prio_q_pos));
                        heap.push(next);

                        // Relaxation, we have now found a better way. Update cost, est_cost and came_from
                        self.node_list.insert(
                            *next_node,
                            Node {
                                g_cost: g,
                                f_est_cost: f,
                                came_from: Some(prio_q_pos),
                                seen: false,
                            },
                        );
                    }
                }
                // run as --profile release for animations
                if !cfg!(debug_assertions) {
                    self.a_star_visual_plot(120000,false).unwrap();
                    self.plot_sequence += 1;
                }
            }
        }
        // Goal not reachable
        None
    }

    pub fn a_star_all(&mut self) -> Option<i32> {
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
                seen: false,
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
        while let Some(Reverse((_prio_q_f, prio_q_g, prio_q_pos, prio_q_prev))) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if prio_q_pos == self.end {
                return Some(prio_q_g);
            }

            // Important as we may have already found a better way. Can it improve?
            let position_g = self.node_list[&prio_q_pos].g_cost;
            if prio_q_g > position_g {
                // No, can't improve. Go back to next on heap.
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            if let Some(edges) = self.adjacency_list.get(&prio_q_pos) {
                for next_node in edges {
                    // Need to account for a 90-degree turn here. Use previous and
                    // next points to check for a change in x && y. Add 1000 to weight g
                    let g = if abs(prio_q_prev.x - next_node.x) > 0
                        && abs(prio_q_prev.y - next_node.y) > 0
                    {
                        prio_q_g + 1001
                    } else {
                        prio_q_g + 1
                    };

                    let h = abs(self.end.x - next_node.x) + abs(self.end.y - next_node.y);
                    let f = g + h;

                    // If so, add it to the frontier and continue

                    if g <= self.node_list[&next_node].g_cost {
                        let next = Reverse((f, g, *next_node, prio_q_pos));
                        heap.push(next);

                        // Relaxation, we have now found a better way. Update cost, est_cost and came_from
                        self.node_list.insert(
                            *next_node,
                            Node {
                                g_cost: g,
                                f_est_cost: f,
                                came_from: Some(prio_q_pos),
                                seen: false,
                            },
                        );
                    }
                }
                // run as --profile release for animations
                if !cfg!(debug_assertions) {
                    self.a_star_visual_plot(120000,false).unwrap();
                    self.plot_sequence += 1;
                }
            }
        }
        // Goal not reachable
        None
    }
}
