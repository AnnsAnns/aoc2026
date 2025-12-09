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

#[derive(Debug, Clone)]
pub struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    x_pos: usize,
    y_pos: usize,
}

impl TreeNode {
    fn new(x_pos: usize, y_pos: usize) -> Self {
        TreeNode {
            left: None,
            right: None,
            x_pos,
            y_pos,
        }
    }
    fn insert_left(&mut self, node: TreeNode) {
        self.left = Some(Box::new(node));
    }
    fn insert_right(&mut self, node: TreeNode) {
        self.right = Some(Box::new(node));
    }
}

impl crate::utils::BinaryTreeDisplay for TreeNode {
    fn get_left(&self) -> Option<&Self> {
        self.left.as_ref().map(|boxed| boxed.as_ref())
    }

    fn get_right(&self) -> Option<&Self> {
        self.right.as_ref().map(|boxed| boxed.as_ref())
    }

    fn get_display_text(&self) -> String {
        format!("({}, {})", self.x_pos, self.y_pos)
    }
}

pub struct DaySeven {
    pub tree: Vec<Vec<char>>,
    splits_done: usize,
    root_node: Option<TreeNode>,
}

impl DaySeven {
    pub fn new(input: &str) -> Self {
        let tree: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        DaySeven {
            tree,
            splits_done: 0,
            root_node: None,
        }
    }

    fn is_out_of_bounds(&self, row: isize, col: isize) -> bool {
        row < 0 || col < 0 || row as usize >= self.tree.len() || col as usize >= self.tree[0].len()
    }

    pub fn tree_to_binary_tree(&mut self) -> Option<Box<TreeNode>> {
        // The part 1 solution must have drawn the beams first
        // Saves me some time :)
        if self.root_node.is_none() {
            self.draw_beams();
        }
        
        let mut queue: Vec<&mut Box<TreeNode>> = Vec::new();
        // Heck it, lets do depth first search because I 
        // cant imagine doing anything better on a Sunday
        let mut root = Box::new(self.root_node.take().unwrap());
        queue.push(&mut root);

        while let Some(current_node) = queue.pop() {
            
            let x = current_node.x_pos;
            let y = current_node.y_pos;

            for direction in [-1, 1] {
                // We follow each beam downwards
                for offset in 1..self.tree.len()-y {
                    let char = self.tree[y + offset][(x as isize + direction) as usize];
                    if char == '^' {
                        let new_node = TreeNode::new((x as isize + direction) as usize, y + offset);
                        if direction == -1 {
                            current_node.insert_left(new_node);
                        } else {
                            current_node.insert_right(new_node);
                        }
                        break;
                    }
                }
            }

            // Create leaf nodes if children don't exist
            let tree_height = self.tree.len();
            let left_leaf = current_node.left.is_none();
            if left_leaf {
                let leaf_node = TreeNode::new(x, tree_height - 1);
                current_node.insert_left(leaf_node);
            }
            let right_leaf = current_node.right.is_none();
            if right_leaf {
                let leaf_node = TreeNode::new(x, tree_height - 1);
                current_node.insert_right(leaf_node);
            }

            // Push child nodes after we're done mutating current_node
            // to make Rust happy
            // But ignore leaf nodes
            if let Some(ref mut left) = current_node.left && !left_leaf {
                queue.push(left);
            }
            if let Some(ref mut right) = current_node.right && !right_leaf {
                queue.push(right);
            }
        }

        let result = Box::new(*root.clone());
        // Store the tree back in root_node for later use
        self.root_node = Some(*root);
        Some(result)
    }


    pub fn calculate_all_possible_paths(&self, root: &TreeNode) -> Vec<Vec<(usize, usize)>> {
        let mut all_paths = Vec::new();
        let mut current_path = Vec::new();
        
        fn dfs(
            node: &TreeNode,
            current_path: &mut Vec<(usize, usize)>,
            all_paths: &mut Vec<Vec<(usize, usize)>>,
        ) {
            current_path.push((node.x_pos, node.y_pos));

            if node.left.is_none() && node.right.is_none() {
                all_paths.push(current_path.clone());
            } else {
                if let Some(ref left) = node.left {
                    dfs(left, current_path, all_paths);
                }
                if let Some(ref right) = node.right {
                    dfs(right, current_path, all_paths);
                }
            }

            current_path.pop();
        }

        dfs(root, &mut current_path, &mut all_paths);

        println!("Total possible paths: {}", all_paths.len());

        all_paths
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
                            // Required for part 2 (And we don't care about S anyway)
                            if self.root_node.is_none() {
                                self.root_node = Some(TreeNode::new(char_index, line_index));
                            }

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
                                println!("After split:\n{}", self.tree_to_string(&self.tree));
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

    pub fn tree_to_string(&self, tree: &Vec<Vec<char>>) -> String {
        tree.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_splits_done(&self) -> usize {
        self.splits_done
    }

    /// Print the binary tree structure with proper formatting
    pub fn display_binary_tree(&self) -> Option<String> {
        use crate::utils::BinaryTreeDisplay;
        self.root_node.as_ref().map(|root| root.display_tree())
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
        let result_tree = day_seven.tree_to_string(&day_seven.tree);
        assert_eq!(result_tree, EXEPECTED_RESULT_TREE);
        assert_eq!(day_seven.get_splits_done(), EXPECTED_SPLITS);
    }

    #[test]
    fn diff_against_expected_tree() {
        let mut day_seven = DaySeven::new(GRID_DATA);
        day_seven.draw_beams();
        let result_tree = day_seven.tree_to_string(&day_seven.tree);
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

    #[test]
    fn test_binary_tree_visualization() {
        let mut day_seven = DaySeven::new(GRID_DATA);
        day_seven.tree_to_binary_tree();
        
        if let Some(tree_display) = day_seven.display_binary_tree() {
            println!("\nBinary Tree Visualization:");
            println!("{}", tree_display);
        } else {
            panic!("Binary tree was not created");
        }
    }
}
