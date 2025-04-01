/**
 * @file files.rs
 * @brief A lib file used to read and write template files
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use std::fs;

const TEMPLATE_DIR: &str = "templates/";

pub fn get_template_location() -> String {
    return TEMPLATE_DIR.to_string();
}

pub fn get_templates() -> Vec<String> {
    let mut templates: Vec<String> = Vec::new();

    for entry in fs::read_dir("templates").unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir()
        {
            let path = entry.path();
            let filename = path.file_name().unwrap();
            let filename_str = filename.to_str().unwrap();

            templates.push(filename_str.to_string());
        }
    }

    return templates;
}