pub const GRID_DATA: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

#[derive(Debug)]
struct CellType {
    Operation: char,
    Numbers: Vec<usize>,
}

#[derive(Debug)]
pub struct DaySix {
    grid: Vec<CellType>,
}

impl DaySix {
    pub fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        let lines: Vec<&str> = input.lines().collect();

        if lines.is_empty() {
            return DaySix { grid };
        }

        // The operation in the last line always starts on the character that is a new column
        // Thus we need to split each line into columns based on where the operations are
        let mut columns: Vec<Vec<String>> = Vec::new();
        let operation_line = lines.last().unwrap();
        let mut current_col = Vec::new();
        let mut last_index = 0;
        for (idx, ch) in operation_line.char_indices() {
            if !ch.is_whitespace() {
                // New column found
                for line in &lines {
                    let part = line[last_index..idx].to_string();
                    current_col.push(part);
                }
                if !current_col.is_empty() && !current_col[0].is_empty() {
                    println!("Current Col: {:?}", current_col);
                    columns.push(current_col);
                }
                current_col = Vec::new();
                last_index = idx;
            }
        }
        // Add the last column
        for line in &lines {
            let part = line[last_index..].to_string();
            current_col.push(part);
        }
        columns.push(current_col);

        println!("Columns: {:?}", columns);

       for col in columns {
            let mut number_chars = Vec::new();
            let mut operation = ' ';

            let cell_size = col[0].chars().count();
            for _ in 0..cell_size {
                number_chars.push("".to_string());
            }

            for cell in &col {
                let first_char = cell.chars().next().unwrap();

                if !first_char.is_whitespace() && !first_char.is_ascii_digit() {
                    operation = first_char;
                    continue;
                }

                for (index, ch) in cell.chars().enumerate() {
                    if ch.is_ascii_digit() {
                        number_chars[index] = format!("{}{}", number_chars[index], ch);
                    }
                }
            }

            let mut numbers = Vec::new();
            for num_str in number_chars {
                if !num_str.is_empty() {
                    let num = num_str.parse::<usize>().unwrap();
                    numbers.push(num);
                }
            }

            grid.push(CellType { Operation: operation, Numbers: numbers });
        }

        println!("Parsed Grid: {:?}", grid);

        DaySix { grid }
    }

    pub fn process_grid(&self) {
        let mut total = 0;
        for cell in &self.grid {
            total += match cell.Operation {
                '+' => {
                    let sum: usize = cell.Numbers.iter().sum();
                    sum
                }
                '*' => {
                    let product: usize = cell.Numbers.iter().product();
                    product
                }
                _ => {
                    println!("No operation for cell: {:?}", cell);
                    0
                }
            }
        }
        println!("Total: {}", total);
    }
}
