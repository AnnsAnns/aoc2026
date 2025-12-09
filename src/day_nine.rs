pub const TILES: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
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
}

pub struct DayNine {
    fields: Vec<Coords>,
}

impl DayNine {
    pub fn new(input: &str) -> Self {
        let mut fields = Vec::new();
        for line in input.lines() {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().trim().parse::<usize>().unwrap();
            let y = parts.next().unwrap().trim().parse::<usize>().unwrap();
            fields.push(Coords { x, y });
        }
        DayNine { fields }
    }

    pub fn find_largest_rectangle(&self) -> usize {
        let mut largest_area = 0;
        for (i, field_a) in self.fields.iter().enumerate() {
            for (j, field_b) in self.fields.iter().enumerate() {
                if i != j {
                    let area = field_a.get_rectangle_size_with(field_b);
                    if area > largest_area {
                        println!(
                            "New largest rectangle between ({},{}) and ({},{}): area {}",
                            field_a.x, field_a.y, field_b.x, field_b.y, area
                        );
                        largest_area = area;
                    }
                }
            }
        }
        largest_area
    }
}
