/**
 * @file config.rs
 * @brief A lib file used to interact with the config of the project
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use std::fs;

static mut RAW_CONFIG: Option<String> = None;

pub fn read_config() {
    match fs::read_to_string("res/rust-latex.conf") {
        Ok(content) => {
            unsafe {
                RAW_CONFIG = Some(content);
            }
        }
        Err(error) => {
            eprintln!("Error while reading config file!");
            eprint!("{}", error);
        }
    }
}