pub const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

use std::collections::HashMap;

pub struct Filter {
    went_through_dac: bool,
    went_through_fft: bool,
}

impl Filter {
    pub fn went_through_both(&self) -> bool {
        self.went_through_dac && self.went_through_fft
    }

    pub fn new() -> Self {
        Filter {
            went_through_dac: false,
            went_through_fft: false,
        }
    }
}

pub struct DayEleven {
    input: HashMap<String, Vec<String>>,
    cache: HashMap<String, usize>,
    should_filter: bool,
}

impl DayEleven {
    pub fn new(input: &str, filter: bool) -> Self {
        let mut input_map = HashMap::new();
        for line in input.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            let node = parts[0].trim().to_string();
            let children: Vec<String> = if parts.len() > 1 {
                parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect()
            } else {
                Vec::new()
            };
            input_map.insert(node, children);
        }

        DayEleven {
            input: input_map,
            cache: HashMap::new(),
            should_filter: filter,
        }
    }

    pub fn depth_first_search(&mut self, key: &str, filter_state: &mut Filter) -> usize {
        if self.should_filter {
            if key == "dac" {
                filter_state.went_through_dac = true;
            } else if key == "fft" {
                filter_state.went_through_fft = true;
            } else if key == "out" {
                return if filter_state.went_through_both() {
                    1
                } else {
                    0
                };
            }
        } else {
            if key == "out" {
                return 1;
            }
        }

        if let Some(&size) = self.cache.get(key) {
            return size;
        }
        let children = &self.input[key].clone();

        let size: usize = children
            .iter()
            .map(|node| self.depth_first_search(node, filter_state))
            .sum();

        self.cache.insert(key.to_string(), size);
        size
    }
}
