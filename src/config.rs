/**
 * @file config.rs
 * @brief A lib file used to interact with the config of the project
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use std::fs;
use std::path::{Path, PathBuf};
use crate::files;

fn build_template_config_path(template_name: String) -> PathBuf {
    let base_path = files::get_template_location();

    let path_to_config = Path::new(base_path.as_str()).join(template_name.clone()).join(template_name + ".conf");

    return path_to_config;
}

fn read_config(path_to_config: PathBuf) -> Vec<String> {
    let config_file = fs::read_to_string(path_to_config).unwrap();

    let config_lines = config_file.split("\n").map(|n| n.to_string()).collect();

    return config_lines;
}

pub fn get_human_readable_name(template_name: String) -> String
{
    let path_to_config = build_template_config_path(template_name.clone());
    let config_lines = read_config(path_to_config);

    for line in config_lines {
        let line = line.to_string();

        if line.starts_with("READABLE ") {
            return line.replace("READABLE ", "");
        }
    }

    return template_name;
}

pub struct ReplaceField {
    group: String,
    readable: String,
    replace: String,
    value: Option<String>
}

pub fn get_all_template_fields(template_name: String) -> Vec<ReplaceField> {
    let path_to_config = build_template_config_path(template_name.clone());
    let config_lines = read_config(path_to_config);
    
    let fields: Vec<ReplaceField> = Vec::new();
    let mut current_group = "".to_string();

    for line in config_lines {
        let line = line.to_string();

        if line.starts_with("GROUP ") {
            current_group = line.replace("GROUP ", "");
        }

        if line.starts_with("FIELD ") {

        }
    }

    return fields;
}