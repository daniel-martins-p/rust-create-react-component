use std::path::PathBuf;

use crate::services::{files::{create_file::{create_file}, create_folder::{create_folder}, write_content::write_content}};
use crate::models::components::constants::{COMPONENT_EXPORT_INDEX_TEMPLATE, COMPONENT_TEST_TEMPLATE, COMPONENT_TEMPLATE};

pub mod constants;

pub struct Component {
  pub name: String,
  pub path: PathBuf,
  pub folder_name: String,
}

impl Component {
  pub fn build(name: &str, folder_name: &str) -> Self {
    let component = Self {
      name: name.to_string(),
      path: PathBuf::from(format!("{folder_name}{name}.tsx")),
      folder_name: folder_name.to_string(),
    };
    component.create();
    component
  }

  fn create(&self) {
    println!("Creating \x1b[93m<{} />\x1b[0m component", self.name);
    self.create_component_folder();
    self.create_component_file();
    self.create_component_test_file();
    self.create_component_export_index_file();
  }

  fn create_component_folder(&self) {
    println!("Creating {} folder", self.folder_name);
    create_folder(PathBuf::from(self.folder_name.clone()));
  }

  fn create_component_file(&self) {
    let file_name = format!("{}/{}.tsx", self.folder_name, self.name);
    println!("Creating {}", file_name);
    let file = create_file(&file_name);
    let content = COMPONENT_TEMPLATE.replace("{component_name}", &self.name);
    write_content(file, &content);
  }

  fn create_component_test_file(&self) {
    let file_name = format!("{}/{}.test.tsx", self.folder_name, self.name);
    println!("Creating {}", file_name);
    let file = create_file(&file_name);
    let content = COMPONENT_TEST_TEMPLATE.replace("{component_name}", &self.name);
    write_content(file, &content)
  }

  fn create_component_export_index_file(&self) {
    let file_name = format!("{}/{}.tsx", self.folder_name, "index");
    println!("Creating {}", file_name);
    let file = create_file(&file_name);
    let content = COMPONENT_EXPORT_INDEX_TEMPLATE.replace("{component_name}", &self.name);
    write_content(file, &content)
  }
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    
    #[test]
    fn it_should_create_component_folder() {
        let component = Component {
        name: "Test".to_string(),
        path: PathBuf::from("Test.tsx"),
        folder_name: "Test".to_string(),
        };
        component.create_component_folder();
        assert!(Path::new("Test").exists());
    }
    
    #[test]
    fn it_should_create_component_file() {
        let component = Component {
        name: "Test".to_string(),
        path: PathBuf::from("Test.tsx"),
        folder_name: "Test".to_string(),
        };
        component.create_component_file();
        assert!(Path::new("Test/Test.tsx").exists());
    }
    
    #[test]
    fn it_should_create_component_test_file() {
        let component = Component {
        name: "Test".to_string(),
        path: PathBuf::from("Test.tsx"),
        folder_name: "Test".to_string(),
        };
        component.create_component_test_file();
        assert!(Path::new("Test/Test.test.tsx").exists());
    }
    
    #[test]
    fn it_should_create_component_export_index_file() {
        let component = Component {
        name: "Test".to_string(),
        path: PathBuf::from("Test.tsx"),
        folder_name: "Test".to_string(),
        };
        component.create_component_export_index_file();
        assert!(Path::new("Test/index.tsx").exists());
    }
}