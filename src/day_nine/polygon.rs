use super::*;

#[derive(Clone, Copy)]
pub struct Polygon {
    a: Coords,
    b: Coords,
}

impl Polygon {
    /// Create a new polygon edge from two coordinates
    /// I realized that Polygon might be incorrect too late,
    /// so we now keep it :P
    pub fn new(a: Coords, b: Coords) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    /// Get the minimum coordinate (top-left)
    pub fn min(&self) -> Coords {
        if self.a < self.b { self.a } else { self.b }
    }

    /// Get the maximum coordinate (bottom-right)
    pub fn max(&self) -> Coords {
        if self.a < self.b { self.b } else { self.a }
    }

    /// Check if this vertical edge crosses a horizontal edge
    pub fn crosses_horizontal(&self, horiz: &Polygon) -> bool {
        let self_min = self.min();
        let self_max = self.max();
        let horiz_min = horiz.min();
        let horiz_max = horiz.max();

        self_min.get_x() > horiz_min.get_x()
            && self_min.get_x() < horiz_max.get_x()
            && self_min.get_y() < horiz_min.get_y()
            && self_max.get_y() > horiz_min.get_y()
    }

    /// Check if this horizontal edge crosses a vertical edge
    pub fn crosses_vertical(&self, vert: &Polygon) -> bool {
        let self_min = self.min();
        let self_max = self.max();
        let vert_min = vert.min();
        let vert_max = vert.max();

        vert_min.get_x() > self_min.get_x()
            && vert_min.get_x() < self_max.get_x()
            && vert_min.get_y() < self_min.get_y()
            && vert_max.get_y() > self_min.get_y()
    }
}
