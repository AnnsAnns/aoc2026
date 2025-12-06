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
    grid: Vec<CellType>
}

impl DaySix {
    pub fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        let lines: Vec<&str> = input.lines().collect();
        
        if lines.is_empty() {
            return DaySix { grid };
        }
        
        // Split all lines into columns
        let columns: Vec<Vec<&str>> = lines
            .iter()
            .map(|line| line.split_whitespace().collect())
            .collect();
        
        // Get number of columns from first line
        let num_cols = columns[0].len();
        
        // Process each column
        for col_idx in 0..num_cols {
            let mut operation = ' ';
            let mut numbers = Vec::new();
            
            for row in &columns {
                if col_idx < row.len() {
                    let cell = row[col_idx];
                    let first_char = cell.chars().next().unwrap();
                    
                    if first_char.is_digit(10) {
                        let number: usize = cell.parse().unwrap();
                        numbers.push(number);
                    } else {
                        operation = first_char;
                    }
                }
            }
            
            grid.push(CellType {
                Operation: operation,
                Numbers: numbers,
            });
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