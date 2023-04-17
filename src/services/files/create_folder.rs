pub fn create_folder(path: std::path::PathBuf) {
  if !path.exists() {
    std::fs::create_dir_all(path).expect("Failed to create folder");
  }
}