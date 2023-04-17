use std::{fs::File, io::Write};

pub fn write_content(mut file: File, content: &str) {
  file.write_all(content.as_bytes()).expect("Failed to write content");
}