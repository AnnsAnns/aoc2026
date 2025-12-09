use super::*;

/// Represents the bounds and corners of a rectangle
pub struct Rectangle {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl Rectangle {
    /// Construct a rectangle from two corner points
    pub fn from_coords(a: &Coords, b: &Coords) -> Self {
        let left = a.get_x().min(b.get_x());
        let right = a.get_x().max(b.get_x());
        let top = a.get_y().min(b.get_y());
        let bottom = a.get_y().max(b.get_y());

        Rectangle {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Get the top-left corner coordinates
    pub fn get_top_left(&self) -> Coords {
        Coords::new(self.left, self.top)
    }

    /// Get the top-right corner coordinates
    pub fn get_top_right(&self) -> Coords {
        Coords::new(self.right, self.top)
    }

    /// Get the bottom-left corner coordinates
    pub fn get_bottom_left(&self) -> Coords {
        Coords::new(self.left, self.bottom)
    }

    /// Get the bottom-right corner coordinates
    pub fn get_bottom_right(&self) -> Coords {
        Coords::new(self.right, self.bottom)
    }

    /// Check if any point from the given list lies strictly inside this rectangle
    pub fn contains_any_point(&self, points: &Vec<Coords>) -> bool {
        points.iter().any(|c| {
            c.get_x() > self.left
                && c.get_x() < self.right
                && c.get_y() > self.top
                && c.get_y() < self.bottom
        })
    }

    /// Check if all four corners of the rectangle are inside the polygon
    /// otherwise we violate the part 2 rule
    pub fn all_corners_inside_polygon(
        &self,
        horizontal_lines: &Vec<Polygon>,
        vertical_lines: &Vec<Polygon>,
    ) -> bool {
        self.get_top_left()
            .is_inside(horizontal_lines, vertical_lines)
            && self
                .get_top_right()
                .is_inside(horizontal_lines, vertical_lines)
            && self
                .get_bottom_left()
                .is_inside(horizontal_lines, vertical_lines)
            && self
                .get_bottom_right()
                .is_inside(horizontal_lines, vertical_lines)
    }

    /// Check if any of the rectangle's edges cross any of the polygon's edges
    pub fn edges_cross_polygon(
        &self,
        horizontal_lines: &Vec<Polygon>,
        vertical_lines: &Vec<Polygon>,
    ) -> bool {
        let top_edge = Polygon::new(self.get_top_left(), self.get_top_right());
        let bottom_edge = Polygon::new(self.get_bottom_left(), self.get_bottom_right());
        let left_edge = Polygon::new(self.get_top_left(), self.get_bottom_left());
        let right_edge = Polygon::new(self.get_top_right(), self.get_bottom_right());

        // Check if vertical polygon edges cross horizontal rectangle edges
        if vertical_lines
            .iter()
            .any(|v| v.crosses_horizontal(&top_edge) || v.crosses_horizontal(&bottom_edge))
        {
            return true;
        }

        // Check if horizontal polygon edges cross vertical rectangle edges
        if horizontal_lines
            .iter()
            .any(|h| h.crosses_vertical(&left_edge) || h.crosses_vertical(&right_edge))
        {
            return true;
        }

        false
    }
}
