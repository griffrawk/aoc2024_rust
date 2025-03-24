use crate::day16_graph::{Graph, Node};
use aocutils::point::Point;
use num::abs;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Graph {
    // Dijkstra's shortest path algorithm. From BinaryHeap docs, modified to the puzzle.

    // Start at `start` and use `dist` to track the current shortest distance
    // to each node. This implementation isn't memory-efficient as it may leave duplicate
    // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
    // for a simpler implementation.
    pub fn floyd(&mut self) -> Option<i32> {
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
                seen: false,
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
                    // Need to account for a 90-degree turn here. Use previous and
                    // next points to check for a change in x and y
                    let next_cost = if abs(previous.x - node.x) > 0 && abs(previous.y - node.y) > 0
                    {
                        cost + 1001
                    } else {
                        cost + 1
                    };
                    // If so, add it to the frontier and continue
                    // todo for 16.2 consider what to do if the cost is equal as well as lt
                    //  this would be true for alternate paths of the same cost. but it feels tricky
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
                                seen: false,
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
    
}
