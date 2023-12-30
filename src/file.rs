use std::fs;
use std::path::Path;

pub fn parse_filename_from_request_path(path: &str) -> Option<String> {
    let filename = path[7..].to_string(); // Assuming '/files/' is always present
    let path = Path::new(&filename);

    // Check for directory traversal attempts
    if path
        .components()
        .any(|comp| comp == std::path::Component::ParentDir)
    {
        return None; // Reject paths with '..'
    }

    path.to_str().map(|s| s.to_string())
}

pub fn read_file_to_string(file_path: &Path) -> Option<String> {
    match fs::read_to_string(file_path) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

pub fn write_string_to_file(file_path: &Path, to_write: &str) -> io::Result<()> {
    let mut data_file = File::create(file_path).expect("creation failed");
    data_file.write(to_write.as_bytes()).expect("write failed");
    Ok(())
}
