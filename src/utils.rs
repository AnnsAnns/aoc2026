pub fn file_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read file")
}

/// Generic binary tree node display trait
pub trait BinaryTreeDisplay {
    fn get_left(&self) -> Option<&Self>;
    fn get_right(&self) -> Option<&Self>;
    fn get_display_text(&self) -> String;

    /// Display the tree in a formatted string with proper indentation
    fn display_tree(&self) -> String {
        let mut result = String::new();
        self.display_tree_helper(&mut result, "", "", true);
        result
    }

    fn display_tree_helper(&self, result: &mut String, prefix: &str, child_prefix: &str, is_root: bool) {
        // Display current node
        result.push_str(&format!(
            "{}{}\n",
            if is_root { "" } else { prefix },
            self.get_display_text()
        ));

        // Prepare children display
        let has_right = self.get_right().is_some();

        // Display left child
        if let Some(left) = self.get_left() {
            let is_last = !has_right;
            let connector = if is_last { "└── L: " } else { "├── L: " };
            let extension = if is_last { "    " } else { "│   " };
            
            left.display_tree_helper(
                result,
                &format!("{}{}", child_prefix, connector),
                &format!("{}{}", child_prefix, extension),
                false,
            );
        }

        // Display right child
        if let Some(right) = self.get_right() {
            let connector = "└── R: ";
            let extension = "    ";
            
            right.display_tree_helper(
                result,
                &format!("{}{}", child_prefix, connector),
                &format!("{}{}", child_prefix, extension),
                false,
            );
        }
    }
}
