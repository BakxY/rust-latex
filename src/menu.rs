use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::num::ParseIntError;

use crate::config;
/**
 * @file menu.rs
 * @brief A lib file to handle all menus
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */
use crate::files;

pub fn display_template_selection() {
    print!("{}[2J", 27 as char);
    println!("-------------------- RUST-LATEX --------------------\n");
    println!("{}\n", files::get_template_location());
    println!("--------------------- TEMPLATES --------------------\n");
    display_all_templates();
    println!("----------------------------------------------------\n");
    println!("  Please select a template:");
    print!("  - ");

    let _ = stdout().flush();
}

fn display_all_templates() {
    let all_templates = files::get_templates();

    for i in 0..all_templates.len() {
        let human_readable =
            config::get_human_readable_name(all_templates.get(i).unwrap().to_string());

        println!("  {}. {}", i + 1, human_readable);
    }

    print!("\n");
}

pub fn select_template() -> Result<u32, ParseIntError> {
    let mut user_input = String::new();

    let _ = stdin().read_line(&mut user_input);

    let selected_id = user_input.trim().parse::<u32>();

    return selected_id;
}