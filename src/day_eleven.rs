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

pub struct DayEleven {
    input: HashMap<String, Vec<String>>,
    cache: HashMap<String, usize>,
}

impl DayEleven {
    pub fn new(input: &str) -> Self {
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
        }
    }

    pub fn depth_first_search(&mut self, key: &str) -> usize {
        if key == "out" {
            return 1;
        }
        if let Some(&size) = self.cache.get(key) {
            return size;
        }
        let children = &self.input[key].clone();
        let size: usize = children
            .iter()
            .map(|node| self.depth_first_search(node))
            .sum();
        self.cache.insert(key.to_string(), size);
        size
    }
}
