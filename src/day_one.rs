const TEST_ANSWER_SIZE: usize = 6;
const TEST_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
const TEST_INPUTS: usize = 10;
const WRAP_AT: usize = 100;

#[derive(Debug, PartialEq)]
enum Instruction {
    Left(usize),
    Right(usize),
}

pub struct DayOne {
    inputs: Vec<Instruction>,
    current_position: usize,
    numbers_of_zeroes: usize,
}

impl DayOne {
    pub fn new(input: &str) -> Self {
        let inputs = input
            .lines()
            .map(|line| {
                let (dir, dist) = line.split_at(1);
                // Keep the full distance to count all times we pass 0
                let distance = dist.parse::<usize>().unwrap();
                match dir {
                    "L" => Instruction::Left(distance),
                    "R" => Instruction::Right(distance),
                    _ => panic!("Invalid direction"),
                }
            })
            .collect();
        DayOne {
            inputs,
            current_position: 50,
            numbers_of_zeroes: 0,
        }
    }

    pub fn calculate_position(&mut self) {
        for input in &self.inputs {
            match input {
                Instruction::Left(distance) => {
                    let distance = distance.to_owned();
                    // Count how many times we pass 0 when moving left
                    self.numbers_of_zeroes += (self.current_position + distance) / WRAP_AT;
                    self.current_position = if self.current_position < distance {
                        WRAP_AT + self.current_position - (distance % WRAP_AT)
                    } else {
                        self.current_position - distance
                    };
                    self.current_position %= WRAP_AT;
                }
                Instruction::Right(distance) => {
                    // Count how many times we pass 0 when moving right
                    self.numbers_of_zeroes += (self.current_position + distance) / WRAP_AT;
                    self.current_position = (self.current_position + distance) % WRAP_AT;
                }
            }
        }
    }

    pub fn get_number_of_zeroes(&self) -> usize {
        self.numbers_of_zeroes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one_initialization() {
        let day_one = DayOne::new(TEST_INPUT);
        assert_eq!(day_one.inputs.len(), TEST_INPUTS);
    }

    #[test]
    fn test_day_one_calculate_position() {
        let mut day_one = DayOne::new(TEST_INPUT);
        day_one.calculate_position();
        assert_eq!(day_one.get_number_of_zeroes(), TEST_ANSWER_SIZE);
    }
}
