pub const GRID_DATA: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

pub const EXPECTED_SPLITS: usize = 21;

pub const EXEPECTED_RESULT_TREE: &str = ".......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....
...|^|^|||^|...
...|.|.|||.|...
..|^|^|||^|^|..
..|.|.|||.|.|..
.|^|||^||.||^|.
.|.|||.||.||.|.
|^|^|^|^|^|||^|
|.|.|.|.|.|||.|";

pub struct DaySeven {
    tree: Vec<Vec<char>>,
    splits_done: usize,
}

impl DaySeven {
    pub fn new(input: &str) -> Self {
        let tree: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        DaySeven {
            tree,
            splits_done: 0,
        }
    }

    fn is_out_of_bounds(&self, row: isize, col: isize) -> bool {
        row < 0 || col < 0 || row as usize >= self.tree.len() || col as usize >= self.tree[0].len()
    }

    pub fn draw_beams(&mut self) {
        let height = self.tree.len();
        let width = if height > 0 { self.tree[0].len() } else { 0 };

        // First pass: handle 'S' and '^' markers
        for line_index in 0..height {
            for char_index in 0..width {
                let char = self.tree[line_index][char_index];
                // Don't go out of bounds
                match char {
                    'S' => {
                        // Start point, shoot beams in all four directions
                        if line_index + 1 < height {
                            self.tree[line_index + 1][char_index] = '|'; // Down
                        }
                    }
                    '^' => {
                        if self.is_out_of_bounds(line_index as isize - 1, char_index as isize) {
                            continue;
                        }

                        if self.tree[line_index - 1][char_index] == '|' {
                            let mut has_split = false;
                            if char_index > 0
                                && self.tree[line_index][char_index - 1] == '.'
                            {
                                println!("Split at ({}, {})", line_index, char_index - 1);
                                self.tree[line_index][char_index - 1] = '|';
                                has_split = true;
                            }
                            if char_index + 1 < width
                                && self.tree[line_index][char_index + 1] == '.'
                            {
                                println!("Split at ({}, {})", line_index, char_index + 1);
                                self.tree[line_index][char_index + 1] = '|';
                                has_split = true;
                            }
                            if has_split {
                                self.splits_done += 1;
                                println!("After split:\n{}", self.tree_to_string());
                                println!("Total splits: {}", self.splits_done);
                            }
                        }
                    }
                    '.' => {
                        if self.is_out_of_bounds(line_index as isize - 1, char_index as isize) {
                            continue;
                        }

                        if self.tree[line_index - 1][char_index] == '|' {
                            self.tree[line_index][char_index] = '|';
                        }
                    }
                    '|' => {}
                    _ => {
                        panic!(
                            "Unexpected character in tree grid at ({}, {}): {}",
                            line_index, char_index, char
                        );
                    }
                }
            }
        }
    }

    pub fn tree_to_string(&self) -> String {
        self.tree
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_splits_done(&self) -> usize {
        self.splits_done
    }
}

// Test to diff against expected result
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day_seven_beam_drawing() {
        let mut day_seven = DaySeven::new(GRID_DATA);
        day_seven.draw_beams();
        let result_tree = day_seven.tree_to_string();
        assert_eq!(result_tree, EXEPECTED_RESULT_TREE);
        assert_eq!(day_seven.get_splits_done(), EXPECTED_SPLITS);
    }

    #[test]
    fn diff_against_expected_tree() {
        let mut day_seven = DaySeven::new(GRID_DATA);
        day_seven.draw_beams();
        let result_tree = day_seven.tree_to_string();
        let expected_lines: Vec<&str> = EXEPECTED_RESULT_TREE.lines().collect();
        let result_lines: Vec<&str> = result_tree.lines().collect();

        for (i, (expected_line, result_line)) in
            expected_lines.iter().zip(result_lines.iter()).enumerate()
        {
            assert_eq!(
                expected_line,
                result_line,
                "Mismatch at line {}: expected '{}', got '{}'",
                i + 1,
                expected_line,
                result_line
            );
        }
    }
}
