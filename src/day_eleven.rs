pub const EXAMPLE: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

use std::collections::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
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

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct CacheKey {
    key: String,
    filter: Filter,
}

impl CacheKey {
    pub fn new(key: String, filter: Filter) -> Self {
        CacheKey {
            key: key,
            filter: filter,
        }
    }
}

pub struct DayEleven {
    input: HashMap<String, Vec<String>>,
    cache: HashMap<CacheKey, usize>,
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

    pub fn depth_first_search(&mut self, key: &str, mut filter_state: Filter) -> usize {
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

        let cache_key = CacheKey::new(key.to_string(), filter_state);
        if let Some(&size) = self.cache.get(&cache_key) {
            return size;
        }
        let children = &self.input[key].clone();

        let size: usize = children
            .iter()
            .map(|node| self.depth_first_search(node, filter_state))
            .sum();

        let cache_key = CacheKey::new(key.to_string(), filter_state);
        self.cache.insert(cache_key, size);
        size
    }
}
