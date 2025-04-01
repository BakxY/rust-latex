use config::get_human_readable_name;

/**
 * @file main.rs
 * @brief The main source file of the rust-latex project
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

mod menu;
mod files;
mod config;

fn main() {
    let _ = files::get_templates();
    menu::display_template_selection();
    get_human_readable_name("letter".to_string());
}