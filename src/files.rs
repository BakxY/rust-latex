/**
 * @file files.rs
 * @brief A lib file used to read and write template files
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use std::fs;
use std::io;

pub fn get_template_location() -> &'static str {
    return "Template path: ";
}

pub fn get_templates() -> io::Result<()> {
    let mut templates: Vec<Option<String>> = Vec::new();

    for entry in fs::read_dir("templates")? {
        let entry = entry?;
        if entry.path().is_dir()
        {
            let path = entry.path();
            let filename = path.file_name();

            match filename {
                Some(filename_str) => {
                    templates.push(filename_str.to_str().map(String::from));
                }
                None => ()
            }
        }
    }

    Ok(())
}