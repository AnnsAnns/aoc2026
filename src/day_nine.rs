pub const TILES: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

mod coords;
mod polygon;
mod rectangle;

use self::coords::Coords;
use self::polygon::Polygon;
use self::rectangle::Rectangle;

#[derive(Clone)]
pub struct DayNine {
    fields: Vec<Coords>,
    horizontal_lines: Vec<Polygon>,
    vertical_lines: Vec<Polygon>,
}

impl DayNine {
    pub fn new(input: &str) -> Self {
        let mut fields = Vec::new();
        for line in input.lines() {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().trim().parse::<usize>().unwrap();
            let y = parts.next().unwrap().trim().parse::<usize>().unwrap();
            fields.push(Coords::new(x, y));
        }

        let mut horizontal_lines = Vec::new();
        let mut vertical_lines = Vec::new();

        // Each field is also a polygon with its before and after field
        // (Wraps around at 0 / max for connection
        for i in 0..fields.len() {
            let p1 = fields[i];
            let p2 = fields[(i + 1) % fields.len()];

            let polygon = Polygon::new(p1, p2);

            // Build horizontal and vertical edge lists
            if p1.get_y() == p2.get_y() {
                horizontal_lines.push(polygon);
            } else {
                vertical_lines.push(polygon);
            }
        }

        // We need to sort them here since our algorithm
        // expects the lines to be in order
        horizontal_lines.sort_unstable_by_key(|e| e.min().get_y());
        vertical_lines.sort_unstable_by_key(|e| e.min().get_x());

        DayNine {
            fields,
            horizontal_lines,
            vertical_lines,
        }
    }

    /// Part 1: Find the largest rectangle between any two field points
    /// (Alias, we simply brute-force all pairs and calculate the area)
    pub fn find_largest_rectangle(&self) -> usize {
        let mut largest_area = 0;
        for (index_a, field_a) in self.fields.iter().enumerate() {
            for (index_b, field_b) in self.fields.iter().enumerate() {
                if index_a < index_b {
                    let current = field_a.get_rectangle_size_with(field_b);
                    if current > largest_area {
                        // println!(
                        //     "New largest rectangle between ({},{}) and ({},{}): area {}",
                        //     field_a.get_x(),
                        //     field_a.get_y(),
                        //     field_b.get_x(),
                        //     field_b.get_y(),
                        //     ar
                        // );
                        largest_area = current;
                    }
                }
            }
        }
        largest_area
    }

    /// Check if a rectangle is valid (fully inside polygon with no violations)
    fn is_valid_rectangle(&self, rect: &Rectangle) -> bool {
        // Check if any polygon points lie inside the rectangle
        // This saves us quite a bit of computation later since the next methods
        // are really freaking expensive (At LEAST O(n³) from an educated guess without
        // counting it)
        if rect.contains_any_point(&self.fields) {
            return false;
        }

        // Check if all rectangle corners are inside the polygon
        if !rect.all_corners_inside_polygon(&self.horizontal_lines, &self.vertical_lines) {
            return false;
        }

        // Check if rectangle edges cross polygon edges
        if rect.edges_cross_polygon(&self.horizontal_lines, &self.vertical_lines) {
            return false;
        }

        true
    }

    /// Part 2: Find the largest rectangle fully inside the polygon
    /// The implementation spans multiple files don't be fooled
    pub fn find_largest_rectangle_inside_polygon(&self) -> usize {
        let mut max = 0;

        for a in self.fields.iter() {
            for b in self.fields.iter() {
                let area = a.get_rectangle_size_with(b);

                if area > max {
                    // Construct the rectangle from two field points
                    let rect = Rectangle::from_coords(a, b);

                    // Validate the rectangle against all constraints
                    if self.is_valid_rectangle(&rect) {
                        max = max.max(area);
                    }
                }
            }
        }

        max
    }
}
