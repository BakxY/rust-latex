/***
 * @file menu.rs
 * @brief A lib file to handle all menus
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use crate::config;
use crate::files;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::num::ParseIntError;

pub fn display_template_selection(all_templates: Vec<String>) {
    print!("{}[2J", 27 as char);
    println!("-------------------- RUST-LATEX --------------------\n");
    println!("{}\n", files::get_template_location());
    println!("--------------------- TEMPLATES --------------------\n");
    display_all_templates(all_templates);
    println!("----------------------------------------------------\n");
    println!("  Please select a template:");
    print!("  - ");

    let _ = stdout().flush();
}

fn display_all_templates(all_templates: Vec<String>) {
    for i in 0..all_templates.len() {
        let human_readable =
            config::get_human_readable_name(all_templates.get(i).unwrap().to_string());

        println!("  {}. {}", i + 1, human_readable);
    }

    print!("\n");
}

pub fn select_template() -> Result<usize, ParseIntError> {
    let mut user_input = String::new();

    let _ = stdin().read_line(&mut user_input);

    let selected_id = user_input.trim().parse::<usize>();

    return selected_id;
}
