/***
 * @file config.rs
 * @brief A lib file used to interact with the config of the project
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use crate::files;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

fn build_template_config_path(template_name: String) -> PathBuf {
    let base_path = files::get_template_location();

    let path_to_config = Path::new(base_path.as_str())
        .join(template_name.clone())
        .join(template_name + ".conf");

    return path_to_config;
}

fn read_config(path_to_config: PathBuf) -> Vec<String> {
    let config_file = fs::read_to_string(path_to_config).unwrap();

    let config_lines = config_file.split("\n").map(|n| n.to_string()).collect();

    return config_lines;
}

pub fn get_human_readable_name(template_name: String) -> String {
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
    pub group: String,
    pub readable: String,
    pub replace: String,
    pub value: String,
}

pub type FieldGroup = String;

pub fn get_all_template_fields(template_name: String) -> (Vec<ReplaceField>, Vec<FieldGroup>) {
    let path_to_config = build_template_config_path(template_name.clone());
    let config_lines = read_config(path_to_config);

    let mut fields: Vec<ReplaceField> = Vec::new();
    let mut groups: Vec<FieldGroup> = Vec::new();
    let mut current_group = "".to_string();

    let field_regex = Regex::new(r#"FIELD\s+"([^"]+)"\s+"([^"]+)"#).unwrap();

    for line in config_lines {
        let line = line.to_string();

        if line.starts_with("GROUP ") {
            current_group = line.replace("GROUP ", "").trim().to_string();
            groups.push(current_group.clone());
        }

        if line.starts_with("FIELD ") {
            let line = line.replace("\r", "");
            let line_matches = field_regex.captures(&line);

            match line_matches {
                Some(captures) => {
                    if captures.len() == 3 {
                        fields.push(ReplaceField {
                            group: current_group.clone(),
                            readable: captures.get(2).unwrap().as_str().trim().to_string(),
                            replace: captures.get(1).unwrap().as_str().trim().to_string(),
                            value: "".to_string(),
                        });
                    }
                }
                None => {}
            }
        }
    }

    return (fields, groups);
}

pub fn get_group_fields(all_fields: Vec<ReplaceField>, group: FieldGroup) -> (Vec<ReplaceField>, Vec<ReplaceField>) {
    let mut filtered_fields: Vec<ReplaceField> = Vec::new();
    let mut all_fields_cleaned: Vec<ReplaceField> = Vec::new();

    for field in all_fields {
        if field.group == group {
            filtered_fields.push(field);
        }
        else {
            all_fields_cleaned.push(field);
        }
    }

    return (all_fields_cleaned, filtered_fields);
}

pub fn get_template_tex_file(template_name: String) -> String {
    let path_to_config = build_template_config_path(template_name);
    let config_lines = read_config(path_to_config);

    let mut tex_file_read_path = "".to_string();

    for line in config_lines {
        let line = line.to_string();

        if line.starts_with("TEX_FILE ") {
            tex_file_read_path = line.replace("TEX_FILE ", "");
        }
    }

    return tex_file_read_path;
}

pub fn get_template_tex_location(template_name: String) -> PathBuf {
    let tex_file_read_path = get_template_tex_file(template_name.clone());

    let base_path = files::get_template_location();

    let path_to_tex = Path::new(base_path.as_str())
        .join(template_name)
        .join(tex_file_read_path);

    return path_to_tex;
}