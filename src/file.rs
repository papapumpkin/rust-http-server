use std::fs;
use std::path::Path;
use std::io::{self, ErrorKind};

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

pub fn read_file_to_string(file_path: &Path) -> Result<String, std::io::Error> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => Err(io::Error::new(ErrorKind::NotFound, format!("File not found: {}", file_path.display()))),
                ErrorKind::PermissionDenied => Err(io::Error::new(ErrorKind::PermissionDenied, "Permission denied")),
                _ => Err(e),
            }
        }
    }
}
