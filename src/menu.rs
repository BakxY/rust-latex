/**
 * @file menu.rs
 * @brief A lib file to handle all menus
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use crate::files;
use crate::config;

pub fn display_template_selection() {
    println!("-------------------- RUST-LATEX --------------------\n");
    println!("{}\n", files::get_template_location());
    println!("--------------------- TEMPLATES --------------------\n");
    display_all_templates();
    println!("----------------------------------------------------\n");
}

fn display_all_templates() {
    let all_templates = files::get_templates();

    for i in 0 .. all_templates.len() {
        let human_readable = config::get_human_readable_name(all_templates.get(i).unwrap().to_string());

        println!("  {}. {}", i + 1, human_readable);
    }

    print!("\n");
}
