use crate::services::files::{create_file, create_folder, write_content};
use crate::models::components::Component;

#[cfg(test)]
mod tests {
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