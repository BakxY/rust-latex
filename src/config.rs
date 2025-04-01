/**
 * @file config.rs
 * @brief A lib file used to interact with the config of the project
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use std::fs;
use std::path::Path;
use crate::files;

pub fn get_human_readable_name(template_name: String) -> String
{
    let base_path = files::get_template_location();

    let path_to_config = Path::new(base_path.as_str()).join(template_name.clone()).join(template_name.clone() + ".conf");

    let config_file = fs::read_to_string(path_to_config).unwrap();

    let config_lines = config_file.split("\n");

    for line in config_lines {
        let line = line.to_string();

        if line.starts_with("READABLE ") {
            return line.replace("READABLE ", "");
        }
    }

    return template_name;
}