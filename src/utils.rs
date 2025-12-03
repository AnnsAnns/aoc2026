pub fn file_to_string(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read file")
}
