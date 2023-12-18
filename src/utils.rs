pub fn extract_path_method(request: &str) -> Option<(&str, &str)> {
    // Split the request into lines
    let lines: Vec<&str> = request.lines().collect();

    // Check if the request is not empty and starts with "GET"
    if let Some(first_line) = lines.get(0) {
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        // Check if there's a path in the expected position
        let path = (
            parts.get(1).unwrap().clone(),
            parts.get(0).unwrap().clone()
        );
        return Some(path);
    }
    None
}