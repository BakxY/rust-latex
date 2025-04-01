mod config;
mod files;
mod menu;
/**
 * @file main.rs
 * @brief The main source file of the rust-latex project
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

fn main() {
    menu::display_template_selection();
    config::get_human_readable_name("letter".to_string());
    config::get_all_template_fields("letter".to_string());
}
