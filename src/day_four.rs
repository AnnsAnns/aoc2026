pub const GRID_DATA: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

pub const GRID_ANSWER: usize = 13;

pub struct DayFour {
    grid: Vec<Vec<bool>>,
    valid_positions: usize,
}

impl DayFour {
    pub fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c == '@').collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();

        DayFour {
            grid,
            valid_positions: 0,
        }
    }

    fn is_cell_marked(&self, row_index: usize, col_index: usize) -> bool {
        self.grid[row_index][col_index]
    }

    pub fn find_valid_pos(&mut self) {
        for (row_index, row) in self.grid.iter().enumerate() {
            'col_for: for (col_index, col) in row.iter().enumerate() {
                if col.clone() {
                    let mut total_marked = 0;

                    // Check all 8 neighbors
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            if dr == 0 && dc == 0 {
                                continue; // Skip the cell itself
                            }
                            let neighbor_row = row_index as isize + dr;
                            let neighbor_col = col_index as isize + dc;

                            // Check bounds
                            if neighbor_row < 0
                                || neighbor_row >= self.grid.len() as isize
                                || neighbor_col < 0
                                || neighbor_col >= row.len() as isize
                            {
                                continue;
                            }

                            if self.is_cell_marked(neighbor_row as usize, neighbor_col as usize) {
                                total_marked += 1;
                            }

                            if total_marked >= 4 {
                                continue 'col_for;
                            }
                        }
                    }

                    println!(
                        "Valid position found at ({}, {}) with {} marked neighbors",
                        row_index, col_index, total_marked
                    );
                    self.valid_positions += 1;
                }
            }
        }
    }

    pub fn get_total(&self) -> usize {
        self.valid_positions
    }
}
