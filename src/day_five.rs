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
        let mut covered_ranges: HashMap<usize, usize> = HashMap::new();
        let mut non_covered_ranges = self.valid_values.clone();

        // Put first in covered
        let (_first_start, _first_end) = non_covered_ranges.remove(0);

        for (nstart, nend) in non_covered_ranges.iter() {
            for (cstart, cend) in covered_ranges.clone().iter() {
                let is_nstart_within_cstart = nstart >= cstart && nstart <= cend;
                let is_nend_within_cend = nend >= cstart && nend <= cend;
                let is_cend_within_nend = cend >= nstart && cend <= nend;
                let is_cstart_within_nstart = cstart >= nstart && cstart <= nend;
                let is_no_overlap = (nend < cstart) || (nstart > cend);

                // Case 1: Non covered within already covered, thus we extend the covered range
                if is_nstart_within_cstart {
                    // Case 1b: Non Covered is larger than covered
                    if !is_nend_within_cend {
                        println!(
                            "Case 1b: {} - {} extends {} - {}",
                            nstart, nend, cstart, cend
                        );
                        // Extend covered end
                        covered_ranges.insert(*cstart, *nend);
                    }
                    // Case 1a: Full covered within crange
                    else {
                        prinlnt!(
                            "Case 1a: {} - {} fully covered by {} - {}",
                            nstart,
                            nend,
                            cstart,
                            cend
                        );
                        // Fully covered
                    }
                }
                // Case 2: Both start and end are outside of covered range
                else if is_no_overlap {
                    println!(
                        "Case 2: {} - {} has no overlap with {} - {}",
                        nstart, nend, cstart, cend
                    );
                    // No overlap, thus we can simply append to covered ranges
                    covered_ranges.insert(*nstart, *nend);
                } else if is_cstart_within_nstart {
                    // Case 3: Covered start is within non-covered range
                    if !is_cend_within_nend {
                        // Case 3a: Extend covered end
                        covered_ranges.insert(*nstart, *nend);
                    } else {
                        // Case 3b: We set cstart to nstart
                        covered_ranges.insert(*nstart, *cend);
                        covered_ranges.remove(cstart);
                    }
                }
            }
        }

        self.covered_ranges = covered_ranges;
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
