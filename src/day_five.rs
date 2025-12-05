use std::collections::HashSet;

pub const GRID_DATA: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

pub const ANSWER: usize = 3;

pub struct DayFive {
    valid_values: HashSet<usize>,
    available_values: HashSet<usize>,
    total_valid: usize,
}

impl DayFive {
    pub fn new(line: &str) -> Self {
        let mut sections = line.split("\n\n");
        let ranges_part = sections.next().unwrap();
        let values_part = sections.next().unwrap();

        let mut valid_values = HashSet::new();

        for range_line in ranges_part.lines() {
            let mut parts = range_line.split('-');
            let start = parts.next().unwrap().parse::<usize>().unwrap();
            let end = parts.next().unwrap().parse::<usize>().unwrap();

            for value in start..=end {
                valid_values.insert(value);
            }
        }

        let available_values: HashSet<usize> = values_part
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        println!(
            "Valid values: {:?}, Available values: {:?}",
            valid_values, available_values
        );

        DayFive {
            valid_values,
            available_values,
            total_valid: 0,
        }
    }

    pub fn calc_total(&mut self) {
        for ingredient in self.available_values.iter() {
            if self.valid_values.contains(ingredient) {
                self.total_valid += 1;
            }
        }
    }

    pub fn get_total_valid(&self) -> usize {
        self.total_valid
    }
}
