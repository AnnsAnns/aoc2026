use super::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    /// Calculate the size of a rectangle between two coordinates
    pub fn get_rectangle_size_with(&self, other: &Coords) -> usize {
        let width = if self.x > other.x {
            self.x - other.x + 1
        } else {
            other.x - self.x + 1
        };
        let height = if self.y > other.y {
            self.y - other.y + 1
        } else {
            other.y - self.y + 1
        };
        width * height
    }

    /// Check if this point lies on a horizontal edge
    fn is_on_horizontal_edge(&self, horizontal_lines: &[Polygon]) -> bool {
        let i = horizontal_lines.partition_point(|e| e.min().y < self.y);
        let mut idx = i;
        while idx < horizontal_lines.len() && horizontal_lines[idx].min().y == self.y {
            if (horizontal_lines[idx].min().x..=horizontal_lines[idx].max().x).contains(&self.x) {
                return true;
            }
            idx += 1;
        }
        false
    }

    /// Check if this point lies on a vertical edge
    fn is_on_vertical_edge(&self, vertical_lines: &[Polygon]) -> bool {
        let j = vertical_lines.partition_point(|e| e.min().x < self.x);
        let mut idx = j;
        while idx < vertical_lines.len() && vertical_lines[idx].min().x == self.x {
            if (vertical_lines[idx].min().y..=vertical_lines[idx].max().y).contains(&self.y) {
                return true;
            }
            idx += 1;
        }
        false
    }

    /// Update edge counts when hitting a corner of a horizontal edge
    fn update_corner_counts(
        &self,
        edge: &Polygon,
        left_edges: &mut usize,
        right_edges: &mut usize,
        crossed_edges: &mut usize,
    ) {
        if edge.max().x > self.x {
            *right_edges += 1;
        } else {
            *left_edges += 1;
        }

        if *right_edges == *left_edges {
            // We've crossed as many right-pointing as left-pointing
            // edges. Increase the total number of edges crossed.
            *crossed_edges += 1;
        }
    }

    /// Process a horizontal edge crossing and update counters
    fn process_horizontal_edge_crossing(
        &self,
        edge: &Polygon,
        left_edges: &mut usize,
        right_edges: &mut usize,
        crossed_edges: &mut usize,
    ) {
        if self.x == edge.min().x {
            // Hit a corner at the minimum x coordinate
            self.update_corner_counts(edge, left_edges, right_edges, crossed_edges);
        } else if self.x == edge.max().x {
            // Hit a corner at the maximum x coordinate
            self.update_corner_counts(edge, left_edges, right_edges, crossed_edges);
        } else {
            // Hit the inside of the edge (not a corner)
            *crossed_edges += 1;
        }
    }

    /// Count how many edges are crossed by a ray cast downward from this point
    fn count_crossed_edges(&self, horizontal_lines: &[Polygon], start_index: usize) -> usize {
        let mut left_edges = 0;
        let mut right_edges = 0;
        let mut crossed_edges = 0;

        for edge in horizontal_lines.iter().skip(start_index) {
            if (edge.min().x..=edge.max().x).contains(&self.x) {
                self.process_horizontal_edge_crossing(
                    edge,
                    &mut left_edges,
                    &mut right_edges,
                    &mut crossed_edges,
                );
            }
        }

        crossed_edges
    }

    /// Check if this point lies inside the polygon made up of the horizontal edges
    /// `hedges` and the vertical edges `vedges`
    pub fn is_inside(&self, horizontal_lines: &[Polygon], vertical_lines: &[Polygon]) -> bool {
        // First check if the point lies directly on any edge
        if self.is_on_horizontal_edge(horizontal_lines) {
            return true;
        }

        if self.is_on_vertical_edge(vertical_lines) {
            return true;
        }

        // Find the starting index for horizontal lines below this point
        let start_index = horizontal_lines.partition_point(|e| e.min().y < self.y);
        if start_index == horizontal_lines.len() {
            return false;
        }

        let crossed_edges = self.count_crossed_edges(horizontal_lines, start_index);

        // We're inside the polygon if we've crossed an odd number of edges
        crossed_edges % 2 != 0
    }
}
