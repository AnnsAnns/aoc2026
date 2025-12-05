use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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
    valid_values: Vec<(usize, usize)>,
    available_values: Vec<usize>,
    total_valid: usize,
    covered_ranges: HashMap<usize, usize>,
}

impl DayFive {
    pub fn new(line: &str) -> Self {
        let mut sections = line.split("\n\n");
        let ranges_part = sections.next().unwrap();
        let values_part = sections.next().unwrap();

        let mut valid_values = Vec::new();

        for range_line in ranges_part.lines() {
            let mut parts = range_line.split('-');
            let start = parts.next().unwrap().parse::<usize>().unwrap();
            let end = parts.next().unwrap().parse::<usize>().unwrap();

            valid_values.push((start, end));
        }

        let available_values: Vec<usize> = values_part
            .lines()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        DayFive {
            valid_values,
            available_values,
            total_valid: 0,
            covered_ranges: HashMap::new(),
        }
    }

    fn is_valid_value(&self, value: usize) -> bool {
        for (start, end) in self.valid_values.iter() {
            if value >= *start && value <= *end {
                return true;
            }
        }
        false
    }

    pub fn calc_covered_ranges(&mut self) {
        let mut total = 0;
        let mut non_covered_ranges = self.valid_values.clone();
        non_covered_ranges.sort_by_key(|&(start, _end)| start);

        // Get first for inital run
        let (mut cstart, mut cend) = non_covered_ranges[0];
        for (start, end) in non_covered_ranges.into_iter().skip(1) {
            if start <= cend + 1 {
                // Case 1 & 3
                if end > cend {
                    cend = end;
                }
            } else {
                // Case 2
                total += cend - cstart + 1;
                cstart = start;
                cend = end;
            }
        }

        total += cend - cstart + 1;

        println!("Total covered ranges: {}", total);
    }

    pub fn calculate_all_valid_ids_from_covered_range(&self) -> usize {
        let mut total_valid_ids = 0;
        for (start, end) in self.covered_ranges.iter() {
            total_valid_ids += end - start + 1;
        }
        total_valid_ids
    }

    pub fn calc_total(&mut self) {
        for ingredient in self.available_values.iter() {
            if self.is_valid_value(*ingredient) {
                self.total_valid += 1;
            }
        }
    }

    pub fn get_total_valid(&self) -> usize {
        self.total_valid
    }
}
