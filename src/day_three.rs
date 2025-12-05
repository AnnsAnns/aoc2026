pub const GRID_DATA: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

struct LinePos {
    pub index: usize,
    pub number: usize,
}

pub struct DayThree {
    battery_grid: Vec<String>,
    total: usize,
}

impl DayThree {
    pub fn new(input: &str) -> Self {
        let battery_grid = input.lines().map(|line| line.to_string()).collect();
        DayThree {
            battery_grid,
            total: 0,
        }
    }

    pub fn find_largest_number_in_line(&self, line: &String) -> LinePos {
        let mut best_position = LinePos {
            index: 0,
            number: 0,
        };

        for (char_index, char) in line.chars().enumerate() {
            let number = match char.to_digit(10) {
                Some(n) => n as usize,
                None => {
                    panic!("Invalid character in line {}: {}", char, line);
                }
            };
            if number > best_position.number {
                best_position.number = number;
                best_position.index = char_index;
            }
        }

        best_position
    }

    pub fn find_largest_n_numbers_in_line(&self, line: &String, n: usize) -> usize {
        let mut total = "".to_string();
        let mut start_cutoff = 0;
        let mut end_cutoff = line.chars().count() - n;

        for i in 0..n {
            let new_string = line[start_cutoff..=end_cutoff].to_string();
            let largest = self.find_largest_number_in_line(&new_string);
            total = format!("{}{}", total, largest.number);
            println!(
                "Found largest n {}: {} at index {} in {}",
                i + 1,
                largest.number,
                largest.index,
                new_string
            );
            start_cutoff += largest.index + 1;
            end_cutoff += 1;
        }

        println!("Line: {}, Largest {} numbers combined: {}", line, n, total);

        total.parse::<usize>().unwrap()
    }

    pub fn process_input(&mut self, batteries: usize) {
        for line in self.battery_grid.iter() {
            self.total += self.find_largest_n_numbers_in_line(line, batteries);
        }
    }

    pub fn get_total(&self) -> usize {
        self.total
    }
}
