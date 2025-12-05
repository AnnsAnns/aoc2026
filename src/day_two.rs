pub const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const TEST_ANSWER: usize = 1227775554;

#[derive(Debug)]
pub struct DayTwo {
    inputs: Vec<(usize, usize)>,
    wrong_id_total: usize,
    wrong_ids: Vec<usize>,
}

impl DayTwo {
    pub fn new(file: &str) -> DayTwo {
        // We need to split each pair by , and then split the pair by -
        let inputs = file
            .lines()
            .flat_map(|line| line.split(','))
            .map(|pair| {
                println!("Processing pair: {}", pair);
                let mut parts = pair.split('-');
                let first = parts.next().unwrap().to_string();
                let second = parts.next().unwrap().to_string();
                println!("Parsed pair: {} - {}", first, second);
                (
                    first.parse::<usize>().unwrap(),
                    second.parse::<usize>().unwrap(),
                )
            })
            .collect();

        DayTwo {
            inputs,
            wrong_ids: Vec::new(),
            wrong_id_total: 0,
        }
    }

    fn is_wrong_id(id: String) -> bool {
        let mut storage = "".to_string();

        'char_loop: for (index, char) in id.char_indices() {
            storage.push(char);

            //println!("Current storage: {} for {}", storage, id);
            if id.chars().count() % storage.chars().count() != 0 {
                continue;
            }

            if id == storage {
                println!("ID is the same as storage: {}", storage);
                break;
            }

            println!("Checking storage pattern: {}", storage);

            // Split id chars into chunks of storage length
            let mut chunk_start = 0;
            while chunk_start < id.chars().count() {
                let chunk_end = chunk_start + storage.chars().count();
                let chunk = &id[chunk_start..chunk_end];
                chunk_start += storage.chars().count();
                if *chunk != storage {
                    println!("Mismatch: {} != {}", chunk, storage);
                    continue 'char_loop;
                }
            }
            println!("Found repeating pattern: {}", storage);
            return true;
        }
        println!("No repeating pattern found for id: {}", id);
        false
    }

    pub fn get_total_wrong_ids(&self) -> usize {
        self.wrong_id_total
    }

    pub fn print_wrong_ids(&self) {
        for id in &self.wrong_ids {
            println!("Wrong ID: {}", id);
        }
    }

    pub fn find_wrong_ids(&mut self) {
        for (one, two) in &self.inputs {
            for id in *one..=*two {
                let id_str = id.to_string();
                if DayTwo::is_wrong_id(id_str) {
                    self.wrong_id_total += id;
                    println!("Wrong ID found: {}", id);
                    self.wrong_ids.push(id);
                }
            }
        }
    }
}
