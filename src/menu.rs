/***
 * @file menu.rs
 * @brief A lib file to handle all menus
 * @date 31.03.2025
 * @version v1.0.0
 * @author Severin Sprenger (BakxY)
 */

use crate::config;
use crate::config::ReplaceField;
use crate::files;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::num::ParseIntError;

const TERMINAL_WIDTH: usize = 52;

pub fn display_template_selection(all_templates: Vec<String>) {
    clear_cli();
    print_title("RUST-LATEX".to_string());
    println!("{}\n", files::get_template_location());
    print_title("TEMPLATES".to_string());
    display_all_templates(all_templates);
    print_separator();
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

pub fn move_coursor(x: u32, y: u32) {
    print!("\x1B[{};{}H", x, y);
    let _ = stdout().flush();
}

pub fn clear_cli() {
    print!("{}[2J", 27 as char);
}

pub fn print_title(text: String) {
    let text_width = text.len();

    let chars_to_fill = TERMINAL_WIDTH - text_width - 2;
    let filler_left = chars_to_fill / 2;
    let filler_right = chars_to_fill - filler_left;

    for _i in 0..filler_left {
        print!("-");
    }

    print!(" {} ", text.to_ascii_uppercase());

    for _i in 0..filler_right {
        print!("-");
    }

    println!("\n");
}

pub fn print_separator() {
    for _i in 0..TERMINAL_WIDTH {
        print!("-");
    }
    println!("\n");
}

fn get_field_value() -> String {
    let mut user_input = String::new();

    let _ = stdin().read_line(&mut user_input);

    return user_input.trim().to_string();
}

pub fn display_field_menu(selected_template_readable: String,) {
    clear_cli();

    print_title("RUST-LATEX".to_string());
    println!("  Selected template: {}\n", selected_template_readable);
    print_title("TEMPLATE FIELD GROUPS".to_string());
    println!("  Fill in the following fields!\n");
}

pub fn get_all_field_values(all_groups: Vec<config::FieldGroup>, all_fields: Vec<config::ReplaceField>) -> Vec<ReplaceField> {
    let mut all_fields = all_fields;
    let mut all_fields_filled: Vec<ReplaceField> = Vec::new();

    for current_group in all_groups {
        let (cleaned_field_list, mut current_group_fields) = config::get_group_fields(all_fields, current_group.clone());
        all_fields = cleaned_field_list;

        print_title(current_group);

        for i in 0..current_group_fields.len() {
            let current_field = current_group_fields.get(i).unwrap();

            print!("  {}: ", current_field.readable);
            let _ = stdout().flush();

            let user_set_value = get_field_value();

            current_group_fields[i].value = user_set_value;
        }

        all_fields_filled.append(&mut current_group_fields);
        println!("");
    }

    return all_fields_filled;
}