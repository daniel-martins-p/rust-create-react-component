use std::fs::File;

pub fn create_file(file_name: &str) -> File {
  File::create(file_name).expect("Failed to create file")
}