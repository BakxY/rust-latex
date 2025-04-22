/***
 * @file files.rs
 * @brief A lib file used to read and write template files
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use std::fs;
use std::path::{Path, PathBuf};
use crate::config;
use crate::files;

const TEMPLATE_DIR: &str = "templates/";

pub fn get_template_location() -> String {
    return TEMPLATE_DIR.to_string();
}

pub fn get_templates() -> Vec<String> {
    let mut templates: Vec<String> = Vec::new();

    for entry in fs::read_dir("templates").unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            let path = entry.path();
            let filename = path.file_name().unwrap();
            let filename_str = filename.to_str().unwrap();

            templates.push(filename_str.to_string());
        }
    }

    return templates;
}

pub fn read_tex_template_file(path_to_tex_file: PathBuf) -> String {
    let config_file_string = fs::read_to_string(path_to_tex_file).unwrap();

    return  config_file_string;
}

pub fn populate_tex_template_fields(config_file_string: String, all_filled_fields: Vec<config::ReplaceField>) -> String {
    let mut config_file = config_file_string;

    for field in all_filled_fields {
        let field = field;
        config_file = config_file.replace(&field.replace, &field.value);
    }

    return config_file;
}

pub fn write_template(populated_tex_template: String) {
    let _ = fs::write("main.tex", populated_tex_template);
}

pub fn copy_all_template_files(template_name: String) {
    let base_path = files::get_template_location();

    let path_to_template_folder = Path::new(base_path.as_str())
        .join(template_name.clone());

    let _ = fs::copy(path_to_template_folder, "."); //! Doesn't work

    let path_to_tex = config::get_template_tex_file(template_name);

    let _ = fs::remove_file(path_to_tex);
}